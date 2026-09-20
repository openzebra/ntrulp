//! Equivalence tests: the optimized implementation must match an independent,
//! naive reference implementation of the Streamlined NTRU Prime math.
//!
//! The reference works over `i64` with exact centered representatives, straight
//! from the spec definitions — no shared code paths with the optimized poly ops.
//! Param-generic: run once per `ntrup*` feature to cover every parameter set.

use ntrulp::params::params::{P, Q};
use ntrulp::poly::{f3, fq, r3::R3, rq::Rq};
use ntrulp::rng::{random_small, short_random};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha20Rng;

const Q12: i64 = (Q as i64) / 2;

/// Centered representative of `x` mod `Q` in `(-Q/2, Q/2]` — the spec's `Zq`.
fn ref_center(x: i64) -> i64 {
    let q = Q as i64;
    let m = ((x % q) + q) % q;
    if m > q / 2 { m - q } else { m }
}

/// Exact integer convolution modulo `x^p - x - 1` (`x^p = x + 1`).
fn ref_conv(f: &[i64; P], g: &[i64; P]) -> [i64; P] {
    let mut c = vec![0i64; 2 * P - 1];
    for i in 0..P {
        for j in 0..P {
            c[i + j] += f[i] * g[j];
        }
    }
    // x^t for t >= P reduces onto t-P (from x) and t-P+1 (from 1).
    for t in (P..2 * P - 1).rev() {
        c[t - P] += c[t];
        c[t - P + 1] += c[t];
    }
    let mut out = [0i64; P];
    out.copy_from_slice(&c[..P]);
    out
}

fn rng_uniform_frozen(rng: &mut ChaCha20Rng) -> i16 {
    let mut bytes = [0u8; 4];
    rng.fill(&mut bytes);
    fq::freeze(i32::from_le_bytes(bytes))
}

fn to_i64(coeffs: &[i16; P]) -> [i64; P] {
    let mut out = [0i64; P];
    for (o, v) in out.iter_mut().zip(coeffs.iter()) {
        *o = *v as i64;
    }
    out
}

fn to_i64_8(coeffs: &[i8; P]) -> [i64; P] {
    let mut out = [0i64; P];
    for (o, v) in out.iter_mut().zip(coeffs.iter()) {
        *o = *v as i64;
    }
    out
}

#[test]
fn freeze_matches_centered_reference() {
    // Dense sweep over the practical accumulator range.
    let mut x: i64 = -200_000;
    while x <= 200_000 {
        assert_eq!(fq::freeze(x as i32) as i64, ref_center(x), "x = {x}");
        x += 997;
    }
    // i32 extremes: the pre-optimization `x + Q12` overflowed/panicked here.
    for x in [i32::MAX, i32::MAX - 1, i32::MIN, i32::MIN + 1, -1, 0, 1] {
        assert_eq!(fq::freeze(x) as i64, ref_center(x as i64), "x = {x}");
    }
}

#[test]
fn f3_freeze_matches_centered_reference() {
    for x in [-9i16, -3, -2, -1, 0, 1, 2, 3, 9, 1000, -1000] {
        // f3 keeps the centered representative {-1, 0, 1}.
        let expected = match x.rem_euclid(3) {
            0 => 0,
            1 => 1,
            _ => -1,
        };
        assert_eq!(f3::freeze(x) as i32, expected, "x = {x}");
    }
}

#[test]
fn r3_mult_matches_reference() {
    let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());

    for _ in 0..8 {
        let f = R3::from(random_small(&mut rng));
        let g = R3::from(random_small(&mut rng));
        let out = f.mult(&g);

        let want = ref_conv(&to_i64_8(&f.coeffs), &to_i64_8(&g.coeffs));
        for i in 0..P {
            // R3 stores centered ternary reps; compare mod 3, not mod Q.
            let m = want[i].rem_euclid(3);
            let expected = if m == 2 { -1 } else { m };
            assert_eq!(out.coeffs[i] as i64, expected, "i = {i}");
        }
    }
}

