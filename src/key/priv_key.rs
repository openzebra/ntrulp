use crate::kem::r3::R3;
use crate::kem::rq::Rq;

#[derive(Debug)]
pub struct PrivKey<const P: usize, const Q: usize, const Q12: usize> {
    pub f: Rq<P, Q, Q12>,
    pub ginv: R3<P, Q, Q12>,
}

impl<const P: usize, const Q: usize, const Q12: usize> PrivKey<P, Q, Q12> {
    pub fn new() -> Self {
        Self {
            f: Rq::new(),
            ginv: R3::new(),
        }
    }

    pub fn from(f: Rq<P, Q, Q12>, ginv: R3<P, Q, Q12>) -> Self {
        Self { f, ginv }
    }
}
