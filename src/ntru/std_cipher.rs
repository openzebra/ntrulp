use rand::RngCore;

use crate::params::params::{P, R3_BYTES, RQ_BYTES, W};

use crate::compress::compress;
use crate::encode::{r3, rq};
use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::poly::r3::R3;
use crate::poly::rq::Rq;

use super::cipher::{r3_encrypt, rq_decrypt};
use super::std_error::CipherError;

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

pub fn bytes_decrypt(bytes: &[u8], priv_key: &PrivKey) -> Result<Vec<u8>, CipherError> {
    let (bytes_data, size, seed) =
        compress::unpack_bytes(bytes).map_err(CipherError::CompressError)?;
    let chunks = bytes_data.chunks(RQ_BYTES);
    let size_len = size.len();

    let mut r3_chunks = Vec::with_capacity(size_len);

    for chunk in chunks {
        let rq_chunk: [u8; RQ_BYTES] = chunk.try_into().or(Err(CipherError::InvalidRqChunkSize))?;
        let rq: Rq = rq_chunk.into();
        let r3 = rq_decrypt(&rq, priv_key);

        r3_chunks.push(r3.coeffs);
    }

    let out_r3 = compress::r3_merge_w_chunks(&r3_chunks, &size, seed);

    Ok(compress::r3_encode_chunks(&out_r3))
}
