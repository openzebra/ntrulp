use crate::{
    kem::{errors::KemErrors, r3::R3, rq::Rq},
    random::NTRURandom,
};

pub struct KeyPair<const P: usize, const Q: usize, const Q12: usize> {}

impl<const P: usize, const Q: usize, const Q12: usize> KeyPair<P, Q, Q12> {
    pub fn new() -> Self {
        Self {}
    }

    pub fn key_gen_from_seed(&self, g: &R3<P, Q, Q12>, f: &R3<P, Q, Q12>) -> Result<(), KemErrors> {
        let mut finv = f.recip3()?;
        let mut ginv = g.recip()?;
        let h: Rq<P, Q, Q12> = Rq::new();

        // h.mult_small(&finv, &g);
        // Rq_mult_small(h, &finv, &g);

        Ok(())
    }
}

#[cfg(test)]
mod test_pair {
    use super::*;
}
