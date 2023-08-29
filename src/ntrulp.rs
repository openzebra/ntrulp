use crate::ntrup::{NTRUErrors, NTRUPrime};

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
}
