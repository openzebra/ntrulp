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
    pub fn gen() {}

    pub fn from_seed(params: &NTRUParams, g: PolyInt<i16>, f: PolyInt<i8>) {}

    pub fn verify(&self) {}
}
