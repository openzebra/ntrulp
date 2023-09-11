use super::{errors::KemErrors, f3, fq, rq::Rq};
use crate::math::nums::{i16_negative_mask, i16_nonzero_mask};

#[derive(Debug)]
pub struct R3<const P: usize, const Q: usize, const Q12: usize> {
    pub coeffs: [i8; P],
}

impl<const P: usize, const Q: usize, const Q12: usize> R3<P, Q, Q12> {
    pub fn new() -> Self {
        Self { coeffs: [0i8; P] }
    }

    pub fn from(coeffs: [i8; P]) -> Self {
        Self { coeffs }
    }

    pub fn eq_zero(&self) -> bool {
        for c in self.coeffs {
            if c != 0 {
                return false;
            }
        }

        true
    }

    // h = f*g in the ring R3
    pub fn mult<const P_TWICE_MINUS_ONE: usize>(&self, g3: &R3<P, Q, Q12>) -> R3<P, Q, Q12> {
        let f = self.coeffs;
        let g = g3.coeffs;
        let mut out = [0i8; P];
        let mut fg = [0i8; P_TWICE_MINUS_ONE];

        let quotient = |r: i8, f: i8, g: i8| {
            let x = r + f * g;

            f3::freeze(x as i16)
        };

        for i in 0..P {
            let mut r = 0i8;
            for j in 0..=i {
                r = quotient(r, f[j], g[i - j]);
            }
            fg[i] = r;
        }
        for i in P..P_TWICE_MINUS_ONE {
            let mut r = 0i8;
            for j in (i - P + 1)..P {
                r = quotient(r, f[j], g[i - j]);
            }
            fg[i] = r;
        }

        for i in (P..P + P - 1).rev() {
            let x0 = fg[i - P] + fg[i];
            let x1 = fg[i - P + 1] + fg[i];

            fg[i - P] = f3::freeze(x0 as i16);
            fg[i - P + 1] = f3::freeze(x1 as i16);
        }

        out[..P].clone_from_slice(&fg[..P]);

        R3::from(out)
    }

    pub fn eq_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != 0 {
                return false;
            }
        }

        self.coeffs[0] == 1
    }

    pub fn recip<const P_PLUS_ONE: usize>(&self) -> Result<R3<P, Q, Q12>, KemErrors> {
        let input = self.coeffs;
        let mut out = [0i8; P];
        let mut f = [0i8; P_PLUS_ONE];
        let mut g = [0i8; P_PLUS_ONE];
        let mut v = [0i8; P_PLUS_ONE];
        let mut r = [0i8; P_PLUS_ONE];
        let mut delta: i8;
        let mut sign: i8;
        let mut swap: i8;
        let mut t: i8;

        let quotient = |g: i8, sign: i8, f: i8| {
            let x = g + sign * f;
            f3::freeze(x as i16)
        };

        for i in 0..P + 1 {
            v[i] = 0;
        }
        for i in 0..P + 1 {
            r[i] = 0;
        }

        r[0] = 1;

        for i in 0..P {
            f[i] = 0;
        }

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

            sign = -g[0] * f[0];
            swap = (i16_negative_mask(-delta as i16) & i16_nonzero_mask(g[0] as i16)) as i8;
            delta ^= swap & (delta ^ -delta);
            delta += 1;

            for i in 0..P_PLUS_ONE {
                t = swap & (f[i] ^ g[i]);
                f[i] ^= t;
                g[i] ^= t;
                t = swap & (v[i] ^ r[i]);
                v[i] ^= t;
                r[i] ^= t;
            }

            for i in 0..P + 1 {
                g[i] = quotient(g[i], sign, f[i]);
            }
            for i in 0..P + 1 {
                r[i] = quotient(r[i], sign, v[i]);
            }

            for i in 0..P {
                g[i] = g[i + 1];
            }
            g[P] = 0;
        }

        sign = f[0];
        for i in 0..P {
            out[i] = (sign * v[P - 1 - i]) as i8;
        }

        if i16_nonzero_mask(delta as i16) == 0 {
            Ok(R3::from(out))
        } else {
            Err(KemErrors::R3NoSolutionRecip)
        }
    }

    pub fn rq_from_r3(&self) -> Rq<P, Q, Q12> {
        let mut out = [0i16; P];

        for i in 0..P {
            out[i] = fq::freeze::<Q12, Q>(self.coeffs[i].into());
        }

        Rq::from(out)
    }
}

