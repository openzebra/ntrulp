use crate::key::{priv_key::PrivKey, pub_key::PubKey};
use crate::params::params::NTRUParams;
use crate::poly::PolyInt;

#[derive(Debug)]
pub struct KeyPair<'a> {
    pub private: PrivKey,
    pub public: PubKey,
    params: &'a NTRUParams,
}

impl KeyPair<'_> {
    pub fn from_seed(params: &NTRUParams, g: PolyInt<i16>, f: PolyInt<i8>) {
        let x: Vec<i16> = vec![0, 1];
        let rq_f3 = f.clone().mult_int(3);
        let lq = g.create_factor_ring(&x, params.q as i16);
        let r3_q = g.create_factor_ring(&x, 3);

        // TODO: done the poly_inv!
    }

    pub fn verify(&self) {}
}
