use crate::encode::{r3, rq};
#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{rq_bytes, P, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{rq_bytes, P, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, RQ_BYTES, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, RQ_BYTES, W};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{rq_bytes, P, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{rq_bytes, P, W};

use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::random::NTRURandom;
use crate::{
    math::nums::weightw_mask,
    poly::{f3::round, r3::R3, rq::Rq},
};

use super::errors::NTRUErrors;

fn usize_vec_to_bytes(list: &[usize]) -> Vec<u8> {
    list.iter()
        .flat_map(|&x| x.to_ne_bytes().to_vec())
        .collect()
}

fn byte_to_usize_vec(list: &[u8]) -> Vec<usize> {
    list.chunks_exact(std::mem::size_of::<usize>())
        .map(|chunk| {
            let mut bytes = [0; std::mem::size_of::<usize>()];
            bytes.copy_from_slice(chunk);
            usize::from_ne_bytes(bytes)
        })
        .collect()
}

/// Decrypts a polynomial in the Fq field using a private key.
///
/// # Arguments
///
/// - `c`: The ciphertext polynomial to be decrypted.
/// - `priv_key`: The private key used for decryption.
///
/// # Returns
///
/// Returns the decrypted polynomial as a result of applying the private key to the ciphertext.
///
/// # Example
///
/// ```rust
/// #[cfg(feature = "ntrulpr1013")]
/// use ntrulp::params::params1013::P;
/// #[cfg(feature = "ntrulpr1277")]
/// use ntrulp::params::params1277::P;
/// #[cfg(feature = "ntrulpr653")]
/// use ntrulp::params::params653::P;
/// #[cfg(feature = "ntrulpr761")]
/// use ntrulp::params::params761::P;
/// #[cfg(feature = "ntrulpr857")]
/// use ntrulp::params::params857::P;
/// #[cfg(feature = "ntrulpr953")]
/// use ntrulp::params::params953::P;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::rq_decrypt;
/// use ntrulp::random::{CommonRandom, NTRURandom};
///
/// let mut random: NTRURandom = NTRURandom::new();
/// let f = Rq::from(random.short_random().unwrap());
/// let g = R3::from(random.random_small().unwrap());
///
/// // Generate the ciphertext polynomial c and the private key
/// let c = Rq::from(random.short_random().unwrap());
/// let priv_key = PrivKey::compute(&f, &g).unwrap();
///
/// // Decrypt the ciphertext polynomial
/// let decrypted_poly = rq_decrypt(&c, &priv_key);
/// ```
///
/// # Notes
///
/// This function decrypts a ciphertext polynomial `c` using a private key `priv_key`
/// and returns the decrypted polynomial.
///
pub fn rq_decrypt(c: &Rq, priv_key: &PrivKey) -> R3 {
    let f = &priv_key.0;
    let ginv = &priv_key.1;
    let mut r = [0i8; P];
    let cf = c.mult_r3(&f);
    let cf3 = cf.mult_int(3);
    let e = cf3.r3_from_rq();
    let ev = e.mult(&ginv);
    let mask: i16;

    mask = weightw_mask(&ev.coeffs); // 0 if weight w, else -1

    for i in 0..W {
        r[i] = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
    }

    for i in W..P {
        r[i] = (ev.coeffs[i] as i16 & !mask) as i8;
    }

    R3::from(r)
}

/// Encrypts a polynomial in the F3 field using a public key in the Fq field.
///
/// # Arguments
///
/// - `r`: The polynomial to be encrypted in the F3 field.
/// - `pub_key`: The public key used for encryption in the Fq field.
///
/// # Returns
///
/// Returns the ciphertext polynomial as a result of applying the public key to `r`.
///
/// # Example
///
/// ```rust
/// #[cfg(feature = "ntrulpr1013")]
/// use ntrulp::params::params1013::P;
/// #[cfg(feature = "ntrulpr1277")]
/// use ntrulp::params::params1277::P;
/// #[cfg(feature = "ntrulpr653")]
/// use ntrulp::params::params653::P;
/// #[cfg(feature = "ntrulpr761")]
/// use ntrulp::params::params761::P;
/// #[cfg(feature = "ntrulpr857")]
/// use ntrulp::params::params857::P;
/// #[cfg(feature = "ntrulpr953")]
/// use ntrulp::params::params953::P;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::rq_decrypt;
/// use ntrulp::ntru::cipher::r3_encrypt;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::random::{CommonRandom, NTRURandom};
///
/// let mut random: NTRURandom = NTRURandom::new();
/// let f = Rq::from(random.short_random().unwrap());
/// let g = R3::from(random.random_small().unwrap());
///
/// // Generate an content for encrypt
/// let r: R3 = Rq::from(random.short_random().unwrap()).r3_from_rq();
///
/// // Generate the private key priv_key
/// let priv_key = PrivKey::compute(&f, &g).unwrap();
/// let pub_key = PubKey::from_sk(&priv_key).unwrap();
///
/// let encrypted = r3_encrypt(&r, &pub_key);
/// let decrypted = rq_decrypt(&encrypted, &priv_key);
/// ```
///
/// # Notes
///
/// This function encrypts a polynomial `r` in the F3 field using a public key `pub_key`
/// in the Fq field and returns the ciphertext polynomial.
///
pub fn r3_encrypt(r: &R3, pub_key: &PubKey) -> Rq {
    let mut hr = pub_key.mult_r3(&r);

    round(&mut hr.coeffs);

    hr
}

