use crate::kem::rq::Rq;

#[derive(Debug)]
pub struct PubKey<const P: usize, const Q: usize, const Q12: usize> {
    pub h: Rq<P, Q, Q12>,
}

impl<const P: usize, const Q: usize, const Q12: usize> PubKey<P, Q, Q12> {
    pub fn new() -> Self {
        let h = Rq::new();

        Self { h }
    }

    pub fn from(h: Rq<P, Q, Q12>) -> Self {
        Self { h }
    }
}
