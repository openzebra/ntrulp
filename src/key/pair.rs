use crate::{
    config::params::{StartParams, SNTRUP761},
    key::{self, private::NtruPrimePrivKey, public::NtruPrimePubKey},
    poly::poly::NtruIntPoly,
};

#[derive(Debug)]
pub struct NtruPrimeKeyPair {
    pub private: NtruPrimePrivKey,
    pub public: NtruPrimePubKey,
}

impl NtruPrimeKeyPair {
    pub fn from() {}

    pub fn gen(params: &StartParams) -> Self {
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
fn test_key_pair_gen() {
    let pair = NtruPrimeKeyPair::gen(&SNTRUP761);

    assert!(pair.private.f.n == SNTRUP761.0 as usize);
    assert!(pair.private.f.coeffs.contains(&0));
    assert!(pair.private.f.coeffs.contains(&1));
    assert!(pair.private.f.coeffs.contains(&2));
}

#[test]
fn test_gen_from_seed() {
    let params: StartParams = (9, 4591, 286);
    let mut seed_f = NtruIntPoly::empty();
    let mut seed_g = NtruIntPoly::empty();

    seed_f.coeffs = vec![1, 2, 2, 0, 0, 1, 2, 2, 2];
    seed_f.n = seed_f.coeffs.len();

    seed_g.coeffs = vec![2, 0, 1, 1, 2, 0, 0, 1, 1];
    seed_g.n = seed_g.coeffs.len();

    let key_pair = NtruPrimeKeyPair::gen_from_seed(&params, seed_g, seed_f);

    assert!(key_pair.private.f.n == 9);
    assert!(key_pair.private.f.coeffs == [1, 2, 2, 0, 0, 1, 2, 2, 2]);

    assert!(key_pair.private.g_inv.n == 9);
    assert!(
        key_pair.private.g_inv.coeffs == [1381, 2493, 3083, 1045, 3427, 2565, 1249, 3648, 1274]
    );

    assert!(key_pair.public.h.n == 9);
    assert!(key_pair.public.h.coeffs == [3848, 1822, 557, 1204, 4198, 2245, 2292, 587, 1275]);
}
