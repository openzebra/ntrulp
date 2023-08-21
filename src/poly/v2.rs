use crate::poly::traits::TryFrom;
use num::{traits::Euclid, FromPrimitive, One, ToPrimitive, Zero};
use std::ops::{AddAssign, Mul, Neg};

use super::traits::ConversionError;

#[derive(Debug)]
pub struct PolyInt<N: Sized, const SIZE: usize> {
    coeffs: [N; SIZE],
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized + Copy + Default,
{
    pub fn from(coeffs: [N; SIZE]) -> Self {
        Self { coeffs }
    }

    pub fn new() -> Self {
        Self {
            coeffs: [N::default(); SIZE],
        }
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized,
{
    /// Gets the slice of internal data.
    #[inline]
    pub fn get_coeffs(&self) -> &[N; SIZE] {
        &self.coeffs
    }

    // Gets size of coeffs or P of Poly
    #[inline]
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized + One + Zero + PartialOrd<N>,
{
    pub fn equals_zero(&self) -> bool {
        for item in self.coeffs.iter() {
            if *item == N::zero() {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    pub fn equals_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != N::one() {
                return false;
            }
        }

        self.coeffs[0] == N::one()
    }

    pub fn get_poly_degree(&self) -> usize {
        for i in (0..=SIZE - 1).rev() {
            if self.coeffs[i] != N::zero() {
                return i;
            }
        }

        0
    }
}

impl<N, const SIZE: usize> PartialEq for PolyInt<N, SIZE>
where
    N: Zero + PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_coeffs() == other.get_coeffs()
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Copy
        + Sized
        + Zero
        + Default
        + Mul<Output = N>
        + ToPrimitive
        + FromPrimitive
        + std::fmt::Debug,
{
    pub fn mult_int(&mut self, n: N) {
        for i in 0..SIZE {
            self.coeffs[i] = self.coeffs[i] * n;
        }
    }

    pub fn mult_mod(&mut self, factor: N, modulus: u64) -> Result<(), ConversionError> {
        let factor64 = N::to_u64(&factor).ok_or(ConversionError::Overflow)?;

        for i in 0..self.len() {
            let coeff64 = N::to_u64(&self.coeffs[i]).ok_or(ConversionError::Overflow)?;
            let value = (coeff64 * factor64).rem_euclid(modulus);

            self.coeffs[i] = N::from_u64(value).ok_or(ConversionError::Overflow)?;
        }

        Ok(())
    }

    pub fn mult_poly(&self, b: &PolyInt<N, SIZE>, modulus: u64) -> Result<Self, ConversionError> {
        let mut result: PolyInt<N, SIZE> = PolyInt::new();

        for k in 0..SIZE {
            let mut ck1 = 0;

            for i in 0..=k {
                let ai = N::to_u64(&self.coeffs[i]).ok_or(ConversionError::Overflow)?;
                let bk = N::to_u64(&b.coeffs[k - i]).ok_or(ConversionError::Overflow)?;

                ck1 += ai * bk;
            }

            let mut ck2 = 0;

            for i in (k + 1)..SIZE {
                let ai = N::to_u64(&self.coeffs[i]).ok_or(ConversionError::Overflow)?;
                let bki = N::to_u64(&b.coeffs[k + SIZE - i]).ok_or(ConversionError::Overflow)?;

                ck2 += ai * bki;
            }

            let selfk64 = N::to_u64(&result.coeffs[k]).ok_or(ConversionError::Overflow)?;
            let ck = selfk64 + ck1 + ck2;

            result.coeffs[k] = N::from_u64(ck % modulus).ok_or(ConversionError::Overflow)?;

            if k < SIZE - 1 {
                let selfk64 = N::to_u64(&result.coeffs[k + 1]).ok_or(ConversionError::Overflow)?;
                let ck = selfk64 + ck2;

                result.coeffs[k + 1] =
                    N::from_u64(ck % modulus).ok_or(ConversionError::Overflow)?;
            }
        }

        Ok(result)
    }

    pub fn subtract_multiple(
        &mut self,
        b: &PolyInt<N, SIZE>,
        u: u64,
        modulus: u64,
    ) -> Result<(), ConversionError> {
        let n = if b.len() > self.len() {
            b.len()
        } else {
            self.len()
        };

        for i in 0..n {
            let mut ai = N::to_u64(&self.coeffs[i]).ok_or(ConversionError::Overflow)?;
            let bi = N::to_u64(&b.coeffs[i]).ok_or(ConversionError::Overflow)?;
            let dim = u * (modulus - bi);

            ai = ai + dim;

            self.coeffs[i] = N::from_u64(ai % modulus).ok_or(ConversionError::Overflow)?;
        }

        Ok(())
    }

    // Multiplies a polynomial by x^(-1) in (Z/qZ)[x][x^p-x-1] where p=SIZE, q=modulus
    fn div_x(&mut self, modulus: u64) -> Result<(), ConversionError> {
        let a0 = self.coeffs[0];

        self.coeffs.rotate_left(1);
        self.coeffs[SIZE - 1] = a0;

        let zero_self64 = N::to_u64(&self.coeffs[0]).ok_or(ConversionError::Overflow)?;
        let a0 = N::to_u64(&a0).ok_or(ConversionError::Overflow)?;

        self.coeffs[0] =
            N::from_u64((modulus - zero_self64 + a0) % modulus).ok_or(ConversionError::Overflow)?;

        Ok(())
    }

    // Reduces a NtruIntPoly modulo x^p-x-1, where p = Fp.
    fn reduce(&mut self, b: &PolyInt<N, SIZE>, modulus: u64) -> Result<(), ConversionError> {
        let n = SIZE - 1;

        self.coeffs[..n].copy_from_slice(&b.coeffs[..n]);

        let self_zero = N::to_u64(&self.coeffs[0]).ok_or(ConversionError::Overflow)?;
        let self_one = N::to_u64(&self.coeffs[1]).ok_or(ConversionError::Overflow)?;
        let b_n = N::to_u64(&b.coeffs[n]).ok_or(ConversionError::Overflow)?;

        self.coeffs[0] =
            N::from_u64((self_zero + b_n) % modulus).ok_or(ConversionError::Overflow)?;

        self.coeffs[1] =
            N::from_u64((self_one + b_n) % modulus).ok_or(ConversionError::Overflow)?;

        Ok(())
    }
}

#[cfg(test)]
mod test_poly_v2 {
    use crate::math::euclid_inv_num::euclid_num_mod_inverse;

    use super::*;

    #[test]
    fn test_init_from_arr() {
        let a = PolyInt::from([1, 2, 3]);

        assert_eq!(a.get_coeffs(), &[1, 2, 3]);
    }

    #[test]
    fn test_init_zeros() {
        let a: PolyInt<u8, 3> = PolyInt::new();

        assert_eq!(a.len(), 3);
    }

    #[test]
    fn test_is_zeros() {
        let coeffs = [0; 716];
        let mut poly = PolyInt::from(coeffs);

        assert!(poly.equals_zero());

        poly.coeffs[1] = 1;

        assert!(!poly.equals_zero());

        poly.coeffs[1] = -1;

        assert!(!poly.equals_zero());
    }

    #[test]
    fn test_mult_poly_int() {
        let expected_result = [1 * 3, -1 * 3, 0 * 3, -1 * 3, 1 * 3];
        let mut poly = PolyInt::from([1, -1, 0, -1, 1]);

        poly.mult_int(3);

        assert_eq!(poly.get_coeffs(), &expected_result);
    }

    #[test]
    fn test_equals_one() {
        let one_poly = PolyInt::from([1, 1, 1, 1, 1, 1]);

        assert!(one_poly.equals_one());

        let none_one_poly = PolyInt::from([1, 0, 0, 0, -1, 1]);

        assert!(!none_one_poly.equals_one());
    }

    #[test]
    fn test_mult_mod() {
        let mut test_poly: PolyInt<u16, 9> = PolyInt::from([1, 2, 2, 0, 0, 1, 2, 2, 2]);

        test_poly.mult_mod(3845, 9829).unwrap();

        assert!(test_poly.coeffs == [3845, 7690, 7690, 0, 0, 3845, 7690, 7690, 7690]);
    }
    #[test]
    fn test_get_poly_degre() {
        let zero_poly: PolyInt<u8, 740> = PolyInt::new();
        let mut non_zero_poly: PolyInt<u8, 740> = PolyInt::new();

        non_zero_poly.coeffs[730] = 9;

        assert_eq!(zero_poly.get_poly_degree(), 0);
        assert_eq!(non_zero_poly.get_poly_degree(), 730);
    }
    #[test]
    fn test_reduce() {
        let test_poly: PolyInt<u16, 9> = PolyInt::from([1, 2, 2, 0, 0, 1, 2, 2, 2]);
        let mut b: PolyInt<u16, 9> =
            PolyInt::from([7756, 7841, 1764, 7783, 4731, 2717, 1132, 1042, 273]);
        let modulus = 9829;

        b.reduce(&test_poly, modulus).unwrap();

        assert_eq!(b.get_coeffs(), &[3, 4, 2, 0, 0, 1, 2, 2, 273]);
    }

    #[test]
    fn test_div_x() {
        let mut test_poly: PolyInt<u16, 9> =
            PolyInt::from([7756, 7841, 1764, 7783, 4731, 2717, 1132, 1042, 273]);
        let k = 1475;

        for _ in 0..k {
            test_poly.div_x(9829).unwrap();
        }

        assert_eq!(
            test_poly.coeffs,
            [2672, 4340, 2658, 4812, 9587, 6288, 5887, 2572, 6875]
        );
    }

    #[test]
    fn test_subtract_multiple() {
        let modulus = 9829;
        let mut f: PolyInt<u16, 9> = PolyInt::from([756, 741, 0, 78, 470, 7, 0, 0, 273]);
        let g: PolyInt<u16, 9> = PolyInt::from([1, 44, 99, 112, 193, 1235, 908, 285, 9475]);

        let g0_inv = euclid_num_mod_inverse(g.coeffs[0], modulus);
        let u = (f.coeffs[0] * g0_inv) % modulus; // 756;

        f.subtract_multiple(&g, u as u64, modulus as u64).unwrap();

        assert!(f.coeffs == [0, 6793, 3788, 3867, 1997, 102, 1582, 778, 2514]);
    }

    #[test]
    fn test_mult_poly() {
        let modulus = 9829;
        let f: PolyInt<u16, 9> = PolyInt::from([756, 741, 0, 78, 470, 7, 0, 0, 273]);
        let c: PolyInt<u16, 9> = PolyInt::from([4543, 877, 0, 22, 0, 700, 12, 204, 83]);

        let res = f.mult_poly(&c, modulus).unwrap();

        assert_eq!(
            res.get_coeffs(),
            &[5991, 8083, 8262, 8760, 4616, 8326, 4855, 6082, 8069]
        );
    }
}
