use crate::{
    config::{self, params::StartParams},
    key::{private::NtruPrimePrivKey, public::NtruPrimePubKey},
    poly::poly::NtruIntPoly,
};

#[derive(Debug)]
pub struct NtruPrimeKeyPair {
    pub private: NtruPrimePrivKey,
    pub public: NtruPrimePubKey,
}

impl NtruPrimeKeyPair {
    pub fn from() {}

    pub fn generate(params: StartParams) {
        let (p, q, w) = params;
        let g = NtruIntPoly::new(p as usize);
        let g_inv = loop {
            match g.get_inv_poly(q) {
                Some(inv) => {
                    break inv;
                }
                None => continue,
            }
        };

        println!("{:?}", g_inv);
    }
}

#[test]
fn test_key_pair_generate() {
    let pair = NtruPrimeKeyPair::generate(config::params::SNTRUP761);
}
