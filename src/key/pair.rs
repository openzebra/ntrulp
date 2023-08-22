use crate::key::{priv_key::PrivKey, pub_key::PubKey};

#[derive(Debug)]
pub struct KeyPair<const SIZE: usize> {
    pub private: PrivKey<SIZE>,
    pub public: PubKey<SIZE>,
}

impl<const SIZE: usize> KeyPair<SIZE> {
    pub fn gen() {}

    pub fn from_seed() {}

    pub fn verify(&self) {}
}
