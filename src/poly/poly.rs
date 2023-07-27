use std::{println, todo, vec};

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

            c.coeffs[k + 1] = (ck.rem_euclid(modulus as u64)) as i16;
        }
    }

    true
}

fn ntruprime_inv_int(mut a: i16, modulus: u16) -> u16 {
    let mut x: i16 = 0;
    let mut lastx: i16 = 1;
    let mut y: i16 = 1;
    let mut lasty: i16 = 0;
    let mut b: i16 = modulus as i16;

    while b != 0 {
        let quotient = a / b;

        let temp = a as i16;
        a = b;
        b = temp.rem_euclid(b);

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

                entropy.rem_euclid(3) as i16
            })
            .collect();

        NtruIntPoly { n, coeffs }
    }

    pub fn from_zero(n: usize) -> Self {
        // Zeros a polynomial and sets the number of coefficients
        let coeffs = vec![0i16; n];

        NtruIntPoly { n, coeffs }
    }

    pub fn equals_zero(&self) -> bool {
        let sum: i16 = self.coeffs.iter().sum();

        sum == 0
    }

    pub fn get_poly_degree(&self) -> usize {
        for i in (0..=self.n - 1).rev() {
            if self.coeffs[i] != 0 {
                return i;
            }
        }

        0
    }

    pub fn mult_mod(&mut self, factor: u64, modulus: u64) {
        self.coeffs.iter_mut().for_each(|coeff| {
            *coeff = ((*coeff as u64 * factor) % modulus) as i16;
        });
    }

    // Reduces a NtruIntPoly modulo x^N-x-1, where N = a->N.
    pub fn reduce(&self, b: &mut NtruIntPoly, modulus: u64) {
        let n = self.n - 1;

        b.coeffs[..n].copy_from_slice(&self.coeffs[..n]);
        b.coeffs[0] = (b.coeffs[0] as u64)
            .wrapping_add(self.coeffs[n] as u64)
            .rem_euclid(modulus) as i16;
        b.coeffs[1] = (b.coeffs[1] as u64)
            .wrapping_add(self.coeffs[n] as u64)
            .rem_euclid(modulus) as i16;
        b.coeffs.truncate(n);
        b.n = n;
    }

    // Multiplies a polynomial by x^(-1) in (Z/qZ)[x][x^p-x-1] where p=a->N, q=modulus
    pub fn div_x(&mut self, modulus: u64) {
        let n = self.n;
        let a0 = self.coeffs[0];

        self.coeffs.rotate_left(1);
        self.coeffs[n - 1] = a0;

        self.coeffs[0] = (self.coeffs[0] as u64)
            .wrapping_sub(a0 as u64)
            .wrapping_add(modulus)
            .rem_euclid(modulus) as i16
    }

    pub fn get_inv_poly(&self, modulus: u16) {
        let n = self.n;
        let im = modulus as i16;
        let mut inv = NtruIntPoly::from_zero(n);
        let mut k = 0;
        let mut b = NtruIntPoly::from_zero(n + 1);

        b.coeffs[0] = 1;

        let mut c = NtruIntPoly::from_zero(n + 1);

        // f = a
        let mut f = NtruIntPoly::from_zero(n + 1);

        f.coeffs[..n].copy_from_slice(&self.coeffs[..n]);
        f.coeffs[n] = 0;

        // g = x^p - x - 1
        let mut g = NtruIntPoly::from_zero(n + 1);

        g.coeffs[0] = im - 1;
        g.coeffs[1] = im - 1;
        g.coeffs[n] = 1;

        loop {
            while f.coeffs[0] == 0 {
                // f(x) = f(x) / x
                for i in 1..=n {
                    f.coeffs[i - 1] = f.coeffs[i];
                }

                f.coeffs[n] = 0;

                // c(x) = c(x) * x
                for i in (1..n).rev() {
                    c.coeffs[i] = c.coeffs[i - 1];
                }

                c.coeffs[0] = 0;
                k += 1;

                if f.equals_zero() {
                    // return None
                    return ();
                }
            }

            if f.get_poly_degree() == 0 {
                let f0_inv = ntruprime_inv_int(f.coeffs[0], modulus);

                // b = b * f[0]^(-1)
                b.mult_mod(f0_inv as u64, modulus as u64);
                b.reduce(&mut inv, modulus as u64);

                // b = b * x^(-k)
                for _ in 0..k {
                    // ntruprime_div_x(inv, modulus);
                }
            }
            if f.get_poly_degree() < g.get_poly_degree() {}
            //
        }
    }
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
    let a: i16 = 7175;
    let mod0: u16 = 9829;
    let res = ntruprime_inv_int(a, mod0);

    assert!(res == 2885);
}

#[test]
fn test_from_zero() {
    let non_zero_poly = NtruIntPoly::new(761);
    let zero_poly = NtruIntPoly::from_zero(761);

    assert!(!non_zero_poly.equals_zero());
    assert!(zero_poly.equals_zero());
}

#[test]
fn test_get_poly_degre() {
    let zero_poly = NtruIntPoly::from_zero(740);
    let mut non_zero_poly = NtruIntPoly::from_zero(740);

    non_zero_poly.coeffs[non_zero_poly.n - 10] = 9;

    assert!(zero_poly.get_poly_degree() == 0);
    assert!(non_zero_poly.get_poly_degree() == 730);
}

#[test]
fn test_mult_mod() {
    let mut test_poly = NtruIntPoly::from_zero(9);

    test_poly.coeffs = vec![1, 2, 2, 0, 0, 1, 2, 2, 2];
    test_poly.n = test_poly.coeffs.len();

    test_poly.mult_mod(3845, 9829);

    assert!(test_poly.coeffs == [3845, 7690, 7690, 0, 0, 3845, 7690, 7690, 7690]);
}

#[test]
fn test_reduce() {
    let mut test_poly = NtruIntPoly::from_zero(9);
    let mut b = NtruIntPoly::from_zero(9);
    let modulus = 9829;

    test_poly.coeffs = vec![1, 2, 2, 0, 0, 1, 2, 2, 2];
    b.coeffs = vec![7756, 7841, 1764, 7783, 4731, 2717, 1132, 1042, 273];

    test_poly.n = test_poly.coeffs.len();

    test_poly.reduce(&mut b, modulus);

    assert!(b.coeffs == [3, 4, 2, 0, 0, 1, 2, 2]);
}

#[test]
fn test_div_x() {
    let mut test_poly = NtruIntPoly::from_zero(9);
    let k = 1475;

    test_poly.coeffs = vec![7756, 7841, 1764, 7783, 4731, 2717, 1132, 1042, 273];
    test_poly.n = test_poly.coeffs.len();

    for _ in 0..k {
        test_poly.div_x(9829);
    }

    assert!(test_poly.coeffs == [5018, 6408, 7987, 4832, 1047, 387, 1857, 4668, 2577]);
}
