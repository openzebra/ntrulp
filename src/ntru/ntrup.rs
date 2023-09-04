use super::{errors::NTRUErrors, params::check_params};
use crate::{
    encode::r3,
    kem::{f3::round, r3::R3, rq::Rq},
    key::pair::KeyPair,
    math::nums::weightw_mask,
    random::{CommonRandom, NTRURandom},
};

pub const PK_SIZE: usize = 1218; // Public Key
pub const SK_SIZE: usize = 1600; // Private/Secret Key
pub const CT_SIZE: usize = 1047; // Cipher Text
pub const K_SIZE: usize = 32; // Shared Key

pub struct NTRUPrime<const P: usize, const Q: usize, const W: usize, const Q12: usize> {
    pub key_pair: KeyPair<P, Q, Q12>,
    pub rng: NTRURandom<P>,
}

impl<const P: usize, const Q: usize, const W: usize, const Q12: usize> NTRUPrime<P, Q, W, Q12> {
    pub fn new() -> Result<Self, NTRUErrors> {
        check_params::<P, Q, W, Q12>()?;

        let rng: NTRURandom<P> = NTRURandom::new();
        let key_pair: KeyPair<P, Q, Q12> = KeyPair::new();

        Ok(NTRUPrime { rng, key_pair })
    }

    pub fn encrypt(&self, r: &R3<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let h = &self.key_pair.pub_key.h;
        let hr = h.mult_small(&r);
        let hr_rounded = round(&hr.coeffs);

        Rq::from(hr_rounded)
    }

    pub fn decrypt(&self, c: &Rq<P, Q, Q12>) -> R3<P, Q, Q12> {
        let f = &self.key_pair.priv_key.f;
        let ginv = &self.key_pair.priv_key.ginv;
        let mut r = [0i8; P];
        let cf: Rq<P, Q, Q12> = c.mult_small(&f.r3_from_rq());
        let cf3: Rq<P, Q, Q12> = cf.mult3();
        let e: R3<P, Q, Q12> = cf3.r3_from_rq();
        let ev: R3<P, Q, Q12> = e.mult(&ginv);
        #[allow(unused_assignments)]
        let mut mask: i16 = 0;

        mask = weightw_mask::<P, W>(&ev.coeffs); // 0 if weight w, else -1

        for i in 0..W {
            r[i] = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
        }

        for i in W..P {
            r[i] = (ev.coeffs[i] as i16 & !mask) as i8;
        }

        R3::from(r)
    }

    pub fn key_pair_gen(&mut self) -> Result<(), NTRUErrors> {
        const MAX_TRY: usize = 100;

        let mut k: usize = 0;

        loop {
            if k >= MAX_TRY {
                return Err(NTRUErrors::KeyPairGen);
            }

            let short_entropy = match self.rng.short_random(W) {
                Ok(s) => s,
                Err(_) => {
                    k += 1;
                    continue;
                }
            };
            let small_entropy = match self.rng.random_small() {
                Ok(s) => s,
                Err(_) => {
                    k += 1;
                    continue;
                }
            };
            let f: Rq<P, Q, Q12> = Rq::from(short_entropy);
            let g: R3<P, Q, Q12> = R3::from(small_entropy);

            match self.key_pair.from_seed(g, f) {
                Ok(_) => self.key_pair.verify(),
                Err(_) => {
                    k += 1;
                    continue;
                }
            };

            break;
        }

        Ok(())
    }

    // pub fn decapsulate(
    //     &self,
    //     cstr: [u8; CT_SIZE],
    //     sk: [u8; SK_SIZE],
    // ) -> Result<[u8; K_SIZE], bool> {
    //     let f = self.key_pair.priv_key.f;
    //     let c = rq::encoding::decode_rounded(&cstr[32..]);
    //
    //     let f = zx::encoding::decode(&sk[..191]);
    //     let c = rq::encoding::decode_rounded(&cstr[32..]);
    //     let mut t = [0i16; P];
    //     rq::mult(&mut t, c, f);
    //     let mut t3 = [0i8; P];
    //     for i in 0..P {
    //         t3[i] = r3::mod3::freeze(rq::modq::freeze(3 * t[i] as i32) as i32);
    //     }
    //     let gr = zx::encoding::decode(&sk[191..]);
    //     let mut r = [0i8; P];
    //     r3::mult(&mut r, t3, gr);
    //     let w = count_zeroes(r);
    //     let mut check = w == 286;
    //     let h = rq::encoding::decode(&sk[(2 * 191)..]);
    //     let mut hr = [0i16; P];
    //     rq::mult(&mut hr, h, r);
    //     rq::round3(&mut hr);
    //     for i in 0..P {
    //         check &= (hr[i] - c[i]) == 0;
    //     }
    //     let s = Sha512::digest(&zx::encoding::encode(r));
    //     check &= s[..32] == cstr[..32];
    //     let mut k = [0u8; 32];
    //     k.copy_from_slice(&s[32..]);
    //     if check {
    //         Ok(k)
    //     } else {
    //         Err(false)
    //     }
    // }

    pub fn set_key_pair(&mut self, key_pair: KeyPair<P, Q, Q12>) {
        self.key_pair = key_pair;
    }
}

#[cfg(test)]
mod tests {
    use super::NTRUPrime;
    use crate::{
        kem::{r3::R3, rq::Rq},
        random::{CommonRandom, NTRURandom},
    };

    #[test]
    fn test_init_params() {
        NTRUPrime::<761, 4591, 286, 4590>::new().unwrap();
        NTRUPrime::<857, 5167, 322, 5166>::new().unwrap();
        NTRUPrime::<653, 4621, 288, 4620>::new().unwrap();
        NTRUPrime::<953, 6343, 396, 6342>::new().unwrap();
        NTRUPrime::<1013, 7177, 448, 7176>::new().unwrap();
        NTRUPrime::<1277, 7879, 492, 7878>::new().unwrap();
    }

    #[test]
    fn test_gen_key_pair() {
        let mut ntrup = NTRUPrime::<761, 4591, 286, 4590>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<857, 5167, 322, 5166>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<653, 4621, 288, 4620>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<953, 6343, 396, 6342>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1013, 7177, 448, 7176>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1277, 7879, 492, 7878>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        assert!(ntrup.key_pair.verify());
    }

    #[test]
    fn test_decrpt_encrypt() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;

        let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();

        ntrup.key_pair_gen().unwrap();

        for _ in 0..10 {
            let mut rng: NTRURandom<P> = NTRURandom::new();
            let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
            let encrypted = ntrup.encrypt(&c);
            let decrypted = ntrup.decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        }
    }
}
