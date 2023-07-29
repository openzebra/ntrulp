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
        let g = NtruIntPoly::random(p as usize);
        let g_inv = loop {
            match g.get_inv_poly(q) {
                Some(inv) => {
                    break inv;
                }
                None => continue,
            }
        };
        let priv_key = NtruPrimePrivKey {
            p: 8,
            f: NtruIntPoly::random(p as usize),
            g_inv: NtruIntPoly::random(p as usize),
        };

        // println!("{:?}", priv_key);
    }
}

#[test]
fn test_key_pair_generate() {
    let pair = NtruPrimeKeyPair::generate(config::params::SNTRUP761);
}
