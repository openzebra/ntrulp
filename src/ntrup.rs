use crate::math;
use std::io::{Error, ErrorKind};

pub struct NTRUPrime<const P: usize, const Q: usize, const W: usize> {}

impl<const P: usize, const Q: usize, const W: usize> NTRUPrime<P, Q, W> {
    pub fn new() -> Result<Self, Error> {
        if !math::prime::is_prime(P) {
            return Err(Error::new(ErrorKind::Other, "p must be prime number"));
        }
        if !math::prime::is_prime(Q) {
            return Err(Error::new(ErrorKind::Other, "q must be prime number"));
        }
        if !(W > 0) {
            return Err(Error::new(ErrorKind::Other, "w cannot be < 0"));
        }
        if !(2 * P >= 3 * W) {
            return Err(Error::new(ErrorKind::Other, "2*p should be >= 3*w"));
        }
        if !(Q >= 16 * W + 1) {
            return Err(Error::new(ErrorKind::Other, "q should be >= 17 * w + 1"));
        }
        if !(Q % 6 == 1) {
            // spec allows 5 but these tests do not
            return Err(Error::new(ErrorKind::Other, "q mod 6 should be = 1"));
        }

        Ok(NTRUPrime {})
    }

    pub fn encrypt(&self) {}

    pub fn decrypt(&self) {}

    pub fn key_pair_gen(&mut self) {}
}

#[cfg(test)]
mod tests {}
