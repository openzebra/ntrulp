use rand::RngCore;
use std::sync::Arc;
use std::thread;

use crate::params::params::RQ_BYTES;

use crate::compress;
use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::params::params::P;
use crate::poly::r3::R3;
use crate::poly::rq::Rq;

use super::cipher::{r3_encrypt, rq_decrypt};
use super::std_error::CipherError;

/// Encrypts bytes in parallel using multiple processor threads and the provided `NTRURandom` instance and recipient's public key.
///
/// # Arguments
///
/// * `rng`: An instance of `Rng` used for encryption.
/// * `bytes`: Bytes to be encrypted.
/// * `pub_key`: The public key of the recipient.
///
/// # Returns
///
/// Returns the encrypted bytes.
///
/// # Example
///
/// ```rust
/// use ntrulp::ntru::std_cipher::bytes_decrypt;
/// use ntrulp::ntru::std_cipher::bytes_encrypt;
/// use rand::SeedableRng;
/// use rand::RngCore;
/// use rand_chacha::ChaCha20Rng;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::rng::{random_small, short_random};
///
///
/// let mut rng = ChaCha20Rng::from_entropy();
/// let mut g: R3;
/// let mut ciphertext = vec![0u8; 1024];
/// rng.fill_bytes(&mut ciphertext);
///
/// let f: Rq = Rq::from(short_random(&mut rng).unwrap());
/// let sk = loop {
///     g = R3::from(random_small(&mut rng));
///
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
/// let pk = PubKey::compute(&f, &g).unwrap();
/// let mut encrypted = bytes_encrypt(&mut rng, &ciphertext, pk.clone()).unwrap();
/// let decrypted = bytes_decrypt(&encrypted, sk.clone()).unwrap();
///
/// assert_eq!(decrypted, ciphertext.to_vec());
/// ```
///
/// # Panics
///
/// The function may panic if encryption fails, the provided public key is invalid,
/// or if the specified number of threads exceeds the available processor cores.
///
pub fn bytes_encrypt<R: RngCore>(
    rng: &mut R,
    plaintext: &[u8],
    pub_key: PubKey,
) -> Result<Vec<u8>, CipherError> {
    let unlimted_poly = compress::r3::r3_decode_chunks(plaintext);
    let (chunks, size, seed) = compress::r3::r3_split_w_chunks(&unlimted_poly, rng);

    let chunk_count = chunks.len();
    let thread_count = std::cmp::min(chunk_count, num_cpus::get());
    let chunks_per_thread = (chunk_count + thread_count - 1) / thread_count;

    let pub_key = Arc::new(pub_key);
    let results: Vec<_> = chunks
        .chunks(chunks_per_thread)
        .enumerate()
        .map(|(thread_index, chunk_slice)| {
            let chunk_slice = chunk_slice.to_vec();
            let pub_key = Arc::clone(&pub_key);

            thread::spawn(move || -> Result<(usize, Vec<u8>), CipherError> {
                let mut thread_results = Vec::with_capacity(chunk_slice.len() * RQ_BYTES);
                for chunk in chunk_slice {
                    let r3: R3 = chunk.into();
                    let hr = r3_encrypt(&r3, &pub_key);
                    let rq_bytes = hr.to_bytes();
                    thread_results.extend_from_slice(&rq_bytes);
                }
                Ok((thread_index, thread_results))
            })
        })
        .collect();

    let mut ordered_results = vec![Vec::new(); results.len()];

    for handle in results {
        let (index, result) = handle.join().or(Err(CipherError::SyncThreadJoinError))??;
        ordered_results[index] = result;
    }

    let bytes: Vec<u8> = ordered_results.into_iter().flatten().collect();

    Ok(compress::r3::pack_bytes(bytes, size, seed))
}

