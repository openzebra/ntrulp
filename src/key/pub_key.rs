use crate::params::params::PUBLICKEYS_BYTES;

use super::kem_error::KemErrors;
use crate::{
    encode::rq,
    poly::{r3::R3, rq::Rq},
};

use super::priv_key::PrivKey;

pub type PubKey = Rq;

impl PubKey {
    /// Represents a public key in the context of a polynomial in the Fq field.
    ///
    /// A public key `PubKey` is formed as the result of a polynomial operation in the Fq field.
    /// It is computed as `h = (1/3 * fq) * g3`, where:
    /// - `h` is the public key.
    /// - `fq` is the entropy generated from random numbers in the field q.
    /// - `g3` is the entropy generated from random data in the field 3.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::pub_key::PubKey;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let fq = Rq::from(short_random(&mut rng).unwrap());
    /// let g3 = R3::from(random_small(&mut rng));
    /// // Compute the public key
    /// let pub_key = PubKey::compute(&fq, &g3).unwrap();
    /// ```
    ///
    /// # Notes
    ///
    /// This implementation represents a public key formed by performing a polynomial operation
    /// in the Fq field, combining entropy from `fq` and `g3`.
    ///
    pub fn compute(f: &Rq, g: &R3) -> Result<Self, KemErrors> {
        let finv = f.recip::<3>().map_err(KemErrors::PolyErrors)?;

        Ok(finv.mult_r3(g))
    }

    /// Computes a public key from a given private key.
    ///
    /// # Arguments
    ///
    /// * `private_key`: The private key from which to derive the public key.
    ///
    /// # Returns
    ///
    /// Returns the corresponding public key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::pub_key::PubKey;
    /// use ntrulp::key::priv_key::PrivKey;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    ///
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let f: Rq = Rq::from(short_random(&mut rng).unwrap());
    /// let mut g: R3;
    /// let priv_key = loop {
    ///     g = R3::from(random_small(&mut rng));
    ///
    ///     match PrivKey::compute(&f, &g) {
    ///          Ok(s) => break s,
    ///          Err(_) => continue,
    ///      };
    /// };
    /// let load_from_sk = PubKey::from_sk(&priv_key);
    /// ```
    ///
    /// # Panics
    ///
    /// The function may panic if the computation of the public key fails due to an invalid private key.
    ///
    pub fn from_sk(priv_key: &PrivKey) -> Result<Self, KemErrors> {
        let f = priv_key.0.rq_from_r3();
        let ginv = &priv_key.1;
        let g = ginv.recip().map_err(KemErrors::PolyErrors)?;
        let finv = f.recip::<3>().map_err(KemErrors::PolyErrors)?;
        let h = finv.mult_r3(&g);

        Ok(h)
    }

    /// Deserialize a byte array into a public key.
    ///
    /// # Arguments
    ///
    /// * `bytes`: A byte array containing the serialized public key.
    ///
    /// # Returns
    ///
    /// Returns the deserialized public key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::pub_key::PubKey;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let fq = Rq::from(short_random(&mut rng).unwrap());
    /// let g3 = R3::from(random_small(&mut rng));
    /// // Compute the public key
    /// let pub_key = PubKey::compute(&fq, &g3).unwrap();
    /// let imported_pub_key = PubKey::import(&pub_key.to_bytes());
    ///
    /// assert_eq!(pub_key.coeffs, imported_pub_key.coeffs);
    /// ```
    ///
    /// # Panics
    ///
    /// The function may panic if deserialization fails due to invalid or corrupted data.
    ///
    pub fn import(bytes: &[u8; PUBLICKEYS_BYTES]) -> Self {
        rq::decode(bytes).into()
    }
}

impl TryFrom<PrivKey> for PubKey {
    type Error = KemErrors;
    fn try_from(value: PrivKey) -> Result<Self, Self::Error> {
        Self::from_sk(&value)
    }
}

#[cfg(test)]
mod test_pub_key {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use super::*;
    use crate::rng::{random_small, short_random};

    #[test]
    fn test_import_export() {
        let mut rng = ChaCha20Rng::from_entropy();

        for _ in 0..1 {
            let f: Rq = Rq::from(short_random(&mut rng).unwrap());
            let g: R3 = R3::from(random_small(&mut rng));
            let pub_key = PubKey::compute(&f, &g).unwrap();
            let bytes = pub_key.to_bytes();
            let new_pub_key = PubKey::import(&bytes);

            assert_eq!(new_pub_key.coeffs, pub_key.coeffs);
        }
    }

    #[test]
    fn test_from_sk() {
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
        let pub_key_from_entropy = PubKey::compute(&f, &g).unwrap();
        let pub_key_from_sk = PubKey::from_sk(&sk).unwrap();

        assert_eq!(pub_key_from_sk.coeffs, pub_key_from_entropy.coeffs);
    }
}
