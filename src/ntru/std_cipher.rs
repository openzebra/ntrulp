use rand::RngCore;

use crate::params::params::{P, RQ_BYTES};

use crate::compress::compress;
use crate::encode::rq;
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

#[cfg(test)]
mod test_cipher_std {
    use crate::rng::{random_small, short_random};

    use super::*;

    #[test]
    fn test_bytes_cipher() {
        let mut rng = rand::thread_rng();
        let mut g: R3;
        let mut ciphertext = vec![0u8; 1024];
        rng.fill_bytes(&mut ciphertext);
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        let sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();
        let mut encrypted = bytes_encrypt(&mut rng, &ciphertext, &pk);
        let encrypted1 = bytes_encrypt(&mut rng, &ciphertext, &pk);
        assert_ne!(encrypted, encrypted1);
        let decrypted = bytes_decrypt(&encrypted, &sk).unwrap();

        assert_eq!(decrypted, ciphertext.to_vec());

        encrypted[2] = 0;
        encrypted[1] = 0;
        encrypted[3] = 0;
        encrypted[4] = 0;
        encrypted[5] = 0;
        encrypted[6] = 0;

        let decrypted = bytes_decrypt(&encrypted, &sk).unwrap();

        assert_ne!(decrypted, ciphertext.to_vec());
    }

    #[test]
    fn test_invalid_keys() {
        let mut rng = rand::thread_rng();
        let mut g: R3;
        let mut ciphertext = vec![0u8; 1024];
        rng.fill_bytes(&mut ciphertext);
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(_) => break,
                Err(_) => continue,
            };
        }
        let pk = PubKey::compute(&f, &g).unwrap();
        let invalid_sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let encrypted = bytes_encrypt(&mut rng, &ciphertext, &pk);
        let decrypted = bytes_decrypt(&encrypted, &invalid_sk).unwrap();

        assert_ne!(decrypted, ciphertext);
    }

    #[test]
    fn test_invalid_bytes_decrypt() {
        let mut rng = rand::thread_rng();
        let mut g: R3;
        let mut invalid_bytes = vec![0u8; 128];
        rng.fill_bytes(&mut invalid_bytes);
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(_) => break,
                Err(_) => continue,
            };
        }
        let invalid_sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };

        assert!(bytes_decrypt(&invalid_bytes, &invalid_sk).is_err());
    }
}
