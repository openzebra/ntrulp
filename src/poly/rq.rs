use core::ops::{Index, IndexMut};

use crate::{
    encode::rq,
    params::{params::P, params::RQ_BYTES},
};

use super::{error::PolyErrors, f3, r3::R3};
use crate::{
    math::nums::{i16_negative_mask, i16_nonzero_mask},
    poly::fq,
};

#[derive(Debug, Clone)]
pub struct Rq {
    pub coeffs: [i16; P],
}

impl Default for Rq {
    fn default() -> Self {
        Self::new()
    }
}

impl Rq {
    pub fn new() -> Self {
        Self { coeffs: [0i16; P] }
    }

    pub fn from(coeffs: [i16; P]) -> Self {
        Self { coeffs }
    }

    pub fn eq_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != 0 {
                return false;
            }
        }

        self.coeffs[0] == 1
    }

    pub fn eq_zero(&self) -> bool {
        for c in self.coeffs {
            if c != 0 {
                return false;
            }
        }

        true
    }

    // h = f*g in the ring Rq
    pub fn mult_r3(&self, gq: &R3) -> Rq {
        let mut out = [0i16; P];
        let f = self.coeffs;
        let g = gq.coeffs;
        let mut fg = [0i16; P + P - 1];

        let quotient = |r: i16, f: i16, g: i8| {
            let value = r + f * g as i16;
            fq::freeze(value as i32)
        };

        for i in 0..P {
            let mut result = 0i16;

            for j in 0..=i {
                result = quotient(result, f[j], g[i - j]);
            }

            fg[i] = result;
        }

        for i in P..P + P - 1 {
            let mut result = 0i16;

            for j in (i - P + 1)..P {
                result = quotient(result, f[j], g[i - j]);
            }

            fg[i] = result;
        }

        for i in (P..=(P + P - 2)).rev() {
            fg[i - P] = fq::freeze((fg[i - P] + fg[i]) as i32);
            fg[i - P + 1] = fq::freeze((fg[i - P + 1] + fg[i]) as i32);
        }

        out[..P].copy_from_slice(&fg[..P]);

        Rq::from(out)
    }

    /// Computes the inverse of a polynomial in the Fq field.
    ///
    /// # Arguments
    ///
    /// - `ratio`: The coefficient multiplier of the polynomial.
    ///
    /// # Returns
    ///
    /// Returns the result, which represents the inverse of the polynomial.
    ///
    /// # Example
    ///
    /// ```
    /// use ntrulp::poly::rq::Rq;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// const RATIO: i16 = 1;
    /// let mut rng = rand::thread_rng();
    /// let rq: Rq = Rq::from(short_random(&mut rng).unwrap());
    /// let out = rq.recip::<RATIO>().unwrap();
    /// let h = out.mult_r3(&rq.r3_from_rq());
    ///
    /// assert!(h.eq_one());
    /// ```
    ///
    /// # Notes
    ///
    /// This function calculates the inverse of a polynomial in the Fq field using the `ratio` coefficient as a multiplier for the polynomial.
    ///
    /// out = 1/(RATIO*F) in Rq
    pub fn recip<const RATIO: i16>(&self) -> Result<Rq, PolyErrors> {
        let input = self.coeffs;
        let mut out = [0i16; P];
        let mut f = [0i16; P + 1];
        let mut g = [0i16; P + 1];
        let mut v = [0i16; P + 1];
        let mut r = [0i16; P + 1];
        let mut delta: i16;
        let mut swap: i16;
        let mut t: i16;
        let mut f0: i32;
        let mut g0: i32;

        let quotient = |out: &mut [i16], f0: i32, g0: i32, fv: &[i16]| {
            for i in 0..P + 1 {
                let x = f0 * out[i] as i32 - g0 * fv[i] as i32;
                out[i] = fq::freeze(x);
            }
        };

        r[0] = fq::recip(RATIO);
        f[0] = 1;
        f[P - 1] = -1;
        f[P] = -1;

        for i in 0..P {
            g[P - 1 - i] = input[i];
        }

        g[P] = 0;
        delta = 1;

        for _ in 0..2 * P - 1 {
            for i in (1..=P).rev() {
                v[i] = v[i - 1];
            }
            v[0] = 0;

            swap = i16_negative_mask(-delta) & i16_nonzero_mask(g[0]);
            delta ^= swap & (delta ^ -delta);
            delta += 1;

            for i in 0..P + 1 {
                t = swap & (f[i] ^ g[i]);
                f[i] ^= t;
                g[i] ^= t;
                t = swap & (v[i] ^ r[i]);
                v[i] ^= t;
                r[i] ^= t;
            }

            f0 = f[0] as i32;
            g0 = g[0] as i32;

            quotient(&mut g, f0, g0, &f);
            quotient(&mut r, f0, g0, &v);

            for i in 0..P {
                g[i] = g[i + 1];
            }

            g[P] = 0;
        }

        let scale = fq::recip(f[0]);

        for i in 0..P {
            let x = scale as i32 * (v[P - 1 - i] as i32);
            out[i] = fq::freeze(x);
        }

        if i16_nonzero_mask(delta) == 0 {
            Ok(Rq::from(out))
        } else {
            Err(PolyErrors::NoSolutionRecip3)
        }
    }

    /// out = (num * poly) in Fq
    /// Multiplies a polynomial by a scalar integer value.
    ///
    /// This function multiplies each coefficient of the polynomial by the given integer `num`.
    ///
    /// # Arguments
    ///
    /// - `num`: The integer scalar value to multiply the polynomial by.
    ///
    /// # Returns
    ///
    /// Returns a new polynomial with coefficients multiplied by `num`.
    ///
    /// # Example
    ///
    /// ```
    /// use ntrulp::params::params::P;
    /// use ntrulp::poly::rq::Rq;
    /// use rand::RngCore;
    ///
    /// let mut rng = rand::thread_rng();
    /// let rq: Rq = Rq::from([1_i16; P]);
    /// let out = rq.mult_int(3);
    ///
    /// for i in 0..P {
    ///     assert_eq!(out.coeffs[i], 3);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// This function performs scalar multiplication of the polynomial by the provided integer value.
    ///
    pub fn mult_int(&self, num: i16) -> Rq {
        let mut out = [0i16; P];

        for (i, v) in out.iter_mut().enumerate() {
            let x = (num * self.coeffs[i]) as i32;

            *v = fq::freeze(x);
        }

        Rq::from(out)
    }

    pub fn r3_from_rq(&self) -> R3 {
        let mut out = [0i8; P];

        for (i, v) in out.iter_mut().enumerate() {
            *v = f3::freeze(self.coeffs[i])
        }

        R3::from(out)
    }

    pub fn to_bytes(&self) -> [u8; RQ_BYTES] {
        rq::encode(self.as_ref())
    }
}

