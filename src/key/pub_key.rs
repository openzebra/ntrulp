#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::RQ_BYTES;
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::RQ_BYTES;
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::RQ_BYTES;
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::RQ_BYTES;
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::RQ_BYTES;
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::RQ_BYTES;

use crate::{
    encode::rq,
    ntru::errors::NTRUErrors,
    poly::{r3::R3, rq::Rq},
};

pub type PubKey = Rq;

impl PubKey {
    pub fn from_sk(f: &Rq, g: &R3) -> Self {
        let finv = f.recip::<3>().unwrap();

        finv.mult_r3(&g)
    }

    pub fn import(bytes: &[u8; RQ_BYTES]) -> Result<Self, NTRUErrors> {
        let coeffs = rq::decode(&bytes);
        let h = Rq::from(coeffs);

        Ok(h)
    }

    pub fn as_bytes(&self) -> [u8; RQ_BYTES] {
        let h = self.coeffs;

        rq::encode(&h)
    }
}

#[cfg(test)]
mod test_pub_key {
    use super::*;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[test]
    fn test_import_export() {
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..1 {
            let f: Rq = Rq::from(random.short_random().unwrap());
            let g: R3 = R3::from(random.random_small().unwrap());
            let pub_key = PubKey::from_sk(&f, &g);
            let bytes = pub_key.as_bytes();
            let new_pub_key = match PubKey::import(&bytes) {
                Ok(v) => v,
                Err(_) => continue,
            };

            assert_eq!(new_pub_key.coeffs, pub_key.coeffs);
        }
    }
}
