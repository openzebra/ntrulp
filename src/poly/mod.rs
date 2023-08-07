use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::ops::{AddAssign, Mul, SubAssign};

pub struct PolyInt<T> {
    pub coeffs: Vec<T>,
}

impl<T> PolyInt<T>
where
    T: Copy + Euclid + Mul<Output = T> + AddAssign + SubAssign + PartialOrd<T> + FromPrimitive,
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
            .iter()
            .all(|&value| value <= T::from_i8(1).unwrap() && value >= T::from_i8(-1).unwrap())
    }

    pub fn mod_poly(&mut self, modulus: T) {
        self.coeffs = self
            .coeffs
            .iter_mut()
            .map(|coeff| coeff.rem_euclid(&modulus))
            .collect();
    }

    pub fn sub_poly(&mut self, p2: &[T]) {
        for (c1, &c2) in self.coeffs.iter_mut().zip(p2.iter()) {
            *c1 -= c2;
        }
    }

    pub fn mult_int(&mut self, n: T) {
        self.coeffs = self.coeffs.iter_mut().map(|v| *v * n).collect();
    }

    pub fn mult_poly(&mut self, p2: &[T]) {
        let len_result = self.coeffs.len() + p2.len() - 1;
        let mut result: Vec<T> = Vec::with_capacity(len_result);

        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in p2.iter().enumerate() {
                result[i + j] += c1 * c2;
            }
        }

        self.coeffs.clear();
        self.coeffs.extend_from_slice(&result);
    }

    pub fn add_poly(&mut self, p2: &[T]) {
        for (c1, &c2) in self.coeffs.iter_mut().zip(p2.iter()) {
            *c1 += c2;
        }
    }
}
