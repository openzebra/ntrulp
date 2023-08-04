use crate::math::finite_field::GF;
use num::traits::real::Real;
use num::traits::Euclid;
use num::FromPrimitive;
use rand::prelude::*;
use std::cmp::PartialOrd;
use std::ops::{Add, Mul, Sub};
use std::sync::Arc;

#[derive(Debug, PartialEq)]
struct PolyInt<T> {
    pub coeffs: Arc<Vec<T>>,
    r: GF<T>,
}

impl<T> PolyInt<T>
where
    T: Copy
        + Euclid
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + PartialOrd<T>
        + FromPrimitive
        + Real,
{
    pub fn empty() -> Self {
        let r: GF<T> = GF::new(T::from_u8(1).unwrap(), T::from_u8(3).unwrap());
        let coeffs = Arc::new(vec![]);

        PolyInt { r, coeffs }
    }

    pub fn is_small(&self, r: &[T]) -> bool {
        r.iter()
            .all(|&value| value.abs() <= T::from_u8(1).unwrap() && self.r.has(value))
    }
}
