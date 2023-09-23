#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{R3_BYTES, SECRETKEYS_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{R3_BYTES, SECRETKEYS_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{R3_BYTES, SECRETKEYS_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{R3_BYTES, SECRETKEYS_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{R3_BYTES, SECRETKEYS_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{R3_BYTES, SECRETKEYS_BYTES};

use crate::{
    encode::r3,
    ntru::errors::NTRUErrors,
    poly::{errors::KemErrors, r3::R3, rq::Rq},
};

pub struct PrivKey(R3, R3);

impl PrivKey {
    pub fn compute(f: &Rq, g: &R3) -> Result<Self, KemErrors> {
        let ginv = g.recip()?;

        Ok(PrivKey(f.r3_from_rq(), ginv))
    }

    pub fn as_bytes(&self) -> [u8; SECRETKEYS_BYTES] {
        let mut sk = [0u8; SECRETKEYS_BYTES];
        let f = &self.0;
        let ginv = &self.1.coeffs;
        let f_bytes = r3::r3_encode(&f.coeffs);
        let ginv_bytes = r3::r3_encode(ginv);

        sk[..R3_BYTES].copy_from_slice(&ginv_bytes);
        sk[R3_BYTES..].copy_from_slice(&f_bytes);

        sk
    }

    pub fn import(sk: &[u8; SECRETKEYS_BYTES]) -> Result<Self, NTRUErrors> {
        let common_error = NTRUErrors::PrivateKeyImport("Incorrect SK");
        let ginv_bytes: [u8; R3_BYTES] = match sk[..R3_BYTES].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(common_error),
        };
        let f_bytes: [u8; R3_BYTES] = match sk[R3_BYTES..].try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(common_error),
        };

        let ginv = R3::from(r3::r3_decode(&ginv_bytes));
        let f = R3::from(r3::r3_decode(&f_bytes));

        Ok(PrivKey(f, ginv))
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

        for _ in 0..2 {
            let f: Rq = Rq::from(random.short_random().unwrap());
            let g: R3 = R3::from(random.random_small().unwrap());
            let secret_key = PrivKey::compute(&f, &g).unwrap();
            let bytes = secret_key.as_bytes();
            let new_secret_key = match PrivKey::import(&bytes) {
                Ok(v) => v,
                Err(_) => continue,
            };

            assert_eq!(new_secret_key.0.coeffs, secret_key.0.coeffs);
            assert_eq!(new_secret_key.1.coeffs, secret_key.1.coeffs);
        }
    }
}
