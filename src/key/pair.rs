use crate::kem::{errors::KemErrors, r3::R3, rq::Rq};

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
    pub fn from_seed(&self, g: R3<P, Q, Q12>, f: Rq<P, Q, Q12>) -> Result<Self, KemErrors> {
        let finv = f.recip3()?;
        let ginv = g.recip()?;
        let h = finv.mult_small(&g);

        Ok(Self {
            priv_key: PrivKey::from(f, ginv),
            pub_key: PubKey::from(h),
        })
    }

    pub fn verify(&self) -> bool {
        let f3 = self.priv_key.f.r3_from_rq();
        let mut a = self.pub_key.h.mult_small(&f3);

        a.mult_mod(3);

        let b = a.mult_small(&self.priv_key.ginv);

        b.eq_zero()
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
        let pair: KeyPair<P, Q, Q12> = KeyPair::new();
        let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
        let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

        pair.from_seed(g, f).unwrap();

        pair.verify();
    }
}
