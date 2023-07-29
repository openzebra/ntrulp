use crate::{
    config::params::{StartParams, SNTRUP761},
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

    pub fn generate(params: StartParams) -> Self {
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
            g_inv,
            p: 8, // TODO: remove magic number
            f: NtruIntPoly::fisher_yates_shuffle(p as usize),
        };
        let mut pub_key = NtruPrimePubKey {
            p: 8, // TODO: remove magic number
            h: NtruIntPoly::empty(),
        };
        let f_inv = loop {
            match priv_key.f.get_inv_poly(q) {
                Some(inv) => {
                    break inv;
                }
                None => continue,
            }
        };

        pub_key.h.mult_poly(&g, &f_inv, q);
        pub_key.h.mult_mod(w as u64, q as u64);

        NtruPrimeKeyPair {
            private: priv_key,
            public: pub_key,
        }
    }
}

#[test]
fn test_key_pair_generate() {
    let pair = NtruPrimeKeyPair::generate(SNTRUP761);

    dbg!(pair);
}
