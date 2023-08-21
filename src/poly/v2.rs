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
    N: Sized + One + Zero + PartialOrd<N> + Neg<Output = N>,
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

    pub fn is_small(&self) -> bool {
        self.coeffs
            .iter()
            .all(|value| *value <= N::one() && *value >= -N::one())
    }

    pub fn equals_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != N::one() {
                return false;
            }
        }

        self.coeffs[0] == N::one()
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
    N: Copy + Sized + Mul<Output = N> + ToPrimitive + FromPrimitive,
{
    pub fn mult_int(&mut self, n: N) {
        for i in 0..SIZE {
            self.coeffs[i] = self.coeffs[i] * n;
        }
    }

    pub fn mult_mod(&mut self, factor: N, modulus: N) -> Result<(), ConversionError> {
        let factor64 = N::to_u64(&factor).ok_or(ConversionError::Overflow)?;
        let modulus64 = N::to_u64(&modulus).ok_or(ConversionError::Overflow)?;

        for i in 0..self.len() {
            let coeff64 = N::to_u64(&self.coeffs[i]).ok_or(ConversionError::Overflow)?;
            let value = (coeff64 * factor64).rem_euclid(modulus64);

            self.coeffs[i] = N::from_u64(value).ok_or(ConversionError::Overflow)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_poly_v2 {
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
    fn test_is_small() {
        let coefficients_big = [0, -1, -2, 2];
        let coefficients_small = [0, -1, -1, 1];

        let poly = PolyInt::from(coefficients_big);

        assert!(!poly.is_small());

        let poly = PolyInt::from(coefficients_small);
        assert!(poly.is_small());
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
}
