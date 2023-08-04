use crate::math;
use crate::math::finite_field::GF;
use crate::params::params::NTRUParams;
use rand::Rng;
use std::io::{Error, ErrorKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct NTRUPrime {
    pub params: NTRUParams,
    r: GF<i8>,
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

        let r: GF<i8> = GF::new(1, 3);

        Ok(NTRUPrime { params, r })
    }

    pub fn encrypt(&self, msg: &[u8]) {}

    pub fn decrypt(&self) {}

    pub fn key_pair_gen(&self) {
        // let g = Arc::new([0u8, 2000]);
    }

    fn random_u32(&self) -> u32 {
        let mut rng = rand::thread_rng();
        // rng.gen::<u8>()
        let c0 = 1; //rng.gen::<u8>() as u32;
        let c1 = 2; //rng.gen::<u8>() as u32;
        let c2 = 3; //rng.gen::<u8>() as u32;
        let c3 = 4; //rng.gen::<u8>() as u32;

        c0 + 256 * c1 + 65536 * c2 + 16777216 * c3
    }

    fn randomrange3(&self) -> i8 {
        let r: u32 = self.random_u32();

        (((r & 0x3fffffff) * 3) >> 30) as i8
    }

    fn is_small(&self, r: &[i8]) -> bool {
        r.iter().all(|&value| value.abs() <= 1 && self.r.has(value))
    }

    fn small_random(&self) -> Option<Vec<i8>> {
        // TODO: Make it async.
        let r: Vec<i8> = vec![0u8; self.params.p]
            .iter_mut()
            .map(|_| self.randomrange3() - 1)
            .collect();

        if self.is_small(&r) {
            return Some(r);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[test]
    fn test_random_u32() {
        let ntrup = NTRUPrime::from(config::params::SNTRP_1277).unwrap();
        // let r = ntrup.small_random();
        // let r = ntrup.randomrange3();
    }
}
