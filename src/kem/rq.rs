use crate::kem::fq;

use super::f3;

#[derive(Debug)]
pub struct Rq<const P: usize, const Q: usize, const Q12: usize> {
    coeffs: [i16; P],
}

impl<const P: usize, const Q: usize, const Q12: usize> Rq<P, Q, Q12> {
    pub fn new() -> Self {
        Self { coeffs: [0i16; P] }
    }

    // h = f*g in the ring Rq
    pub fn mult_small(&mut self, f: &[i16], g: &[i8]) {
        // TODO: possible make it on stack.
        let mut fg = vec![0i16; P + P - 1];

        for i in 0..P {
            let mut result = i16::default();

            for j in 0..=i {
                let value = result + f[j] * (g[i - j] as i16);
                result = fq::freeze::<Q12, Q>(value as i32);
            }

            fg[i] = result;
        }

        for i in P..P + P - 1 {
            let mut result = i16::default();

            for j in i - P + 1..P {
                let value = result + f[j] * (g[i - j] as i16);
                result = fq::freeze::<Q12, Q>(value as i32);
            }

            fg[i] = result;
        }

        for i in (P..P + P - 2).rev() {
            fg[i - P] = fq::freeze::<Q12, Q>((fg[i - P] + fg[i]) as i32);
            fg[i - P + 1] = fq::freeze::<Q12, Q>((fg[i - P + 1] + fg[i]) as i32);
        }

        for i in 0..P {
            self.coeffs[i] = fg[i];
        }
    }

    // h = 3f in Rq
    pub fn mult3(&mut self, f: &[i16]) {
        for i in 0..P {
            let x = (3 * f[i]) as i32;

            self.coeffs[i] = fq::freeze::<Q12, Q>(x);
        }
    }

    // int i;
    // for (i = 0; i < p; ++i)
    //   out[i] = F3_freeze(r[i]);
    // TODO: make it as R3 Poly
    pub fn r3_from_rq(&self) -> [i8; P] {
        let out = [0i8; P];

        for i in 0..P {
            // out[i] = f3::freeze(self.coeffs[i])
        }

        out
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

        let f = [0, 0, 1, 0, 0, -1, 0, -1, -1];
        let g = [-1, 0, -1, 1, -1, 0, 1, 0, 0];
        let mut h: Rq<P, Q, Q12> = Rq::new();

        h.mult_small(&f, &g);

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
}
