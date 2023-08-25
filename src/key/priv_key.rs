use crate::kem::r3::R3;

#[derive(Debug)]
pub struct PrivKey<const P: usize, const Q: usize, const Q12: usize> {
    pub f: R3<P, Q, Q12>,
    pub ginv: R3<P, Q, Q12>,
}

impl<const P: usize, const Q: usize, const Q12: usize> PrivKey<P, Q, Q12> {
    pub fn new() -> Self {
        Self {
            f: R3::new(),
            ginv: R3::new(),
        }
    }

    pub fn from(f: R3<P, Q, Q12>, ginv: R3<P, Q, Q12>) -> Self {
        Self { f, ginv }
    }
}
