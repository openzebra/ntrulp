use crate::config::params::StartParams;
use crate::math;
use crate::math::finite_field::GF;
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
        if !(p % 4 == 1) {
            // spec allows 3 but ref C code does not
            return Err(Error::new(ErrorKind::Other, "p mod 4 should be = 1"));
        }

        Ok(NTRUPrime {
            params,
            key_pair: NtruPrimeKeyPair::empty(params),
        })
    }

    pub fn encrypt(&self) {}
    pub fn decrypt(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
