use std::println;

use rand::prelude::*;

#[derive(Debug)]
pub struct NtruIntPoly {
    pub n: usize,
    pub coeffs: Vec<i16>,
}

pub fn ntruprime_mult_poly(
    a: &NtruIntPoly,
    b: &NtruIntPoly,
    c: &mut NtruIntPoly,
    modulus: u16,
) -> bool {
    let n = a.n;

    if n != b.n {
        return false;
    }

    c.n = n;
    c.coeffs = vec![0; n];

    for k in 0..n {
        let mut ck1 = 0;

        for i in 0..=k {
            ck1 += (a.coeffs[i] as u64) * (b.coeffs[k - i] as u64);
        }

        let mut ck2 = 0;

        for i in (k + 1)..n {
            ck2 += (a.coeffs[i] as u64) * (b.coeffs[k + n - i] as u64);
        }

        let ck = c.coeffs[k] as u64 + ck1 + ck2;

        c.coeffs[k] = (ck % (modulus as u64)) as i16;

        if k < n - 1 {
            let ck = c.coeffs[k + 1] as u64 + ck2;

            c.coeffs[k + 1] = (ck % (modulus as u64)) as i16;
        }
    }

    true
}

fn ntruprime_inv_int(mut a: u16, modulus: u16) -> u16 {
    let mut x: i16 = 0;
    let mut lastx: i16 = 1;
    let mut y: i16 = 1;
    let mut lasty: i16 = 0;
    let mut b: i16 = modulus as i16;

    while b != 0 {
        let quotient = (a as i16) / b;

        let temp = a as i16;
        a = b as u16;
        b = temp % b;

        let temp = x;
        x = lastx - quotient * x;
        lastx = temp;

        let temp = y;
        y = lasty - quotient * y;
        lasty = temp;
    }

    if lastx < 0 {
        lastx += modulus as i16;
    }

    lastx as u16
}

impl NtruIntPoly {
    // Add here random method
    pub fn new(n: usize) -> Self {
        let mut rng = thread_rng();
        let coeffs: Vec<i16> = (0..n)
            .map(|_| {
                let entropy = rng.gen::<u32>();

                (entropy % 3) as i16
            })
            .collect();

        NtruIntPoly { n, coeffs }
    }

    pub fn from_zero(n: usize) -> Self {
        // Zeros a polynomial and sets the number of coefficients
        let coeffs = vec![0i16; n];

        NtruIntPoly { n, coeffs }
    }

    pub fn ntruprime_inv_poly() {}
}

#[test]
fn test_ntru_poly() {
    let mut poly = NtruIntPoly::new(761);

    // dbg!(poly);
}

#[test]
fn test_ntruprime_zero() {
    let poly = NtruIntPoly::from_zero(761);

    // dbg!(poly);
}

#[test]
fn ntruprime_inv_int_test() {
    let a: u16 = 7175;
    let mod0: u16 = 9829;
    let res = ntruprime_inv_int(a, mod0);

    assert!(res == 2885);
}