#[cfg(test)]
mod test_r3 {
    use crate::random::{CommonRandom, NTRURandom};

    use super::*;

    #[test]
    fn test_r3_mult() {
        const P: usize = 761;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let f: R3<P, Q, Q12> = R3::from([
            1, 0, -1, 0, 1, -1, 0, 0, -1, 0, -1, 1, -1, -1, 0, 1, 1, 0, 0, 0, 0, -1, 0, -1, 0, 1,
            0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 0, 0, 0, -1, 0, 0, 1, 1, 1, -1, 1, 1, 1, 1,
            0, 0, 1, -1, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 1, -1, -1, -1, 0,
            0, 1, 0, -1, 1, 1, -1, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, -1, 0, -1, 0, -1, 1, 1, 0, 0,
            1, -1, 0, 1, 0, -1, 0, -1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, -1, 0, 1, 0, 0, 1, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, 0, 0, -1, 0, 0, 1, 0, 1,
            0, 0, 0, -1, 1, 0, -1, 1, 0, 0, 1, 0, 1, -1, 0, 0, 1, 0, 1, -1, 1, 0, 1, 0, -1, 1, 0,
            0, -1, 0, 0, 0, 0, 0, 1, -1, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, -1, 0,
            0, 0, -1, 0, -1, 1, 1, -1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, -1, 0, 0, 1, 0, 0, -1,
            0, 0, -1, 1, 1, 0, 0, 1, 0, 1, 1, -1, -1, 0, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, 0, -1,
            0, 1, 1, -1, -1, -1, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, -1, -1, 0, -1, 0, -1, -1,
            0, 0, 1, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, -1, 0, 0, 0, 0, 1, 0, 0, -1, 0,
            0, -1, -1, 0, 0, 0, 1, 0, 1, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0,
            0, 0, 1, -1, 0, 0, 0, -1, 1, 1, 1, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1, 0,
            -1, 0, 1, 0, 0, 1, -1, 0, 0, 0, 1, 0, 0, 1, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, -1, 0,
            0, 0, -1, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1,
            0, 0, 1, -1, -1, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, -1, -1, 0, 0, 0, 0, -1,
            -1, -1, 0, 1, 0, 1, -1, 0, -1, 0, -1, -1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 1, -1, -1, 0, -1, 0, 1, 0, -1, 0,
            0, 0, 0, 0, 1, -1, 0, 0, -1, 1, 0, 1, 0, 0, 1, -1, 0, 0, 0, 1, 0, 0, 0, 0, -1, 1, 0, 0,
            0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, -1, 1, 0, 1, 0, 0, 1, -1, 1, 0, 1,
            1, -1, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, 0, 0, 0,
            1, -1, 0, -1, 1, 0, 0, 1, 0, -1, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, -1, -1, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, -1, 0, -1, 1, 1, 0, 0, 1, 0, 1, -1, -1, 0, 1,
            -1, -1, 0, 0, 0, 0, -1, 1, 0, 0, -1, -1, 0, 0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 1, -1, 1, 0,
            0, 0, 1, 1, 1, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, -1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
        ]);
        let g: R3<P, Q, Q12> = R3::from([
            -1, 1, -1, 0, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, -1,
            -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
            -1, 0, -1, -1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 1, 0, 0, -1, -1,
            0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0,
            0, -1, 1, 1, -1, 0, -1, -1, 0, 1, 0, 0, -1, -1, 1, 1, 0, -1, 0, 0, -1, 1, 0, -1, 0, 1,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0, 0,
            1, 0, 1, 1, -1, 0, 1, 0, -1, 1, 0, 0, 0, 1, 1, 0, 1, -1, 1, 0, 1, -1, 0, 0, 0, -1, 1,
            0, 1, 1, -1, 0, 0, 1, 0, 0, -1, -1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0,
            -1, 1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 1, 0, 1, -1, 0, 0, 0, 1, 0, 0, 1,
            -1, 1, -1, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 1, 1, 1, 0, 0, -1, 1,
            0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 1, 0, 0, 1, -1, 0, -1, 0, 0, 0, 0, 0, 1, 0, -1,
            1, 0, -1, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, -1, -1, 0, 1, 1, 1, -1, 0, 0, 0, -1, -1, 1,
            0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, -1, 0, 0, -1, 0, 1, 1,
            0, -1, -1, 0, 0, 1, 0, 1, -1, -1, 0, 1, 0, 0, 0, 1, 0, 0, -1, -1, -1, 0, -1, 1, -1, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, -1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            0, -1, 1, 0, 0, -1, 0, 0, 0, -1, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0,
            -1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, -1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, -1, 0, 0, 1, -1, 0, 0, 1, -1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 1, 1, 0, -1, 1, 0, 0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 1, 0, 1, -1,
            0, 0, -1, 1, -1, 0, 1, 0, 1, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 1,
            0, 0, 1, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, -1, 1, 0, 0, 0, 1, 0, 1,
            -1, 0, 1, 0, 0, 0, 0, 1, -1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 0, 0, 0, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 1, -1, 1, -1, -1, 1, 0, 1, -1, -1, 0, 0,
            0, 1, -1, -1, 1, 0, 1, -1, 1, 0, 0, 0, 0, -1, 0, 0, 0, -1, 1, 1, 0, 0, 0, 1, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 0, -1, 0, 0, 0, 1, -1, 0, -1, 0, -1, 0, 0, -1, 0, 0,
            1, -1, 0, 0, 1, 0, 0, 0, 1, -1, 0, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0,
            0, 0, 1, 1, 1, -1, -1, -1, 0, 0, 0, 0, 1, -1, 1, 0, 0, 0, 0, 0, 1, 0, -1, 0, 1, -1, 0,
            0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 1,
        ]);
        let h = f.mult::<P_TWICE_MINUS_ONE>(&g);

        assert_eq!(
            h.coeffs,
            [
                -1, 1, 1, 0, 0, 1, -1, 1, 0, 1, 0, 1, 0, 1, 1, -1, 0, 0, 0, 1, 0, 1, -1, 0, -1, -1,
                0, 0, 0, 0, -1, -1, 0, 1, 0, 1, -1, -1, 1, 0, -1, -1, 1, 0, 0, -1, 1, 1, 1, -1, 1,
                1, 0, 1, -1, -1, 0, 1, 1, -1, -1, -1, 0, -1, 0, -1, 1, 1, -1, 0, 0, 0, -1, 0, 0,
                -1, -1, 0, -1, 1, 1, 1, -1, 0, -1, -1, -1, 1, -1, 0, -1, 0, 1, 1, -1, 0, -1, 0, 0,
                0, -1, 0, -1, -1, -1, -1, 0, -1, -1, 1, 0, -1, 0, 1, 1, 0, 0, 1, 0, 0, -1, 0, 1,
                -1, -1, -1, 0, -1, 1, -1, 0, 1, 1, 1, 0, -1, 1, -1, -1, 0, -1, 1, 1, 1, 1, -1, 1,
                -1, 1, 0, 1, 1, 1, -1, 1, 1, 0, -1, 1, -1, 0, 1, -1, -1, 0, 0, 1, -1, -1, -1, 1, 0,
                0, -1, -1, 0, 0, 0, 0, -1, -1, 0, 1, -1, -1, 0, 1, 1, 0, 1, 1, -1, 0, 0, 1, 1, -1,
                0, 0, 1, 0, 0, 1, -1, -1, 1, -1, -1, -1, 1, -1, -1, 1, 1, -1, -1, -1, 1, 0, 1, 0,
                0, 1, 1, 1, 0, 1, 0, 0, -1, 0, -1, 1, 1, -1, -1, 0, 0, 0, -1, 1, -1, 1, 0, 0, -1,
                0, 0, 0, -1, -1, 0, 0, -1, -1, -1, -1, -1, -1, -1, 1, 0, 0, 0, -1, 0, -1, 0, 1, 1,
                0, -1, -1, 0, 1, 1, 0, 0, 1, -1, 0, 0, -1, 0, 0, -1, 1, 1, -1, -1, 0, -1, 1, 0, 0,
                0, 1, 0, -1, 1, -1, 1, -1, 0, 0, 1, 0, -1, 1, -1, -1, -1, -1, -1, -1, 1, -1, -1,
                -1, -1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, -1, -1, 1, -1, 1, -1, 0, 0, 1, 1, 1, 1,
                0, 0, 1, 0, -1, -1, -1, -1, 0, -1, 1, -1, -1, 0, -1, 0, 0, 0, -1, -1, 0, -1, 0, -1,
                0, 0, -1, 1, 1, 1, -1, -1, 0, 0, 0, -1, -1, 0, 0, 1, 0, -1, 1, -1, -1, 1, 0, 0, 1,
                0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 1, 0, 1, 0, -1, -1, 0, 1, 1, 1, 0, 1, -1, -1,
                -1, 1, 0, 1, -1, 1, 0, 0, 0, 1, 0, -1, -1, -1, 0, 0, 1, 1, -1, 0, 0, 1, 1, 1, 1,
                -1, 0, -1, -1, -1, 0, 1, 0, 1, -1, 0, -1, 0, -1, 1, -1, 0, -1, 0, -1, 1, 0, 0, 1,
                -1, 1, -1, 0, 0, -1, 0, -1, 1, 0, -1, -1, 0, 0, -1, 0, 0, 1, -1, 1, 0, 1, -1, 0, 0,
                1, 1, 0, 0, -1, 1, -1, 0, -1, 0, 1, 1, 0, 0, 1, 0, -1, -1, 1, 1, 0, 0, 1, 1, 1, 1,
                -1, 1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, -1,
                -1, -1, 1, 1, 0, -1, -1, 1, 1, -1, 0, 1, -1, 1, 0, 0, 0, 1, 1, -1, 0, 1, 1, 1, 1,
                1, 1, -1, 1, 0, 1, 0, -1, 1, -1, 1, -1, 1, -1, 1, 0, 0, -1, 0, -1, 1, 1, -1, 1, -1,
                0, 1, 0, -1, 1, 0, 0, -1, 1, 1, 0, 1, -1, 0, 1, -1, 1, -1, 1, 1, -1, 0, 1, -1, -1,
                1, 0, -1, 0, 1, 0, 0, 0, -1, -1, 0, 0, 0, 1, 1, 1, 1, -1, 1, 1, 1, -1, 1, -1, 1, 1,
                0, -1, -1, 0, -1, -1, 0, 0, 0, 0, -1, 0, -1, 1, 0, -1, 0, 0, -1, -1, -1, 1, -1, 1,
                -1, -1, 0, -1, 0, 1, 0, -1, 1, -1, 1, 0, 0, -1, 0, -1, -1, 1, 1, 0, 0, -1, -1, 0,
                0, 0, 1, -1, 0, -1, -1, -1, 0, -1, -1, -1, 1, 1, 0, 0, 0, 0, -1, -1, 1, 0, 1, 0,
                -1, -1, 0, 0, 1, 0, 1, 0, 0, 0, -1, -1, 0, 1, 0, 0, -1, 1, 1, 0, 0, -1, 0, 0, 1,
                -1, 0, -1, 0, 0, -1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 0, 1, -1, 1
            ]
        );
    }

    #[test]
    fn test_recip_761() {
        const P: usize = 761;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..2 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }

    #[test]
    fn test_recip_857() {
        const P: usize = 857;
        const Q: usize = 5167;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..10 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }

    #[test]
    fn test_recip_653() {
        const P: usize = 653;
        const Q: usize = 4621;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..10 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }

    #[test]
    fn test_recip_953() {
        const P: usize = 953;
        const Q: usize = 6343;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..10 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }

    #[test]
    fn test_recip_1013() {
        const P: usize = 1013;
        const Q: usize = 7177;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..10 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }

    #[test]
    fn test_recip_1277() {
        const P: usize = 1277;
        const Q: usize = 7879;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom<P> = NTRURandom::new();

        for _ in 0..10 {
            let r3: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

            let out = match r3.recip::<P_PLUS_ONE>() {
                Ok(o) => o,
                Err(_) => continue,
            };
            let one = out.mult::<P_TWICE_MINUS_ONE>(&r3);

            assert_eq!(one.coeffs[0], 1);
            assert!(one.eq_one());
        }
    }
}
