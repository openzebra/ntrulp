use rand::RngCore;

use crate::params::params::{P, R3_BYTES, RQ_BYTES, W};

use crate::compress::compress;
use crate::encode::rq;
use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::poly::r3::R3;

use super::cipher::r3_encrypt;

pub fn bytes_encrypt<R: RngCore>(rng: &mut R, bytes: &[u8], pub_key: &PubKey) -> Vec<u8> {
    let unlimted_poly = compress::r3_decode_chunks(bytes);
    let (chunks, size, seed) = compress::r3_split_w_chunks(&unlimted_poly, rng);
    let mut bytes: Vec<u8> = Vec::with_capacity(P * size.len());

    for chunk in chunks {
        let r3: R3 = chunk.into();
        let hr = r3_encrypt(&r3, pub_key);
        let rq_bytes = rq::encode(&hr.coeffs);

        bytes.extend(rq_bytes);
    }

    compress::pack_bytes(bytes, size, seed)
}
