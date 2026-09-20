//! Deterministic probe: is `R3::recip` a correct inversion algorithm?
//! Independent reference: classic extended Euclid over GF(3)[x] (pure
//! polynomial ring, no shared code with ntrulp), then reduce mod x^p-x-1.

use ntrulp::params::params::P;
use ntrulp::poly::r3::R3;
use ntrulp::rng::random_small;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

type Poly = Vec<i8>; // GF(3) coefficients in {-1, 0, 1}, index = degree

fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn poly_mod3(a: Poly) -> Poly {
    a.into_iter()
        .map(|c| match c.rem_euclid(3) {
            0 => 0,
            1 => 1,
            _ => -1,
        })
        .collect()
}

fn poly_mul(a: &Poly, b: &Poly) -> Poly {
    let mut out = vec![0i8; a.len() + b.len().saturating_sub(1)];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            out[i + j] += ai * bj;
        }
    }
    poly_mod3(trim(out))
}

fn poly_sub(a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    poly_mod3(
        (0..n)
            .map(|i| a.get(i).copied().unwrap_or(0) - b.get(i).copied().unwrap_or(0))
            .collect::<Vec<_>>(),
    )
}

/// Returns (quotient, remainder) of a / b over GF(3)[x].
fn poly_divmod(a: &Poly, b: &Poly) -> (Poly, Poly) {
    let b = trim(b.clone());
    assert!(!b.is_empty(), "division by zero");
    let mut rem = trim(a.clone());
    let db = b.len() - 1;
    if rem.len() < b.len() {
        return (vec![], rem);
    }
    let mut q = vec![0i8; rem.len() - db];
    let binv = match b[db].rem_euclid(3) {
        1 => 1i8,
        _ => 2i8, // 2 == -1, inverse of -1 in GF(3)
    };
    while trim(rem.clone()).len() >= b.len() {
        let rem_trimmed = trim(rem.clone());
        if rem_trimmed.len() < b.len() {
            break;
        }
        let dq = rem_trimmed.len() - 1 - db;
        let lead = rem_trimmed[rem_trimmed.len() - 1];
        q[dq] = lead * binv;
        let mut shifted = vec![0i8; dq + b.len()];
        shifted[dq..].copy_from_slice(&b.iter().map(|c| lead * binv * c).collect::<Vec<_>>());
        rem = poly_sub(&rem_trimmed, &trim(shifted));
        rem = trim(poly_mod3(rem));
    }
    (trim(q), trim(rem))
}

/// True inverse of `c` in GF(3)[x]/(x^p - x - 1), or None if non-invertible.
fn ref_r3_inverse(c: &[i8; P]) -> Option<[i8; P]> {
    let modulus: Poly = {
        let mut m = vec![0i8; P + 1];
        m[0] = -1; // x^p - x - 1
        m[1] = -1;
        m[P] = 1;
        m
    };
    let cc: Poly = c.iter().copied().collect();

    // Extended Euclid: r0 = modulus, r1 = c; s0 = 0, s1 = 1.
    let mut r0 = trim(modulus);
    let mut r1 = trim(cc.clone());
    let mut s0: Poly = vec![];
    let mut s1: Poly = vec![1];

    while !r1.is_empty() {
        let (q, r) = poly_divmod(&r0, &r1);
        r0 = r1;
        r1 = r;
        let qs = poly_mul(&q, &s1);
        let s = poly_sub(&s0, &qs);
        s0 = s1;
        s1 = s;
    }

    // gcd = r0 must be a nonzero constant for invertibility.
    if r0.len() != 1 || r0[0] == 0 {
        return None;
    }
    let inv_unit = match r0[0].rem_euclid(3) {
        1 => 1i8,
        _ => -1i8,
    };
    let full = poly_mul(&poly_mod3(s0), &vec![inv_unit]);

    // Reduce modulo x^p - x - 1: x^k = x^{k-p+1} + x^{k-p} for k >= p.
    let mut reduced = vec![0i64; full.len().max(P + 1)];
    for (i, &v) in full.iter().enumerate() {
        reduced[i] += v as i64;
    }
    for k in (P..reduced.len()).rev() {
        let v = reduced[k];
        reduced[k - (P - 1)] += v;
        reduced[k - P] += v;
        reduced[k] = 0;
    }
    let mut out = [0i8; P];
    for (o, v) in out.iter_mut().zip(reduced.iter()) {
        *o = match v.rem_euclid(3) {
            0 => 0,
            1 => 1,
            _ => -1,
        };
    }
    Some(out)
}

fn mul_r3(a: &[i8; P], b: &[i8; P]) -> [i8; P] {
    let mut acc = vec![0i64; 2 * P - 1];
    for i in 0..P {
        for j in 0..P {
            acc[i + j] += a[i] as i64 * b[j] as i64;
        }
    }
    for k in (P..2 * P - 1).rev() {
        acc[k - P] += acc[k];
        acc[k - P + 1] += acc[k];
    }
    let mut out = [0i8; P];
    for (o, v) in out.iter_mut().zip(acc.iter()) {
        *o = match v.rem_euclid(3) {
            0 => 0,
            1 => 1,
            _ => -1,
        };
    }
    out
}

#[test]
fn r3_recip_classification() {
    let mut rng = ChaCha20Rng::seed_from_u64(0xC0FFEE);

    let (mut n_inv, mut n_recip_ok, mut n_ident, mut bad) =
        (0usize, 0usize, 0usize, 0usize);

    for trial in 0..300 {
        let mut f = [0i8; P];
        random_small(&mut rng).iter().enumerate().for_each(|(i, &v)| f[i] = v);

        // sanity of probe itself every 50 trials
        if trial % 50 == 0 {
            if let Some(inv) = ref_r3_inverse(&f) {
                let prod = mul_r3(&f, &inv);
                assert_eq!(prod[0], 1, "reference self-check failed");
            }
        }

        let invertible = ref_r3_inverse(&f).is_some();
        if invertible {
            n_inv += 1;
        }

        match R3::from(f).recip() {
            Ok(g) => {
                n_recip_ok += 1;
                let prod = mul_r3(&f, &g.coeffs);
                if prod[0] == 1 {
                    n_ident += 1;
                } else if invertible {
                    bad += 1;
                    println!("BAD: invertible input, recip() returned wrong inverse (trial {trial})");
                } else {
                    bad += 1;
                    println!(
                        "BAD: NON-invertible input accepted as Ok (trial {trial}), f*g0 = {}",
                        prod[0]
                    );
                }
            }
            Err(_) => {
                if invertible {
                    bad += 1;
                    println!("BAD: invertible input, recip() returned Err (trial {trial})");
                }
            }
        }
    }

    println!(
        "trials=300 invertible={n_inv} recip_ok={n_recip_ok} identity_ok={n_ident} bad={bad}"
    );
    assert_eq!(bad, 0, "recip() disagrees with GF(3) reference");
}
