use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::ops::{Add, Deref, Mul, Sub};
use std::sync::Arc;

pub struct PolyInt<T> {
    pub coeffs: Arc<Vec<T>>,
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
        let coeffs = Arc::new(vec![]);

        PolyInt { coeffs }
    }

    pub fn from(coeffs: &[T]) -> Self {
        PolyInt {
            coeffs: Arc::new(Vec::from(coeffs)),
        }
    }

    pub fn is_small(&self) -> bool {
        self.coeffs
            .deref()
            .iter()
            .all(|&value| value <= T::from_i8(1).unwrap() && value >= T::from_i8(-1).unwrap())
    }
}