/// Decrypts previously encrypted bytes in parallel using multiple processor threads.
///
/// # Arguments
///
/// * `bytes`: A reference to an `Arc<Vec<u8>>` containing the bytes to be decrypted.
/// * `priv_key`: A reference to an `Arc<PrivKey>` representing the private key for decryption.
///
/// # Returns
///
/// Returns the decrypted bytes.
///
/// # Example
///
/// ```rust
/// use ntrulp::ntru::std_cipher::bytes_decrypt;
/// use ntrulp::ntru::std_cipher::bytes_encrypt;
/// use rand::SeedableRng;
/// use rand::RngCore;
/// use rand_chacha::ChaCha20Rng;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::rng::{random_small, short_random};
///
///
/// let mut rng = ChaCha20Rng::from_entropy();
/// let mut g: R3;
/// let mut ciphertext = vec![0u8; 1024];
/// rng.fill_bytes(&mut ciphertext);
///
/// let f: Rq = Rq::from(short_random(&mut rng).unwrap());
/// let sk = loop {
///     g = R3::from(random_small(&mut rng));
///
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
/// let pk = PubKey::compute(&f, &g).unwrap();
/// let mut encrypted = bytes_encrypt(&mut rng, &ciphertext, pk.clone()).unwrap();
/// let decrypted = bytes_decrypt(&encrypted, sk.clone()).unwrap();
///
/// assert_eq!(decrypted, ciphertext.to_vec());

/// ```
///
/// # Panics
///
/// The function may panic if decryption fails or if the specified number of threads exceeds the available processor cores.
///
pub fn bytes_decrypt(cipher: &[u8], priv_key: PrivKey) -> Result<Vec<u8>, CipherError> {
    let (bytes_data, size, seed) =
        compress::r3::unpack_bytes(cipher).map_err(CipherError::CompressError)?;
    let chunk_count = bytes_data.len() / RQ_BYTES;
    let thread_count = std::cmp::min(chunk_count, num_cpus::get());
    let chunks_per_thread = (chunk_count + thread_count - 1) / thread_count;

    let priv_key = Arc::new(priv_key);
    let results: Vec<_> = bytes_data
        .chunks(chunks_per_thread * RQ_BYTES)
        .enumerate()
        .map(|(thread_index, chunk_slice)| {
            let chunk_slice = chunk_slice.to_vec();
            let priv_key = Arc::clone(&priv_key);

            thread::spawn(move || -> Result<(usize, Vec<[i8; P]>), CipherError> {
                let mut thread_results = Vec::with_capacity(chunk_slice.len() / RQ_BYTES);
                for chunk in chunk_slice.chunks(RQ_BYTES) {
                    let rq_chunk: [u8; RQ_BYTES] =
                        chunk.try_into().or(Err(CipherError::InvalidRqChunkSize))?;
                    let rq: Rq = rq_chunk.into();
                    let r3 = rq_decrypt(&rq, &priv_key);
                    thread_results.push(r3.coeffs);
                }
                Ok((thread_index, thread_results))
            })
        })
        .collect();

    let mut ordered_results = vec![Vec::new(); results.len()];
    for handle in results {
        let (index, result) = handle
            .join()
            .map_err(|_| CipherError::SyncThreadJoinError)??;
        ordered_results[index] = result;
    }

    let r3_chunks: Vec<[i8; P]> = ordered_results.into_iter().flatten().collect();
    let out_r3 = compress::r3::r3_merge_w_chunks(&r3_chunks, &size, seed);

    Ok(compress::r3::r3_encode_chunks(&out_r3))
}

#[cfg(test)]
mod test_cipher_std {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use crate::rng::{random_small, short_random};

    use super::*;

    #[test]
    fn test_bytes_cipher() {
        let mut rng = ChaCha20Rng::from_entropy();
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
        let mut encrypted = bytes_encrypt(&mut rng, &ciphertext, pk.clone()).unwrap();
        let encrypted1 = bytes_encrypt(&mut rng, &ciphertext, pk).unwrap();

        assert_ne!(encrypted, encrypted1);

        let decrypted = bytes_decrypt(&encrypted, sk.clone()).unwrap();

        assert_eq!(decrypted, ciphertext.to_vec());

        encrypted[2] = 0;
        encrypted[1] = 0;
        encrypted[3] = 0;
        encrypted[4] = 0;
        encrypted[5] = 0;
        encrypted[6] = 0;

        let decrypted = bytes_decrypt(&encrypted, sk).unwrap();

        assert_ne!(decrypted, ciphertext.to_vec());
    }

    #[test]
    fn test_invalid_keys() {
        let mut rng = ChaCha20Rng::from_entropy();
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
        let encrypted = bytes_encrypt(&mut rng, &ciphertext, pk).unwrap();
        let decrypted = bytes_decrypt(&encrypted, invalid_sk).unwrap();

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

        assert!(bytes_decrypt(&invalid_bytes, invalid_sk).is_err());
    }
}
