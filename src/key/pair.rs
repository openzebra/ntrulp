use crate::{
    encode::{
        r3::{r3_decode, r3_encode},
        rq::{rq_decode, rq_encode},
    },
    kem::{errors::KemErrors, r3::R3, rq::Rq},
    ntrup::NTRUErrors,
};

use super::{priv_key::PrivKey, pub_key::PubKey};

pub struct KeyPair<const P: usize, const Q: usize, const Q12: usize> {
    pub pub_key: PubKey<P, Q, Q12>,
    pub priv_key: PrivKey<P, Q, Q12>,
}

impl<const P: usize, const Q: usize, const Q12: usize> KeyPair<P, Q, Q12> {
    pub fn new() -> Self {
        Self {
            pub_key: PubKey::new(),
            priv_key: PrivKey::new(),
        }
    }

    // h,(f,ginv)
    pub fn from_seed(&mut self, g: R3<P, Q, Q12>, f: Rq<P, Q, Q12>) -> Result<(), KemErrors> {
        let finv = f.recip3()?;
        let ginv = g.recip()?;
        let h = finv.mult_small(&g);

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
    pub fn export_pair(&mut self) -> Result<(Vec<u8>, Vec<u8>), NTRUErrors> {
        if !self.verify() {
            return Err(NTRUErrors::KeysIsEmpty);
        }

        let r3_bytes = (P + 3) / 4;
        let h = self.pub_key.h.coeffs;
        let f = self.priv_key.f.r3_from_rq().coeffs;
        let ginv = self.priv_key.ginv.coeffs;

        let pk = rq_encode::<P, Q, Q12>(&h);
        let mut sk = vec![0u8; r3_bytes * 2];
        let fencoded = r3_encode(&f).to_vec();
        let ginv_encoded = r3_encode(&ginv);

        sk[..r3_bytes].copy_from_slice(&ginv_encoded);
        sk[r3_bytes..].copy_from_slice(&fencoded);

        Ok((pk, sk))
    }

    pub fn import_pair(&mut self, pk: &[u8], sk: &[u8]) {
        let r3_bytes = (P + 3) / 4;
        let mut h: Rq<P, Q, Q12> = Rq::new();
        let mut f: R3<P, Q, Q12> = R3::new();
        let mut ginv: R3<P, Q, Q12> = R3::new();

        h.coeffs = rq_decode::<P, Q, Q12>(pk);
        f.coeffs = r3_decode(&sk[r3_bytes..]);
        ginv.coeffs = r3_decode(&sk[..r3_bytes]);

        self.priv_key.ginv = ginv;
        self.priv_key.f = f.rq_from_r3();
        self.pub_key.h = h;
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

        let mut random: NTRURandom<P> = NTRURandom::new();
        let mut pair: KeyPair<P, Q, Q12> = KeyPair::new();
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
        const Q12: usize = (Q - 1) / 2;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let mut pair0: KeyPair<P, Q, Q12> = KeyPair::new();
        let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
        let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());
        let mut pair1: KeyPair<P, Q, Q12> = KeyPair::new();

        pair0.from_seed(g, f).unwrap();

        let (pk, sk) = pair0.export_pair().unwrap();

        pair1.import_pair(&pk, &sk);

        assert_eq!(&pair0.pub_key.h.coeffs, &pair1.pub_key.h.coeffs);
        assert_eq!(&pair0.priv_key.f.coeffs, &pair1.priv_key.f.coeffs);
        assert_eq!(&pair0.priv_key.ginv.coeffs, &pair1.priv_key.ginv.coeffs);
    }
}
