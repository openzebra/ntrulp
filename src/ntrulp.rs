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

    fn get_z_keys<const R3_BYTES: usize>(&mut self) -> Result<(Vec<u8>, Vec<u8>), NTRUErrors> {
        if !self.ntrup.key_pair.verify() {
            return Err(NTRUErrors::KeysIsEmpty);
        }

        let h = self.ntrup.key_pair.pub_key.h.coeffs;
        let f = self.ntrup.key_pair.priv_key.f.r3_from_rq().coeffs;
        let ginv = self.ntrup.key_pair.priv_key.ginv.coeffs;

        let pk = rq_encode::<P, Q, Q12>(&h);
        let mut sk = r3_encode(&f);

        let (sk_first, sk_rest) = sk.split_at_mut(R3_BYTES);
        let sk_first = r3_encode(&ginv);

        Ok((pk, sk))
    }
}
