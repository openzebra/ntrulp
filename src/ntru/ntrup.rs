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

fn rq_decrypt<const P: usize, const Q: usize, const W: usize, const Q12: usize>(
    c: &Rq<P, Q, Q12>,
    f: &Rq<P, Q, Q12>,
    ginv: &R3<P, Q, Q12>,
) -> R3<P, Q, Q12> {
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

pub struct NTRUPrime<
    const P: usize,
    const Q: usize,
    const W: usize,
    const Q12: usize,
    const ROUNDED_BYTES: usize,
    const RQ_BYTES: usize,
    const P_PLUS_ONE: usize,
> {
    pub key_pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE>,
    pub num_threads: usize,
}

impl<
        const P: usize,
        const Q: usize,
        const W: usize,
        const Q12: usize,
        const ROUNDED_BYTES: usize,
        const RQ_BYTES: usize,
        const P_PLUS_ONE: usize,
    > NTRUPrime<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>
{
    pub fn new() -> Result<Self, NTRUErrors<'static>> {
        check_params::<P, Q, W, Q12, P_PLUS_ONE>()?;

        let key_pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE> = KeyPair::new();
        let num_threads = num_cpus::get();

        Ok(NTRUPrime {
            key_pair,
            num_threads,
        })
    }

    // return bytes where
    // content = [content, chunk_size, 8_bytes_usize_len_chunks_size_bytes]
    pub fn encrypt(&self, bytes: &[u8], pk: &[u8]) -> Vec<u8> {
        let unlimted_poly = r3::r3_decode_chunks(bytes);
        let pub_key_coeffs = rq::rq_decode::<P, Q, Q12, RQ_BYTES>(pk);
        let h: Arc<Rq<P, Q, Q12>> = Arc::new(Rq::from(pub_key_coeffs));
        let (chunks, size) = r3::r3_split_w_chunks::<P, W>(&unlimted_poly);
        let enc: Arc<Mutex<HashMap<usize, [u8; ROUNDED_BYTES]>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mut threads = Vec::with_capacity(self.num_threads);

        for (index, chunk) in chunks.into_iter().enumerate() {
            let h_ref = Arc::clone(&h);
            let enc_ref = Arc::clone(&enc);
            let handle = thread::spawn(move || {
                let r3: R3<P, Q, Q12> = R3::from(chunk);
                let mut hr = h_ref.mult_small(&r3);

                round(&mut hr.coeffs);

                let rq_bytes = rq::rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&hr.coeffs);
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
        let size_bytes = self.usize_vec_to_bytes(&size);
        let size_len = size_bytes.len().to_ne_bytes().to_vec();
        let mut bytes: Vec<u8> = Vec::with_capacity(P * size.len());

        for i in 0..size.len() {
            match enc_ref.get(&i) {
                Some(v) => bytes.extend(v),
                None => panic!("cannot find from enc"), // TODO: add error handler, remove all
                                                        // unwrap!
            }
        }

        bytes.extend(size_bytes);
        bytes.extend(size_len);

        bytes
    }

    pub fn decrypt(&self, bytes: Vec<u8>) -> Vec<u8> {
        let bytes_len = bytes.len();
        let size_bytes_len: &[u8; 8] = &bytes[bytes_len - 8..].try_into().unwrap(); // TODO: remove unwrap!
        let size_len = usize::from_ne_bytes(*size_bytes_len);
        let size_bytes = &bytes[bytes_len - size_len - 8..(bytes_len - 1)];
        let size = self.byte_to_usize_vec(size_bytes);
        let bytes_data = &bytes[..bytes_len - size_len - 8];
        let chunks = bytes_data.chunks(ROUNDED_BYTES);

        let f = Arc::new(self.key_pair.priv_key.f.coeffs);
        let ginv = Arc::new(self.key_pair.priv_key.ginv.coeffs);

        let sync_hash_map: Arc<Mutex<HashMap<usize, [i8; P]>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mut threads = Vec::with_capacity(self.num_threads);

        for (index, chunk) in chunks.into_iter().enumerate() {
            let sync_map_ref = Arc::clone(&sync_hash_map);
            // TODO: Remove unwrap!
            let rounded_chunk: [u8; ROUNDED_BYTES] = chunk.try_into().unwrap();
            let f_ref = Arc::clone(&f);
            let ginv_ref = Arc::clone(&ginv);
            let handle = thread::spawn(move || {
                let rq_coeffs = rq::rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&rounded_chunk);
                let rq: Rq<P, Q, Q12> = Rq::from(rq_coeffs);
                let f: Rq<P, Q, Q12> = Rq::from(*f_ref);
                let ginv: R3<P, Q, Q12> = R3::from(*ginv_ref);
                let r3 = rq_decrypt::<P, Q, W, Q12>(&rq, &f, &ginv);

                let mut sync_map = sync_map_ref.lock().unwrap();

                sync_map.insert(index, r3.coeffs);
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

        // TODO: Remove unwrap.
        let sync_map = sync_hash_map.lock().unwrap();
        let mut r3_chunks = Vec::new();
        for i in 0..size.len() {
            match sync_map.get(&i) {
                Some(v) => r3_chunks.push(*v),
                None => panic!("cannot find from enc"), // TODO: add error handler, remove all
                                                        // unwrap!
            }
        }

        let out_r3 = r3::r3_merge_w_chunks::<P>(&r3_chunks, &size);

        r3::r3_encode_chunks(&out_r3)
    }

    pub fn r3_encrypt(&self, r: &R3<P, Q, Q12>, h: &Rq<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let mut hr = h.mult_small(&r);

        round(&mut hr.coeffs);

        Rq::from(hr.coeffs)
    }

    pub fn rq_decrypt(&self, c: &Rq<P, Q, Q12>) -> R3<P, Q, Q12> {
        let f = &self.key_pair.priv_key.f;
        let ginv = &self.key_pair.priv_key.ginv;

        rq_decrypt::<P, Q, W, Q12>(c, f, ginv)
    }

    pub fn key_pair_gen(&mut self, rng: ThreadRng) -> Result<(), NTRUErrors> {
        const MAX_TRY: usize = 100;

        let mut ntru_rng: NTRURandom<P> = NTRURandom::from(rng);
        let mut k: usize = 0;

        loop {
            if k >= MAX_TRY {
                return Err(NTRUErrors::KeyGenError(
                    "faild to generate key_pari check RNG",
                ));
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

    pub fn set_key_pair(&mut self, key_pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE>) {
        self.key_pair = key_pair;
    }

    fn usize_vec_to_bytes(&self, list: &[usize]) -> Vec<u8> {
        list.iter()
            .flat_map(|&x| x.to_ne_bytes().to_vec())
            .collect()
    }

    fn byte_to_usize_vec(&self, list: &[u8]) -> Vec<usize> {
        list.chunks_exact(std::mem::size_of::<usize>())
            .map(|chunk| {
                let mut bytes = [0; std::mem::size_of::<usize>()];
                bytes.copy_from_slice(chunk);
                usize::from_ne_bytes(bytes)
            })
            .collect()
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
        NTRUPrime::<761, 4591, 286, 4590, 1007, 1158, 762>::new().unwrap();
        NTRUPrime::<857, 5167, 322, 5166, 1152, 1322, 858>::new().unwrap();
        NTRUPrime::<653, 4621, 288, 4620, 865, 994, 654>::new().unwrap();
        NTRUPrime::<953, 6343, 396, 6342, 1317, 1505, 954>::new().unwrap();
        NTRUPrime::<1013, 7177, 448, 7176, 1423, 1623, 1014>::new().unwrap();
        NTRUPrime::<1277, 7879, 492, 7878, 1815, 2067, 1278>::new().unwrap();
    }

    #[test]
    fn test_gen_key_pair() {
        let mut ntrup = NTRUPrime::<761, 4591, 286, 4590, 1007, 1158, 762>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<857, 5167, 322, 5166, 1152, 1322, 858>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<653, 4621, 288, 4620, 865, 994, 654>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<953, 6343, 396, 6342, 1317, 1505, 954>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1013, 7177, 448, 7176, 1423, 1623, 1014>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());

        let mut ntrup = NTRUPrime::<1277, 7879, 492, 7878, 1815, 2067, 1278>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        assert!(ntrup.key_pair.verify());
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_761() {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1158;
        const ROUNDED_BYTES: usize = 1007;
        const P_PLUS_ONE: usize = P + 1;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_857() {
        const P: usize = 857;
        const W: usize = 322;
        const Q: usize = 5167;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1322;
        const ROUNDED_BYTES: usize = 1152;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_653() {
        const P: usize = 653;
        const Q: usize = 4621;
        const W: usize = 288;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 994;
        const ROUNDED_BYTES: usize = 865;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_953() {
        const P: usize = 953;
        const Q: usize = 6343;
        const W: usize = 396;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1505;
        const ROUNDED_BYTES: usize = 1317;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_1013() {
        const P: usize = 1013;
        const Q: usize = 7177;
        const W: usize = 448;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1623;
        const ROUNDED_BYTES: usize = 1423;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_decrpt_encrypt_r3_to_rq_1277() {
        const P: usize = 1277;
        const Q: usize = 7879;
        const W: usize = 492;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 2067;
        const ROUNDED_BYTES: usize = 1815;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

        let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
        let decrypted = ntrup.rq_decrypt(&encrypted);

        assert_eq!(decrypted.coeffs, c.coeffs);
    }

    #[test]
    fn test_uszie_convert() {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1158;
        const ROUNDED_BYTES: usize = 1007;

        let mut rng = rand::thread_rng();
        let ntrup = NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let usize_list: Vec<usize> = (0..1000).map(|_| rng.gen::<usize>()).collect();
        let bytes = ntrup.usize_vec_to_bytes(&usize_list);
        let out = ntrup.byte_to_usize_vec(&bytes);

        assert_eq!(out, usize_list);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_761() {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1158;
        const ROUNDED_BYTES: usize = 1007;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_857() {
        const P: usize = 857;
        const W: usize = 322;
        const Q: usize = 5167;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1322;
        const ROUNDED_BYTES: usize = 1152;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_653() {
        const P: usize = 653;
        const Q: usize = 4621;
        const W: usize = 288;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 994;
        const ROUNDED_BYTES: usize = 865;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_953() {
        const P: usize = 953;
        const Q: usize = 6343;
        const W: usize = 396;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1505;
        const ROUNDED_BYTES: usize = 1317;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_1013() {
        const P: usize = 1013;
        const Q: usize = 7177;
        const W: usize = 448;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1623;
        const ROUNDED_BYTES: usize = 1423;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }

    #[test]
    fn test_decrpt_encrypt_bytes_1277() {
        const P: usize = 1277;
        const Q: usize = 7879;
        const W: usize = 492;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 2067;
        const ROUNDED_BYTES: usize = 1815;

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE>::new().unwrap();
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        let encrypted = ntrup.encrypt(&bytes, &pk);
        let decrypted = ntrup.decrypt(encrypted);

        assert_eq!(decrypted, bytes);
    }
}
