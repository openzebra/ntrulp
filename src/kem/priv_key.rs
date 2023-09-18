#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{SECRETKEYS_BYTES, SMALL_BYTES};

use crate::encode::r3;
use crate::ntru::errors::NTRUErrors;
use crate::poly::r3::R3;
use crate::poly::rq::Rq;

#[derive(Debug)]
pub struct PrivKey {
    pub f: Rq,
    pub ginv: R3,
}

impl PrivKey {
    pub fn new() -> Self {
        Self {
            f: Rq::new(),
            ginv: R3::new(),
        }
    }

    pub fn from(f: Rq, ginv: R3) -> Self {
        Self { f, ginv }
    }

    pub fn import(sk: &[u8; SECRETKEYS_BYTES]) -> Result<Self, NTRUErrors> {
        let common_error = NTRUErrors::PrivateKeyImport("Incorrect SK");
        let ginv_bytes: [u8; SMALL_BYTES] = match sk[..SMALL_BYTES].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(common_error),
        };
        let f_bytes: [u8; SMALL_BYTES] = match sk[SMALL_BYTES..].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(common_error),
        };

        let ginv = R3::from(r3::r3_decode(&ginv_bytes));
        let f = R3::from(r3::r3_decode(&f_bytes)).rq_from_r3();

        Ok(Self::from(f, ginv))
    }

    pub fn as_bytes(&self) -> [u8; SECRETKEYS_BYTES] {
        let mut sk = [0u8; SECRETKEYS_BYTES];
        let f = &self.f.r3_from_rq();
        let ginv = &self.ginv.coeffs;
        let f_bytes = r3::r3_encode(&f.coeffs);
        let ginv_bytes = r3::r3_encode(ginv);

        sk[..SMALL_BYTES].copy_from_slice(&ginv_bytes);
        sk[SMALL_BYTES..].copy_from_slice(&f_bytes);

        sk
    }
}

#[cfg(test)]
mod test_private_key {
    use super::*;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[test]
    fn test_import_export() {
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..10 {
            let f: Rq = Rq::from(random.short_random().unwrap());
            let g: R3 = R3::from(random.random_small().unwrap());
            let ginv = g.recip().unwrap();
            let secret_key = PrivKey::from(f, ginv);
            let bytes = secret_key.as_bytes();
            let new_secret_key = match PrivKey::import(&bytes) {
                Ok(v) => v,
                Err(_) => continue,
            };

            assert_eq!(new_secret_key.f.coeffs, secret_key.f.coeffs);
            assert_eq!(new_secret_key.ginv.coeffs, secret_key.ginv.coeffs);
        }
    }
}
