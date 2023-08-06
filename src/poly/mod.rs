use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::ops::{Add, Deref, Mul, Sub};

pub struct PolyInt<T> {
    pub coeffs: Vec<T>,
}

impl<T> PolyInt<T>
where
    T: Copy
        + Euclid
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + PartialOrd<T>
        + FromPrimitive,
{
    pub fn empty() -> Self {
        let coeffs = vec![];

        PolyInt { coeffs }
    }

    pub fn from(coeffs: &[T]) -> Self {
        PolyInt {
            coeffs: Vec::from(coeffs),
        }
    }

    pub fn is_small(&self) -> bool {
        self.coeffs
            .deref()
            .iter()
            .all(|&value| value <= T::from_i8(1).unwrap() && value >= T::from_i8(-1).unwrap())
    }

    pub fn mul_int(&mut self, n: T) {
        self.coeffs = self.coeffs.iter_mut().map(|v| *v * n).collect();
    }
}
