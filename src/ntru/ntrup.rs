use super::{errors::NTRUErrors, params::check_params};
use crate::{
    encode::{r3, rq},
    kem::{f3::round, r3::R3, rq::Rq},
    key::pair::KeyPair,
    math::nums::weightw_mask,
    random::{CommonRandom, NTRURandom},
};
use sha2::{Digest, Sha512};

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

    pub fn encap(&mut self, pk: &[u8]) -> (Vec<u8>, [u8; 32]) {
        // TODO: 1158=this is lenght of pub_key as bytes.
        let r: R3<P, Q, Q12> = Rq::from(self.rng.short_random(W).unwrap()).r3_from_rq();
        let cache: [u8; 32] = self.hash_prefix(4, pk, 1158);
        let (c, r_enc) = self.hide(&r, &cache);
        let gamma = self.hash_session(1, &r_enc, &c);

        (c, gamma)
    }

    pub fn encrypt(&self, r: &R3<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let h = &self.key_pair.pub_key.h;
        let mut hr = h.mult_small(&r);

        round(&mut hr.coeffs);

        Rq::from(hr.coeffs)
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

    fn hash_prefix(&self, b: u8, input: &[u8], length: usize) -> [u8; 32] {
        let ext_len = length + 1;
        let mut out = [0u8; 32];
        let mut x = vec![0u8; ext_len];

        x[0] = b;

        for i in 0..length {
            x[i + 1] = match input.get(i) {
                Some(&v) => v,
                None => continue,
            };
        }

        let mut hasher = Sha512::new();
        hasher.update(&x[..ext_len]);
        let hash_result = hasher.finalize();

        out.copy_from_slice(&hash_result[..32]);

        out
    }

    fn hash_confirm(&self, r_enc: &[u8], cache: &[u8]) -> [u8; 32] {
        let mut x = [0u8; 64];

        x[..32].copy_from_slice(&self.hash_prefix(3, &r_enc, r_enc.len() + 1));

        for i in 0..32 {
            x[32 + i] = cache[i];
        }

        self.hash_prefix(2, &x, x.len())
    }

    fn hide(&self, r: &R3<P, Q, Q12>, cache: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // TODO: 32 is a half of the length sha512
        // TODO: r_enc is [u8; P + 3 / 4]
        let r_enc = r3::r3_encode(&r.coeffs);
        let rq = self.encrypt(&r);
        let mut c = vec![0u8; 1007 + 32];

        c[32..].copy_from_slice(rq::rq_rounded_encode::<P, Q, Q12>(&rq.coeffs).as_slice());

        let gamma = self.hash_confirm(&r_enc, cache);

        // TODO: Rounded_bytes=1007 for p=761
        c[..32].copy_from_slice(&gamma);

        (c, r_enc)
    }

    fn hash_session(&self, b: u8, y: &[u8], z: &[u8]) -> [u8; 32] {
        // TODO: 191 is Inputs_bytes
        let mut x = self.hash_prefix(3, y, 191);

        // TODO: Rounded_bytes=1007 + Confirm_bytes=32
        for i in 0..1007 + 32 {
            x[32 + i] = z[i];
        }

        self.hash_prefix(b, &x, x.len())
    }

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

        for _ in 0..2 {
            let mut rng: NTRURandom<P> = NTRURandom::new();
            let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

            let encrypted = ntrup.encrypt(&c);
            let decrypted = ntrup.decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        }
    }
}
