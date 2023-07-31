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

    pub fn generate(params: &StartParams) -> Self {
        let g = NtruIntPoly::random(params.0 as usize);
        let f = NtruIntPoly::fisher_yates_shuffle(params.0 as usize);

        NtruPrimeKeyPair::gen_from_seed(&params, g, f)
    }

    pub fn gen_from_seed(params: &StartParams, g: NtruIntPoly, f: NtruIntPoly) -> Self {
        let (_, q, w) = params;
        let g_inv = loop {
            match g.get_inv_poly(*q) {
                Some(inv) => {
                    break inv;
                }
                None => continue,
            }
        };
        let priv_key = NtruPrimePrivKey { f, g_inv };
        let mut pub_key = NtruPrimePubKey {
            h: NtruIntPoly::empty(),
        };
        let f_inv = loop {
            match priv_key.f.get_inv_poly(*q) {
                Some(inv) => {
                    break inv;
                }
                None => continue,
            }
        };

        pub_key.h.mult_poly(&g, &f_inv, *q);
        pub_key.h.mult_mod(*w as u64, *q as u64);

        NtruPrimeKeyPair {
            private: priv_key,
            public: pub_key,
        }
    }
}

#[test]
fn test_key_pair_generate() {
    let pair = NtruPrimeKeyPair::generate(&SNTRUP761);

    assert!(pair.private.f.n == SNTRUP761.0 as usize);
    assert!(pair.private.f.coeffs.contains(&0));
    assert!(pair.private.f.coeffs.contains(&1));
    assert!(pair.private.f.coeffs.contains(&2));
}