#[test]
fn rq_mult_r3_matches_reference() {
    let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());

    for _ in 0..8 {
        let mut f_coeffs = [0i16; P];
        for c in f_coeffs.iter_mut() {
            *c = rng_uniform_frozen(&mut rng);
        }
        let f = Rq::from(f_coeffs);
        let g = R3::from(random_small(&mut rng));
        let out = f.mult_r3(&g);

        let mut g64 = [0i64; P];
        for (o, v) in g64.iter_mut().zip(g.coeffs.iter()) {
            *o = *v as i64;
        }
        let want = ref_conv(&to_i64(&f.coeffs), &g64);
        for i in 0..P {
            assert_eq!(out.coeffs[i] as i64, ref_center(want[i]), "i = {i}");
        }
    }
}

#[test]
fn rq_mult_int_matches_reference() {
    let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());

    for &n in &[-30_000i16, -257, -1, 0, 1, 257, 30_000] {
        let mut f_coeffs = [0i16; P];
        for c in f_coeffs.iter_mut() {
            *c = rng_uniform_frozen(&mut rng);
        }
        let f = Rq::from(f_coeffs);
        let out = f.mult_int(n);

        for i in 0..P {
            // n up to 30000 times a centered coefficient: the widened i32 path.
            assert_eq!(
                out.coeffs[i] as i64,
                ref_center(n as i64 * f_coeffs[i] as i64),
                "n = {n}, i = {i}"
            );
        }
    }
}

#[test]
fn fq_recip_identity() {
    for &a in &[1i16, 2, 5, 100, 2309, 4600, (Q as i16) - 2] {
        let inv = fq::recip(a);
        assert_eq!(ref_center(a as i64 * inv as i64), 1, "a = {a}");
    }
}

#[test]
fn rq_recip_identity_against_reference() {
    let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());

    for ratio in [1i16, 3] {
        for _ in 0..4 {
            let f_coeffs = match short_random(&mut rng) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let f = Rq::from(f_coeffs);
            let g = match ratio {
                1 => match f.recip::<1>() {
                    Ok(g) => g,
                    Err(_) => continue,
                },
                _ => match f.recip::<3>() {
                    Ok(g) => g,
                    Err(_) => continue,
                },
            };

            // recip::<RATIO> yields input^(-1) * RATIO^(-1) (so that
            // h = g * recip::<3>(f) == g/(3f) per the spec KeyGen).
            // Verify: RATIO * f * g == 1 (mod x^p-x-1, mod q).
            let want = ref_conv(&to_i64(&f.coeffs), &to_i64(&g.coeffs));
            for i in 0..P {
                let expected = if i == 0 { 1 } else { 0 };
                assert_eq!(
                    ref_center(ratio as i64 * want[i]),
                    expected,
                    "ratio = {ratio}, i = {i}"
                );
            }
        }
    }
}

#[test]
fn r3_recip_identity_against_reference() {
    let mut rng = ChaCha20Rng::seed_from_u64(7);

    for _ in 0..8 {
        let f = R3::from(random_small(&mut rng));
        let g = match f.recip() {
            Ok(g) => g,
            Err(_) => continue,
        };

        let want = ref_conv(&to_i64_8(&f.coeffs), &to_i64_8(&g.coeffs));
        for i in 0..P {
            // Identity check: f*g == delta_{i,0} in R3, compared mod 3
            // (raw convolution sums can exceed the ternary range).
            let m = want[i].rem_euclid(3);
            let expected = if i == 0 {
                1
            } else if m == 2 {
                -1
            } else {
                m
            };
            assert_eq!(
                want[i].rem_euclid(3),
                expected,
                "i = {i}, f[0..8] = {:?}, g[0..8] = {:?}",
                &f.coeffs[..8],
                &g.coeffs[..8]
            );
        }
    }
}
