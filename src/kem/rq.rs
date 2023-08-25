use super::{f3, r3::R3};
use crate::kem::fq;

#[derive(Debug)]
pub struct Rq<const P: usize, const Q: usize, const Q12: usize> {
    coeffs: [i16; P],
}

impl<const P: usize, const Q: usize, const Q12: usize> Rq<P, Q, Q12> {
    pub fn new() -> Self {
        Self { coeffs: [0i16; P] }
    }

    pub fn from(coeffs: [i16; P]) -> Self {
        Self { coeffs }
    }

    /// Gets the slice of internal data.
    #[inline]
    pub fn get_coeffs(&self) -> &[i16; P] {
        &self.coeffs
    }

    pub fn eq_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != 0 {
                return false;
            }
        }

        self.coeffs[0] == -1530
    }

    // h = f*g in the ring Rq
    pub fn mult_small(&self, g3: &R3<P, Q, Q12>) -> Rq<P, Q, Q12> {
        // TODO Add hyperthreading.
        // TODO: possible make it on stack.
        let mut out = [0i16; P];
        let f = self.get_coeffs();
        let g = g3.get_coeffs();
        let mut fg = [0i16; 761 + 761 - 1];

        for i in 0..P {
            let mut result = 0i16;

            for j in 0..=i {
                let value = result + f[j] * (g[i - j] as i16);
                result = fq::freeze::<Q12, Q>(value as i32);
            }

            fg[i] = result;
        }

        for i in P..(P + P - 1) {
            let mut result = 0i16;

            for j in (i - P + 1)..P {
                let value = result as i32 + f[j] as i32 * (g[i - j] as i32);
                result = fq::freeze::<Q12, Q>(value);
            }

            fg[i] = result;
        }

        for i in (P..=(P + P - 2)).rev() {
            // TODO: -1530 = f * 1/f.
            fg[i - P] = fq::freeze::<Q12, Q>((fg[i - P] + fg[i]) as i32);
            fg[i - P + 1] = fq::freeze::<Q12, Q>((fg[i - P + 1] + fg[i]) as i32);
        }

        out[..P].clone_from_slice(&fg[..P]);

        Rq::from(out)
    }

    // h = 3f in Rq
    pub fn mult3(&mut self, f: &[i16]) {
        for i in 0..P {
            let x = (3 * f[i]) as i32;

            self.coeffs[i] = fq::freeze::<Q12, Q>(x);
        }
    }

    // TODO: add return it as R3 Poly
    pub fn r3_from_rq(&self) -> R3<P, Q, Q12> {
        let mut out = [0i8; P];

        for i in 0..P {
            out[i] = f3::freeze(self.coeffs[i])
        }

        R3::from(out)
    }
}

#[cfg(test)]
mod test_rq {
    use super::*;

    #[test]
    fn test_mult_small() {
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P: usize = 9;

        let f: Rq<P, Q, Q12> = Rq::from([0, 0, 1, 0, 0, -1, 0, -1, -1]);
        let g: R3<P, Q, Q12> = R3::from([-1, 0, -1, 1, -1, 0, 1, 0, 0]);
        let h = f.mult_small(&g);

        assert_eq!(h.coeffs, [2, 2, -2, 0, -1, 0, -2, 2, 1,])
    }

    #[test]
    fn test_mult3() {
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P: usize = 9;
        let f = [0, 0, 1, 0, 0, -1, 0, -1, -1];
        let mut h: Rq<P, Q, Q12> = Rq::new();

        h.mult3(&f);

        assert_eq!(h.coeffs, [0, 0, 3, 0, 0, -3, 0, -3, -3,])
    }

    #[test]
    fn test_r3_from_rq() {
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P: usize = 9;

        let h: Rq<P, Q, Q12> = Rq::from([0, 0, 1, 0, 0, -1, 0, -1, -1]);
        let r3 = h.r3_from_rq();

        assert_eq!(r3.get_coeffs(), &[0, 0, 1, 0, 0, -1, 0, -1, -1]);
    }
}
