use crate::config::params::StartParams;
use crate::math;
use crate::{config, key::pair::NtruPrimeKeyPair};

use std::io::{Error, ErrorKind};
// use std::sync::Arc;

#[derive(Debug)]
pub struct NTRUPrime {
    pub params: StartParams,
    pub key_pair: NtruPrimeKeyPair,
}

impl NTRUPrime {
    pub fn from(params: StartParams) -> Result<Self, Error> {
        let (p, q, w, _) = params;

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

        Ok(NTRUPrime {
            params,
            key_pair: NtruPrimeKeyPair::empty(params),
        })
    }

    pub fn encrypt(&self, msg: &[u8]) {}
    pub fn decrypt(&self) {}

    fn enc_len(&self) -> usize {
        let (p, q, _, _) = self.params;

        self.enc_len_pq(p, q)
    }

    fn enc_len_pq(&self, p: u16, q: u16) -> usize {
        // Make sure q is a power of 2
        if q & (q - 1) != 0 {
            return 0;
        }

        let len_bits = p as f64 * (q as f64).log2();

        let len_bytes = (len_bits + 7.0) / 8.0;

        len_bytes as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enc_len_pq() {
        let params = config::params::SNTRUP761;
        let ntrup = NTRUPrime::from(params).unwrap();
        let result = ntrup.enc_len_pq(1171, 2048);

        assert!(result == 1611);
    }
}
