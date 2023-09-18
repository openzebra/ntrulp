#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES};

use crate::{encode::rq, ntru::errors::NTRUErrors, poly::rq::Rq};

#[derive(Debug)]
pub struct PubKey {
    pub h: Rq,
    pub seed: [u8; SEEDS_BYTES],
}

impl PubKey {
    pub fn new() -> Self {
        let h = Rq::new();
        let seed = [0u8; SEEDS_BYTES];

        Self { h, seed }
    }

    pub fn from(h: Rq, seed: [u8; SEEDS_BYTES]) -> Self {
        Self { h, seed }
    }

    pub fn import(bytes: &[u8; PUBLICKEYS_BYTES]) -> Result<Self, NTRUErrors> {
        let seed: [u8; SEEDS_BYTES] = match bytes[..SEEDS_BYTES].try_into() {
            Ok(s) => s,
            Err(_) => return Err(NTRUErrors::PubKeyKeyImport("Incorrent SEED")),
        };
        let pk: [u8; ROUNDED_BYTES] = match bytes[SEEDS_BYTES..].try_into() {
            Ok(h) => h,
            Err(_) => return Err(NTRUErrors::PubKeyKeyImport("Incorrent PK")),
        };
        let coeffs = rq::rq_rounded_decode(&pk);
        let h = Rq::from(coeffs);

        Ok(Self { h, seed })
    }

    pub fn as_bytes(&self) -> [u8; PUBLICKEYS_BYTES] {
        let h = self.h.coeffs;
        let pk = rq::rq_rounded_encode(&h);
        let mut bytes = [0u8; PUBLICKEYS_BYTES];

        bytes[..SEEDS_BYTES].copy_from_slice(&self.seed);
        bytes[SEEDS_BYTES..].copy_from_slice(&pk);

        bytes
    }
}

#[cfg(test)]
mod test_pub_key {
    use super::*;
    use crate::poly::r3::R3;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;
    use rand::Rng;

    #[test]
    fn test_import_export() {
        let mut random: NTRURandom = NTRURandom::new();
        let mut seed = [0u8; SEEDS_BYTES];
        let mut rng = rand::thread_rng();

        for _ in 0..1 {
            rng.fill(&mut seed[..]);
            let f: Rq = Rq::from(random.short_random().unwrap());
            let g: R3 = R3::from(random.random_small().unwrap());
            let finv = f.recip3().unwrap();
            let h = finv.mult_r3(&g);
            let pub_key = PubKey::from(h, seed);
            let bytes = pub_key.as_bytes();
            let new_pub_key = match PubKey::import(&bytes) {
                Ok(v) => v,
                Err(_) => continue,
            };

            assert_eq!(new_pub_key.seed, pub_key.seed);
        }
    }
}
