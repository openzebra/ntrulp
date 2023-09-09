extern crate num_cpus;

use rand::rngs::ThreadRng;

use super::{errors::NTRUErrors, params::check_params};
use crate::{
    encode::{r3, rq},
    kem::{f3::round, r3::R3, rq::Rq},
    key::pair::KeyPair,
    math::nums::weightw_mask,
    random::{CommonRandom, NTRURandom},
};
use std::thread::{self};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub const PK_SIZE: usize = 1218; // Public Key
pub const SK_SIZE: usize = 1600; // Private/Secret Key
pub const CT_SIZE: usize = 1047; // Cipher Text
pub const K_SIZE: usize = 32; // Shared Key

pub struct NTRUPrime<const P: usize, const Q: usize, const W: usize, const Q12: usize> {
    pub key_pair: KeyPair<P, Q, Q12>,
    pub num_threads: usize,
}

impl<const P: usize, const Q: usize, const W: usize, const Q12: usize> NTRUPrime<P, Q, W, Q12> {
    pub fn new() -> Result<Self, NTRUErrors> {
        check_params::<P, Q, W, Q12>()?;

        let key_pair: KeyPair<P, Q, Q12> = KeyPair::new();
        let num_threads = num_cpus::get();

        Ok(NTRUPrime {
            key_pair,
            num_threads,
        })
    }

    pub fn encrypt(&self, bytes: &[u8], pk: &[u8]) -> Vec<u8> {
        let unlimted_poly = r3::r3_decode_chunks(bytes);
        let pub_key_coeffs = rq::rq_decode::<P, Q, Q12>(pk);
        let h: Arc<Rq<P, Q, Q12>> = Arc::new(Rq::from(pub_key_coeffs));
        let (chunks, size) = r3::r3_split_w_chunks::<P, W>(&unlimted_poly);
        let enc: Arc<Mutex<HashMap<usize, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));
        let mut threads = Vec::with_capacity(self.num_threads);

        for (index, chunk) in chunks.into_iter().enumerate() {
            let h_ref = Arc::clone(&h);
            let enc_ref = Arc::clone(&enc);
            let handle = thread::spawn(move || {
                let r3: R3<P, Q, Q12> = R3::from(chunk);
                let mut hr = h_ref.mult_small(&r3);

                round(&mut hr.coeffs);

                let rq_bytes = rq::rq_encode::<P, Q, Q12>(&hr.coeffs);
                let mut enc = enc_ref.lock().unwrap();

                enc.insert(index, rq_bytes);
            });

            threads.push(handle);

            if threads.len() >= self.num_threads {
                let handle = threads.remove(0);

                handle.join().unwrap();
            }
        }

        for h in threads {
            h.join().unwrap();
        }

        let enc_ref = enc.lock().unwrap();
        let mut bytes: Vec<u8> = Vec::with_capacity(P * size.len());

        for i in 0..size.len() {
            match enc_ref.get(&i) {
                Some(v) => bytes.extend(v),
                None => panic!("cannot find from enc"),
            }
        }

        bytes
    }

    pub fn decrypt(&self, bytes: &[u8]) {}

    pub fn r3_encrypt(&self, r: &R3<P, Q, Q12>, h: &Rq<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let mut hr = h.mult_small(&r);

        round(&mut hr.coeffs);

        Rq::from(hr.coeffs)
    }

    pub fn rq_decrypt(&self, c: &Rq<P, Q, Q12>) -> R3<P, Q, Q12> {
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

    pub fn key_pair_gen(&mut self, rng: ThreadRng) -> Result<(), NTRUErrors> {
        const MAX_TRY: usize = 100;

        let mut ntru_rng: NTRURandom<P> = NTRURandom::from(rng);
        let mut k: usize = 0;

        loop {
            if k >= MAX_TRY {
                return Err(NTRUErrors::KeyPairGen);
            }

            let short_entropy = match ntru_rng.short_random(W) {
                Ok(s) => s,
                Err(_) => {
                    k += 1;
                    continue;
                }
            };
            let small_entropy = match ntru_rng.random_small() {
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
    use rand::Rng;

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

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<857, 5167, 322, 5166>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<653, 4621, 288, 4620>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<953, 6343, 396, 6342>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1013, 7177, 448, 7176>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1277, 7879, 492, 7878>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;

        let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_bytes() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;

        let mut rng = rand::thread_rng();
        let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();
        let bytes: Vec<u8> = (0..1000).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, sk) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);

        println!("{:?}", encrypted);
    }
}
