use crate::config;
use crate::config::params::StartParams;
use crate::math;
use crate::math::finite_field::GF;

use std::io::{Error, ErrorKind};
// use std::sync::Arc;

#[derive(Debug)]
pub struct NTRU {
    pub params: StartParams,
    hash_bytes: Vec<u8>, // TODO: change to ARC<[u8]>
    usecache: bool,
    fq: GF<u64>,
    q12: u16,
}

impl NTRU {
    pub fn from(params: StartParams) -> Result<Self, Error> {
        let (round1, p, q, w) = params;
        let hash_bytes = vec![];
        let mut usecache = !round1;

        if !math::prime::is_prime(p) {
            return Err(Error::new(ErrorKind::Other, "p must be prime number"));
        }
        if !math::prime::is_prime(q) {
            return Err(Error::new(ErrorKind::Other, "q must be prime number"));
        }
        if !(w > 0) {
            return Err(Error::new(ErrorKind::Other, "w cannot be < 0"));
        }
        if !(2 * p >= 3 * w) {
            return Err(Error::new(ErrorKind::Other, "2*p should be >= 3*w"));
        }
        if !(q >= 16 * w + 1) {
            return Err(Error::new(ErrorKind::Other, "q should be >= 17 * w + 1"));
        }
        if !(q % 6 == 1) {
            // spec allows 5 but these tests do not
            return Err(Error::new(ErrorKind::Other, "q mod 6 should be = 1"));
        }
        if !(p % 4 == 1) {
            // spec allows 3 but ref C code does not
            return Err(Error::new(ErrorKind::Other, "p mod 4 should be = 1"));
        }

        if round1 {
            // encodings defined only for (761,4591)
            usecache = false;
            if !(p == config::params::SNTRUP4591761.1) {
                return Err(Error::new(ErrorKind::Other, "for round1 p should be 761"));
            }
            if !(q == config::params::SNTRUP4591761.2) {
                return Err(Error::new(ErrorKind::Other, "for round1 p should be 4591"));
            }
        }

        let fq = GF::new(1, q as u64);
        let q12 = (q - 1) / 2;

        Ok(NTRU {
            params,
            hash_bytes,
            usecache,
            fq,
            q12,
        })
    }

    pub fn gen_key_pair(&self) {
        // let key_len = self.params.1 as usize;
        // let mut rng = rand::thread_rng();
        // let mut g = vec![0i8; key_len];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_params() {
        let result = NTRU::from(config::params::SNTRUP4591761);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP761);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP653);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP857);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP953);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP1013);
        assert!(result.is_ok());
        let result = NTRU::from(config::params::SNTRUP1277);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_p_not_prime() {
        let invalid_params: StartParams = (false, 15, 4591, 135);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_q_not_prime() {
        let invalid_params: StartParams = (false, 761, 20, 135);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_w_negative() {
        let invalid_params: StartParams = (false, 761, 4591, 0);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_2p_not_greater_than_3w() {
        let invalid_params: StartParams = (false, 10, 8, 4);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_q_mod_6_not_1() {
        let invalid_params: StartParams = (false, 761, 4590, 135);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_p_mod_4_not_1() {
        let invalid_params: StartParams = (false, 14, 4591, 135);
        let result = NTRU::from(invalid_params);
        assert!(result.is_err());
    }
}
