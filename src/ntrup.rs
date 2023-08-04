use crate::math;
use crate::params::params::NTRUParams;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct NTRUPrime {
    pub params: NTRUParams,
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

        Ok(NTRUPrime { params })
    }

    pub fn encrypt(&self, msg: &[u8]) {}
    pub fn decrypt(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
