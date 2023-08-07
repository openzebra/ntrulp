use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::ops::{AddAssign, Div, Mul, SubAssign};

#[derive(Clone)]
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

    // Modifies a polynomial by taking each coefficient modulo the given modulus.
    pub fn mod_poly(&mut self, modulus: T) -> Self {
        self.coeffs = self
            .coeffs
            .iter_mut()
            .map(|coeff| coeff.rem_euclid(&modulus))
            .collect();

        self.to_owned()
    }

    // Subtracts one polynomial from another coefficient-wise.
    pub fn sub_poly(&mut self, poly: &[T]) -> Self {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 -= c2;
        }

        self.to_owned()
    }

    pub fn mult_int(&mut self, n: T) -> Self {
        self.coeffs = self.coeffs.iter_mut().map(|v| *v * n).collect();

        self.to_owned()
    }

    // Multiplies two polynomials using convolution of coefficients.
    pub fn mult_poly(&mut self, poly: &[T]) -> Self {
        let len_result = self.coeffs.len() + poly.len() - 1;
        let mut result: Vec<T> = Vec::with_capacity(len_result);

        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in poly.iter().enumerate() {
                result[i + j] += c1 * c2;
            }
        }

        self.coeffs.clear();
        self.coeffs.extend_from_slice(&result);

        self.to_owned()
    }

    // Adds polynomials coefficient-wise.
    pub fn add_poly(&mut self, poly: &[T]) -> Self {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 += c2;
        }

        self.to_owned()
    }

    // Performs polynomial division with remainder.
    pub fn div_mod_poly(&mut self, poly: &[T]) -> Self {
        if self.coeffs.len() > poly.len() {
            return self.to_owned();
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

        self.to_owned()
    }

    // create factor rint for poly x^p - x - 1
    pub fn create_factor_ring(&self, x: &[T], modulus: T) -> PolyInt<T> {
        let x_deg1: PolyInt<T> = PolyInt::from(&x);
        let mut x_deg_p: PolyInt<T> = PolyInt::from(&[T::from_u8(1).unwrap()]);
        let modulus_poly = self.clone().mod_poly(modulus);

        for &coeff in &self.coeffs {
            x_deg_p = x_deg_p.mult_poly(&x_deg1.coeffs);
            x_deg_p = x_deg_p.mod_poly(modulus);

            let mut one_times_coeff_deg = PolyInt::from(&[coeff]);

            one_times_coeff_deg.mult_poly(&x_deg1.coeffs);
            one_times_coeff_deg.mod_poly(modulus);

            x_deg_p.add_poly(&one_times_coeff_deg.coeffs);
            x_deg_p.mod_poly(modulus);
        }

        x_deg_p.div_mod_poly(&modulus_poly.coeffs);

        x_deg_p
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_init_empty_poly() {
        let empty: PolyInt<u8> = PolyInt::empty();

        assert!(empty.coeffs.len() == 0);
    }

    #[test]
    fn test_init_from_coeffs() {
        let coefficients = [0, -1, -2, 2];
        let test_poly = PolyInt::from(&coefficients);

        assert!(test_poly.coeffs == coefficients);
    }

    #[test]
    fn test_is_small() {
        let coefficients_big = [0, -1, -2, 2];
        let coefficients_small = [0, -1, -1, 1];

        let poly = PolyInt::from(&coefficients_big);

        assert!(!poly.is_small());

        let poly = PolyInt::from(&coefficients_small);
        assert!(poly.is_small());
    }

    #[test]
    fn test_create_factor_ring() {
        let coefficients = [
            -1, 1, 0, 1, -1, 0, 0, 1, 1, -1, 0, 1, -1, 1, -1, -1, 0, -1, 0, -1, -1, -1, 0, 0, 0,
            -1, -1, -1, 1, 0, 0, 0, 1, 0, -1, 0, 1, 0, -1, -1, 0, -1, 0, 1, -1, 1, 1, -1, 1, 1, -1,
            0, -1, 0, 1, -1, -1, 1, 1, 0, 1, -1, 0, 1, -1, 1, -1, 0, 1, -1, 0, 0, 0, 1, -1, -1, 1,
            1, 0, 0, -1, 1, -1, 0, 0, 0, 1, -1, 0, -1, -1, 0, -1, -1, 1, -1, -1, 0, 0, -1, 1, 1, 0,
            -1, -1, 0, -1, 1, -1, -1, 0, 1, 1, 1, 1, -1, 0, 0, -1, -1, 0, 0, 1, 1, -1, -1, 0, 0, 0,
            0, 1, 0, 1, 1, -1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, 0, 1, -1, -1, -1, 0, 0, 0,
            -1, -1, 1, 0, 1, -1, 0, 0, 1, 0, 0, 0, -1, 0, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, 0,
            -1, 1, 1, -1, 0, -1, 1, 0, 0, -1, 0, -1, 1, -1, 1, 0, -1, 1, -1, 1, -1, -1, -1, 1, 0,
            0, 0, -1, -1, -1, 1, 0, 0, 0, 1, 1, -1, -1, -1, 1, -1, 1, 0, 1, -1, -1, 0, 1, 1, 1, 0,
            1, 1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1, 0, -1, 1, -1, 1, 0, 0,
            -1, -1, -1, 1, -1, -1, -1, 1, 1, 0, 0, -1, 0, -1, 1, 0, 1, -1, 1, -1, 1, -1, 0, 1, 0,
            -1, 1, -1, 0, -1, 1, -1, 0, 0, 0, -1, 1, 1, 0, -1, 1, 0, -1, 1, 0, -1, 1, 0, -1, 0, -1,
            -1, -1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, -1, 0, 0, 1, 0, 0, -1, -1, 0, 1, 0, -1, 0, 1,
            1, 1, 0, 1, -1, 0, 0, -1, 0, 1, 1, -1, 1, 0, -1, -1, -1, 0, 0, -1, -1, -1, -1, 0, -1,
            0, 0, -1, -1, -1, -1, -1, -1, 0, 0, 0, -1, 1, 0, -1, 0, 1, -1, -1, 0, -1, -1, 0, 0, 0,
            0, 1, 0, -1, 1, 1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, -1, 1, 0, 1, 1, 1, 0, -1, 1, 1, -1,
            1, -1, -1, 0, -1, 1, 1, -1, -1, 1, -1, -1, 1, 1, 1, -1, -1, 1, 0, 1, -1, -1, -1, 0, 1,
            1, 1, -1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, -1, 0, 0, 1, 0, 1, 0, 1, -1, -1, -1, -1, 1, 1,
            0, 1, 0, -1, -1, -1, 0, -1, 0, 1, 0, -1, 0, -1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, -1, 1,
            -1, -1, 0, -1, 0, 0, -1, 0, 1, -1, 0, 1, 1, -1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, -1, -1,
            0, 0, 0, -1, 1, 1, -1, -1, -1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, -1, 1, 0, 1, -1, 0, 0, 1,
            0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, -1, -1, 1, 1, 1, -1, -1, 0, -1, 0, -1, -1, 0, -1,
            0, 0, 0, 0, -1, 1, 1, -1, 1, 1, 0, -1, -1, 0, -1, -1, -1, 0, -1, 1, 0, -1, 0, 0, 1, 0,
            -1, 1, -1, 0, -1, -1, 1, -1, 1, -1, -1, -1, -1, -1, 1, -1, 0, -1, -1, 0, -1, -1, 1, 0,
            0, -1, -1, 0, -1, 0, -1, -1, -1, 0, 1, 1, -1, 1, 0, -1, 1, 1, -1, 0, -1, 0, 1, 1, -1,
            0, 0, 1, 0, -1, 0, 0, 0, 0, 0, -1, 1, -1, 0, 0, 1, 0, 0, 0, -1, 1, 0, -1, 0, 1, -1, 1,
            1, 0, -1, 0, 0, -1, 0, -1, -1, -1, 1, -1, -1, -1, 0, 0, -1, 0, 0, 1, 1, -1, 0, -1, -1,
            1, 0, -1, 1, 0, 1, -1, 1, 1, 0, -1, -1, -1, -1, 1, -1, -1, 1, 1, 0, -1, -1, 0, 0, -1,
            -1, -1, 0, 0, 1, 1, 0, 1, 1, 1, 1, -1, 0, 0, 1, 1, -1, 0, 0, 1, 1, -1, -1, -1, -1, 1,
            -1, 1, 1, 1, -1, 1, -1, 1, -1, 0, 0, -1, 0, -1, 0, 1,
        ];
        let poly: PolyInt<i16> = PolyInt::from(&coefficients);
        let x: Vec<i16> = vec![0, 1];
        let modulus: i16 = 4591;
        let fq_ring = poly.create_factor_ring(&x, modulus);

        dbg!(fq_ring.coeffs);
    }
}