pub fn bytes_encrypt(rng: &mut NTRURandom, bytes: &[u8], pub_key: &PubKey) -> Vec<u8> {
    let unlimted_poly = r3::r3_decode_chunks(bytes);
    let (chunks, size) = r3::r3_split_w_chunks(&unlimted_poly, rng);
    let mut bytes: Vec<u8> = Vec::with_capacity(P * size.len());

    for chunk in chunks {
        let r3 = R3::from(chunk);
        let hr = r3_encrypt(&r3, pub_key);
        let rq_bytes = rq::encode(&hr.coeffs);

        bytes.extend(rq_bytes);
    }

    let size_bytes = usize_vec_to_bytes(&size);
    let size_len = size_bytes.len().to_ne_bytes().to_vec();

    bytes.extend(size_bytes);
    bytes.extend(size_len);

    bytes
}

pub fn bytes_decrypt<'a>(bytes: &[u8], priv_key: &PrivKey) -> Result<Vec<u8>, NTRUErrors<'a>> {
    let bytes_len = bytes.len();
    let binding = bytes[bytes_len - 8..].try_into();
    let size_bytes_len: &[u8; 8] = match &binding {
        Ok(v) => v,
        Err(_) => return Err(NTRUErrors::SliceError("incorrect or damaged cipher bytes")),
    };
    let size_len = usize::from_ne_bytes(*size_bytes_len);
    let size_bytes = &bytes[bytes_len - size_len - 8..(bytes_len - 1)];
    let size = byte_to_usize_vec(size_bytes);
    let bytes_data = &bytes[..bytes_len - size_len - 8];
    let chunks = bytes_data.chunks(RQ_BYTES);

    let mut r3_chunks = Vec::new();

    for chunk in chunks {
        let rq_chunk: [u8; RQ_BYTES] = match chunk.try_into() {
            Ok(c) => c,
            Err(_) => {
                return Err(NTRUErrors::SliceError(
                    "Cannot into [u8; RQ_BYTES], Incorrect cipher!",
                ))
            }
        };
        let rq = Rq::from(rq::decode(&rq_chunk));
        let r3 = rq_decrypt(&rq, priv_key);

        r3_chunks.push(r3.coeffs);
    }

    let out_r3 = r3::r3_merge_w_chunks(&r3_chunks, &size);

    Ok(r3::r3_encode_chunks(&out_r3))
}

#[cfg(test)]
mod test_cipher {
    use super::*;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[test]
    fn test_bytes_cipher() {
        let mut random: NTRURandom = NTRURandom::new();

        let mut g: R3;
        let ciphertext = random.randombytes::<1024>();
        let f: Rq = Rq::from(random.short_random().unwrap());
        let sk = loop {
            g = R3::from(random.random_small().unwrap());

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();
        let encrypted = bytes_encrypt(&mut random, &ciphertext, &pk);
        let decrypted = bytes_decrypt(&encrypted, &sk).unwrap();

        assert_eq!(decrypted, ciphertext);
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let mut random: NTRURandom = NTRURandom::new();

        let r: R3 = Rq::from(random.short_random().unwrap()).r3_from_rq();
        let f: Rq = Rq::from(random.short_random().unwrap());
        let mut g: R3;
        let sk = loop {
            g = R3::from(random.random_small().unwrap());

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();
        let encrypted = r3_encrypt(&r, &pk);
        let decrypted = rq_decrypt(&encrypted, &sk);

        assert_eq!(decrypted.coeffs, r.coeffs);
    }
}
