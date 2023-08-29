use crate::{
    encode::{r3::r3_encode, rq::rq_encode},
    ntrup::{NTRUErrors, NTRUPrime},
};

pub struct NTRULPRime<const P: usize, const Q: usize, const W: usize, const Q12: usize> {
    pub ntrup: NTRUPrime<P, Q, W, Q12>,
}

impl<const P: usize, const Q: usize, const W: usize, const Q12: usize> NTRULPRime<P, Q, W, Q12> {
    pub fn new() -> Result<Self, NTRUErrors> {
        let ntrup: NTRUPrime<P, Q, W, Q12> = NTRUPrime::new()?;

        Ok(Self { ntrup })
    }

    pub fn from(ntrup: NTRUPrime<P, Q, W, Q12>) -> Self {
        Self { ntrup }
    }

    pub fn kem_key_gen(&self) {}
}
