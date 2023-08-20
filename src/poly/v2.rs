use num::{One, Zero};
use std::ops::{Add, Neg};

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
}

impl<N, const SIZE: usize> PartialEq for PolyInt<N, SIZE>
where
    N: Zero + PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_coeffs() == other.get_coeffs()
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
}
