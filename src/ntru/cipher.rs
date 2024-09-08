use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::params::params::{P, R3_BYTES, RQ_BYTES, W};
use crate::{
    math::nums::weightw_mask,
    poly::{f3::round, r3::R3, rq::Rq},
};

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
/// use rand::RngCore;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::rq_decrypt;
/// use ntrulp::rng::{random_small, short_random};
///
/// let mut rng = rand::thread_rng();
/// let f = Rq::from(short_random(&mut rng).unwrap());
/// let mut g: R3;
///
/// // Generate the ciphertext polynomial c and the private key
/// let c = Rq::from(short_random(&mut rng).unwrap());
/// let priv_key = loop {
///     g = R3::from(random_small(&mut rng));
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
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
    let cf = c.mult_r3(f);
    let cf3 = cf.mult_int(3);
    let e = cf3.r3_from_rq();
    let ev = e.mult(ginv);
    let mask: i16 = weightw_mask(ev.as_ref()); // 0 if weight w, else -1

    for (i, r) in r.iter_mut().enumerate() {
        if i < W {
            *r = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
        } else {
            *r = (ev.coeffs[i] as i16 & !mask) as i8;
        }
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
/// use rand::RngCore;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::rq_decrypt;
/// use ntrulp::ntru::cipher::r3_encrypt;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::rng::{random_small, short_random};
///
/// let mut rng = rand::thread_rng();
/// let f = Rq::from(short_random(&mut rng).unwrap());
/// let mut g: R3;
///
/// // Generate an content for encrypt
/// let r: R3 = Rq::from(short_random(&mut rng).unwrap()).r3_from_rq();
///
/// // Generate the private key priv_key
/// let priv_key = loop {
///     g = R3::from(random_small(&mut rng));
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
/// let pub_key = PubKey::from_sk(&priv_key).unwrap();
///
/// let encrypted = r3_encrypt(&r, &pub_key);
/// let decrypted = rq_decrypt(&encrypted, &priv_key);
///
/// assert_eq!(decrypted.coeffs, r.coeffs);
/// ```
///
/// # Notes
///
/// This function encrypts a polynomial `r` in the F3 field using a public key `pub_key`
/// in the Fq field and returns the ciphertext polynomial.
///
pub fn r3_encrypt(r: &R3, pub_key: &PubKey) -> Rq {
    let mut hr = pub_key.mult_r3(r);

    round(&mut hr.coeffs);

    hr
}

/// Encrypts a slice of bytes using the provided `NTRURandom` instance and the recipient's public key.
///
/// # Arguments
///
/// * `rng`: An instance of `NTRURandom` used for encryption.
/// * `bytes`: A slice of bytes to be encrypted.
/// * `pub_key`: The public key of the recipient.
///
/// # Returns
///
/// Returns the encrypted bytes.
///
/// # Example
/// ```rust
/// use rand::RngCore;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::static_bytes_encrypt;
/// use ntrulp::ntru::cipher::static_bytes_decrypt;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::rng::{random_small, short_random};
///
/// let mut rng = rand::thread_rng();
/// let f = Rq::from(short_random(&mut rng).unwrap());
/// let mut g: R3;
///
/// // Generate the private key priv_key
/// let sk = loop {
///     g = R3::from(random_small(&mut rng));
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
///
/// // Generate an content for encrypt
///
/// let pk = PubKey::from_sk(&sk).unwrap();
/// let plaintext = Rq::from(short_random(&mut rng).unwrap())
///     .r3_from_rq()
///     .to_bytes();
///
/// let encrypted = static_bytes_encrypt(&plaintext, &pk);
/// let decrypted = static_bytes_decrypt(&encrypted, &sk);
///
/// assert_eq!(decrypted, plaintext);
///
/// ```
///
/// # Panics
///
/// The function may panic if encryption fails or if the provided public key is invalid.
///
pub fn static_bytes_encrypt(bytes: &[u8; R3_BYTES], pub_key: &PubKey) -> [u8; RQ_BYTES] {
    r3_encrypt(&bytes.into(), pub_key).to_bytes()
}

/// Decrypts bytes and retrieves the bytes previously encrypted using the `bytes_encrypt` function.
///
/// # Arguments
///
/// * `bytes`: A slice of bytes to decrypt.
/// * `priv_key`: The private key used for decryption.
///
/// # Returns
///
/// Returns the decrypted bytes.
///
/// # Example
///
/// ```rust
/// use rand::RngCore;
/// use ntrulp::key::priv_key::PrivKey;
/// use ntrulp::poly::rq::Rq;
/// use ntrulp::poly::r3::R3;
/// use ntrulp::ntru::cipher::static_bytes_encrypt;
/// use ntrulp::ntru::cipher::static_bytes_decrypt;
/// use ntrulp::key::pub_key::PubKey;
/// use ntrulp::rng::{random_small, short_random};
///
/// let mut rng = rand::thread_rng();
/// let f = Rq::from(short_random(&mut rng).unwrap());
/// let mut g: R3;
///
/// // Generate the private key priv_key
/// let sk = loop {
///     g = R3::from(random_small(&mut rng));
///     match PrivKey::compute(&f, &g) {
///         Ok(s) => break s,
///         Err(_) => continue,
///     };
/// };
///
/// // Generate an content for encrypt
///
/// let pk = PubKey::from_sk(&sk).unwrap();
/// let plaintext = Rq::from(short_random(&mut rng).unwrap())
///     .r3_from_rq()
///     .to_bytes();
///
/// let encrypted = static_bytes_encrypt(&plaintext, &pk);
/// let decrypted = static_bytes_decrypt(&encrypted, &sk);
///
/// assert_eq!(decrypted, plaintext);
/// ```
///
/// # Panics
///
/// The function may panic if decryption fails or if the private key is invalid.
///
pub fn static_bytes_decrypt(cipher_bytes: &[u8; RQ_BYTES], priv_key: &PrivKey) -> [u8; R3_BYTES] {
    rq_decrypt(&cipher_bytes.into(), priv_key).to_bytes()
}

#[cfg(test)]
mod test_cipher {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use crate::rng::{random_small, short_random};

    use super::*;

    #[test]
    fn test_bytes_encrypt_and_decrypt() {
        let mut rng = ChaCha20Rng::from_entropy();
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        let mut g: R3;
        let sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();

        let plaintext = Rq::from(short_random(&mut rng).unwrap())
            .r3_from_rq()
            .to_bytes();

        let encrypted = static_bytes_encrypt(&plaintext, &pk);
        let decrypted = static_bytes_decrypt(&encrypted, &sk);

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let mut rng = ChaCha20Rng::from_entropy();
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        let mut g: R3;
        let sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();

        let plaintext: R3 = Rq::from(short_random(&mut rng).unwrap()).r3_from_rq();

        let encrypted = r3_encrypt(&plaintext, &pk);
        let decrypted = rq_decrypt(&encrypted, &sk);

        assert_eq!(decrypted, plaintext.into());
    }
}
