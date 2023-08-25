use crate::kem::rq::Rq;

#[derive(Debug)]
pub struct PubKey<const P: usize, const Q: usize, const Q12: usize> {
    pub h: Rq<P, Q, Q12>,
}

impl<const P: usize, const Q: usize, const Q12: usize> PubKey<P, Q, Q12> {
    pub fn new() -> Self {
        Self { h: Rq::new() }
    }

    pub fn from(h: Rq<P, Q, Q12>) -> Self {
        Self { h }
    }
}
