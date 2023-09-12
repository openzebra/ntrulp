use crate::{
    encode::{
        r3::{r3_decode, r3_encode},
        rq::{rq_decode, rq_encode},
    },
    kem::{errors::KemErrors, r3::R3, rq::Rq},
    ntru::errors::NTRUErrors,
};

use super::{priv_key::PrivKey, pub_key::PubKey};

pub struct KeyPair<
    const P: usize,
    const Q: usize,
    const Q12: usize,
    const RQ_BYTES: usize,
    const P_PLUS_ONE: usize,
    const P_TWICE_MINUS_ONE: usize,
> {
    pub pub_key: PubKey<P, Q, Q12>,
    pub priv_key: PrivKey<P, Q, Q12>,
}

impl<
        const P: usize,
        const Q: usize,
        const Q12: usize,
        const RQ_BYTES: usize,
        const P_PLUS_ONE: usize,
        const P_TWICE_MINUS_ONE: usize,
    > KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>
{
    const R3_BYTES: usize = (P + 3) / 4;

    pub fn new() -> Self {
        Self {
            pub_key: PubKey::new(),
            priv_key: PrivKey::new(),
        }
    }

    // h,(f,ginv)
    pub fn from_seed(&mut self, g: R3<P, Q, Q12>, f: Rq<P, Q, Q12>) -> Result<(), KemErrors> {
        let finv = f.recip3::<P_PLUS_ONE>()?;
        let ginv = g.recip::<P_PLUS_ONE>()?;
        let h = finv.mult_r3::<P_TWICE_MINUS_ONE>(&g);

        self.priv_key = PrivKey::from(f, ginv);
        self.pub_key = PubKey::from(h);

        Ok(())
    }

    pub fn verify(&self) -> bool {
        if self.priv_key.f.eq_zero() || self.priv_key.ginv.eq_zero() || self.pub_key.h.eq_zero() {
            return false;
        }

        // TODO: calc inverse and add verify method.

        true
    }

    // (PublicKey, SecretKey)
    pub fn export_pair(&mut self) -> Result<([u8; RQ_BYTES], Vec<u8>), NTRUErrors> {
        if !self.verify() {
            return Err(NTRUErrors::KeyExportError("PrivateKey and PubKey is empty"));
        }

        let h = self.pub_key.h.coeffs;
        let f = self.priv_key.f.r3_from_rq().coeffs;
        let ginv = self.priv_key.ginv.coeffs;

        let pk = rq_encode::<P, Q, Q12, RQ_BYTES>(&h);
        let mut sk = vec![0u8; Self::R3_BYTES * 2];
        let fencoded = r3_encode(&f)?.to_vec();
        let ginv_encoded = r3_encode(&ginv)?;

        sk[..Self::R3_BYTES].copy_from_slice(&ginv_encoded);
        sk[Self::R3_BYTES..].copy_from_slice(&fencoded);

        Ok((pk, sk))
    }

    pub fn import_pair(&mut self, pk: &[u8], sk: &[u8]) {
        let mut h: Rq<P, Q, Q12> = Rq::new();
        let mut f: R3<P, Q, Q12> = R3::new();
        let mut ginv: R3<P, Q, Q12> = R3::new();

        h.coeffs = rq_decode::<P, Q, Q12, RQ_BYTES>(pk);
        f.coeffs = r3_decode(&sk[Self::R3_BYTES..]);
        ginv.coeffs = r3_decode(&sk[..Self::R3_BYTES]);

        self.priv_key.ginv = ginv;
        self.priv_key.f = f.rq_from_r3();
        self.pub_key.h = h;
    }

    pub fn import_sk(&mut self, sk: &[u8]) -> Result<(), KemErrors> {
        let f: Rq<P, Q, Q12> = R3::from(r3_decode(&sk[Self::R3_BYTES..])).rq_from_r3();
        let ginv: R3<P, Q, Q12> = R3::from(r3_decode(&sk[..Self::R3_BYTES]));
        let g = ginv.recip::<P_PLUS_ONE>()?;
        let finv = f.recip3::<P_PLUS_ONE>()?;
        let h = finv.mult_r3::<P_TWICE_MINUS_ONE>(&g);

        self.priv_key.ginv = ginv;
        self.priv_key.f = f;
        self.pub_key.h = h;

        Ok(())
    }
}

#[cfg(test)]
mod test_pair {
    use super::*;

    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[test]
    fn test_key_gen_from_seed() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1158;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
        let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
        let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

        pair.from_seed(g, f).unwrap();

        pair.verify();
    }

    #[test]
    fn test_z_keys() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1158;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let mut pair0: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
        let mut pair1: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
        let mut pair2: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
        let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
        let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

        pair0.from_seed(g, f).unwrap();

        let (pk, sk) = pair0.export_pair().unwrap();

        pair1.import_pair(&pk, &sk);
        pair2.import_sk(&sk).unwrap();

        assert_eq!(&pair0.pub_key.h.coeffs, &pair1.pub_key.h.coeffs);
        assert_eq!(&pair0.priv_key.f.coeffs, &pair1.priv_key.f.coeffs);
        assert_eq!(&pair0.priv_key.ginv.coeffs, &pair1.priv_key.ginv.coeffs);

        assert_eq!(&pair0.pub_key.h.coeffs, &pair2.pub_key.h.coeffs);
        assert_eq!(&pair0.priv_key.f.coeffs, &pair2.priv_key.f.coeffs);
        assert_eq!(&pair0.priv_key.ginv.coeffs, &pair2.priv_key.ginv.coeffs);
    }
}
