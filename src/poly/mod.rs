use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::ops::{AddAssign, Div, Mul, SubAssign};

pub struct PolyInt<T> {
    pub coeffs: Vec<T>,
}

impl<T> PolyInt<T>
where
    T: Copy
        + Euclid
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
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

    pub fn sub_poly(&mut self, poly: &[T]) {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 -= c2;
        }
    }

    pub fn mult_int(&mut self, n: T) {
        self.coeffs = self.coeffs.iter_mut().map(|v| *v * n).collect();
    }

    pub fn mult_poly(&mut self, poly: &[T]) {
        let len_result = self.coeffs.len() + poly.len() - 1;
        let mut result: Vec<T> = Vec::with_capacity(len_result);

        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in poly.iter().enumerate() {
                result[i + j] += c1 * c2;
            }
        }

        self.coeffs.clear();
        self.coeffs.extend_from_slice(&result);
    }

    pub fn add_poly(&mut self, poly: &[T]) {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 += c2;
        }
    }

    pub fn div_mod_poly(&mut self, poly: &[T]) {
        if self.coeffs.len() > poly.len() {
            return;
        }

        let mut p1_clone = poly.to_vec();
        let mut p2_clone = self.coeffs.to_vec();

        while p1_clone.len() >= p2_clone.len() {
            let degree_diff = p1_clone.len() - p2_clone.len();
            let coeff_ratio = *p1_clone.last().unwrap() / *p2_clone.last().unwrap();

            for i in 0..p2_clone.len() {
                if let Some(coeff_p1) = p1_clone.get_mut(degree_diff + i) {
                    *coeff_p1 -= coeff_ratio * p2_clone[i];
                }
            }

            if let Some(last_coeff) = p1_clone.last_mut() {
                *last_coeff = T::from_u8(0).unwrap();
            }

            p2_clone.pop();
        }

        self.coeffs.clear();
        self.coeffs.extend_from_slice(&p2_clone);
    }
}
