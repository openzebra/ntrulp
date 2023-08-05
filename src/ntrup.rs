use crate::math;
use crate::math::finite_field::GF;
use crate::params::params::NTRUParams;
use crate::poly::PolyInt;
use crate::random::{CommonRandom, NTRURandom};
use rand::Rng;
use std::io::{Error, ErrorKind};

pub struct NTRUPrime {
    pub params: NTRUParams,
    pub ntru_rng: NTRURandom,
}

impl NTRUPrime {
    pub fn from(params: NTRUParams) -> Result<Self, Error> {
        if !math::prime::is_prime(params.p) {
            return Err(Error::new(ErrorKind::Other, "p must be prime number"));
        }
        if !math::prime::is_prime(params.q) {
            return Err(Error::new(ErrorKind::Other, "q must be prime number"));
        }
        if !(params.w > 0) {
            return Err(Error::new(ErrorKind::Other, "w cannot be < 0"));
        }
        if !(2 * params.p >= 3 * params.w) {
            return Err(Error::new(ErrorKind::Other, "2*p should be >= 3*w"));
        }
        if !(params.q >= 16 * params.w + 1) {
            return Err(Error::new(ErrorKind::Other, "q should be >= 17 * w + 1"));
        }
        if !(params.q % 6 == 1) {
            // spec allows 5 but these tests do not
            return Err(Error::new(ErrorKind::Other, "q mod 6 should be = 1"));
        }

        let ntru_rng = NTRURandom::new();

        Ok(NTRUPrime { params, ntru_rng })
    }

    pub fn encrypt(&self, msg: &[u8]) {}

    pub fn decrypt(&self) {}

    pub fn key_pair_gen(&mut self) {
        // TODO: Add counter, if specific random return error.
        let g = loop {
            let r = self.ntru_rng.random_small_vec(self.params.p);
            let g: PolyInt<i8> = PolyInt::from(&r);

            if r.contains(&0) && r.contains(&1) && r.contains(&-1) && g.is_small() {
                break g;
            }
        };

        // dbg!(g.coeffs);
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[test]
    fn test_key_pair_gen() {
        let mut ntru = NTRUPrime::from(config::params::SNTRP_761).unwrap();

        ntru.key_pair_gen();
    }
}
