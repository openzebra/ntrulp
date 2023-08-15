use crate::key::{priv_key::PrivKey, pub_key::PubKey};
use crate::params::params::NTRUParams;
use crate::poly::PolyInt;

use super::pub_key;

#[derive(Debug)]
pub struct KeyPair {
    pub private: PrivKey,
    pub public: PubKey,
    params: NTRUParams,
}

impl KeyPair {
    pub fn from_seed(params: &NTRUParams, g: PolyInt<i16>, f: PolyInt<i8>) {
        let x: Vec<i16> = vec![0, 1];
        let f3 = f.clone().mult_int(3);
        let gq = g.create_factor_ring(&x, params.q as i16);

        // TODO: done the poly_inv!
    }

    pub fn verify(&self) {}
}
