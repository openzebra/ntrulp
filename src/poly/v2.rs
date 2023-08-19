use num::{Complex, One, Zero};
use std::ops::Add;
use std::{ops::Deref, sync::Arc};

#[derive(Debug)]
pub struct PolyInt<N: Sized, const SIZE: usize> {
    coeffs: Arc<[N; SIZE]>,
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized + Copy + Zero,
{
    pub fn from(coeffs: [N; SIZE]) -> Self {
        Self {
            coeffs: Arc::new(coeffs),
        }
    }

    pub fn new() -> Self {
        Self {
            coeffs: Arc::new([N::zero(); SIZE]),
        }
    }
}

impl<N: Sized, const SIZE: usize> PolyInt<N, SIZE> {
    /// Gets the slice of internal data.
    #[inline]
    pub fn get_coeffs(&self) -> &[N; SIZE] {
        self.coeffs.deref()
    }

    // Gets size of coeffs or P of Poly
    #[inline]
    pub fn len(&self) -> usize {
        self.coeffs.deref().len()
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

impl<N, const SIZE: usize> Add<N> for PolyInt<N, SIZE>
where
    N: Zero + PartialEq + Copy,
{
    type Output = Self;

    fn add(self, other: PolyInt<N, SIZE>) -> Self::Output {
        for (c1, &c2) in self.get_coeffs().iter_mut().zip(other.get_coeffs().iter()) {
            // *c1 += c2;
        }

        Self {
            ...self
        }
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
}