impl From<[u8; RQ_BYTES]> for Rq {
    fn from(value: [u8; RQ_BYTES]) -> Self {
        rq::decode(&value).into()
    }
}

impl From<&[u8; RQ_BYTES]> for Rq {
    fn from(value: &[u8; RQ_BYTES]) -> Self {
        rq::decode(value).into()
    }
}

impl From<R3> for Rq {
    fn from(value: R3) -> Self {
        value.rq_from_r3()
    }
}

impl From<[i16; P]> for Rq {
    fn from(coeffs: [i16; P]) -> Self {
        Rq { coeffs }
    }
}

impl From<Rq> for [i16; P] {
    fn from(rq: Rq) -> Self {
        rq.coeffs
    }
}

impl AsRef<[i16; P]> for Rq {
    fn as_ref(&self) -> &[i16; P] {
        &self.coeffs
    }
}

impl AsMut<[i16; P]> for Rq {
    fn as_mut(&mut self) -> &mut [i16; P] {
        &mut self.coeffs
    }
}

impl AsRef<[i16]> for Rq {
    fn as_ref(&self) -> &[i16] {
        &self.coeffs
    }
}

impl AsMut<[i16]> for Rq {
    fn as_mut(&mut self) -> &mut [i16] {
        &mut self.coeffs
    }
}

impl Index<usize> for Rq {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coeffs[index]
    }
}

impl IndexMut<usize> for Rq {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coeffs[index]
    }
}

impl TryFrom<&[i16]> for Rq {
    type Error = &'static str;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != P {
            Err("Slice length does not match Rq size")
        } else {
            let mut coeffs = [0; P];
            coeffs.copy_from_slice(slice);
            Ok(Rq { coeffs })
        }
    }
}

impl IntoIterator for Rq {
    type Item = i16;
    type IntoIter = core::array::IntoIter<i16, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.coeffs.into_iter()
    }
}

impl<'a> IntoIterator for &'a Rq {
    type Item = &'a i16;
    type IntoIter = core::slice::Iter<'a, i16>;

    fn into_iter(self) -> Self::IntoIter {
        self.coeffs.iter()
    }
}

impl<'a> IntoIterator for &'a mut Rq {
    type Item = &'a mut i16;
    type IntoIter = core::slice::IterMut<'a, i16>;

    fn into_iter(self) -> Self::IntoIter {
        self.coeffs.iter_mut()
    }
}

impl PartialEq<[i16; P]> for Rq {
    fn eq(&self, other: &[i16; P]) -> bool {
        self.coeffs == *other
    }
}

#[cfg(test)]
mod test_rq {
    use super::*;
    use crate::rng::short_random;

    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_mult_int() {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut bytes = [0u8; 5];
        rng.fill_bytes(&mut bytes);
        let num = bytes[2] as i16;
        let rq: Rq = Rq::from([1_i16; P]);
        let out = rq.mult_int(num);

        for i in 0..P {
            assert_eq!(out.coeffs[i], num);
        }
    }

    #[test]
    fn test_recip() {
        const RATIO: i16 = 1;

        let mut rng = ChaCha20Rng::from_entropy();
        let rq: Rq = Rq::from(short_random(&mut rng).unwrap());
        let out = rq.recip::<RATIO>().unwrap();
        let h = out.mult_r3(&rq.r3_from_rq());

        assert!(h.eq_one());
    }
}
