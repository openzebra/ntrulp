use crate::params::params::{R3_BYTES, SECRETKEYS_BYTES};

use crate::{
    encode::r3,
    poly::{r3::R3, rq::Rq},
};

use super::kem_error::KemErrors;

#[derive(Clone)]
pub struct PrivKey(pub R3, pub R3);

impl PrivKey {
    /// Represents a private key with two components `f` and `g` in the context of the Fq and F3 fields.
    ///
    /// A private key `PrivKey` consists of two components:
    /// - `f`: An element in the Fq field, derived from entropy and random data.
    /// - `g`: The inverse of an element in the F3 field, also derived from entropy and random data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::priv_key::PrivKey;
    ///
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let fq = Rq::from(short_random(&mut rng).unwrap());
    /// let mut g3: R3;
    /// // Compute the private key
    /// let priv_key = loop {
    ///     g3 = R3::from(random_small(&mut rng));
    ///     match PrivKey::compute(&fq, &g3) {
    ///         Ok(s) => break s,
    ///         Err(_) => continue,
    ///     };
    /// };
    /// ```
    ///
    /// # Notes
    ///
    /// This implementation represents a private key with two components, `f` and `g`,
    /// which are derived from entropy and random data in the Fq and F3 fields respectively.
    ///
    pub fn compute(f: &Rq, g: &R3) -> Result<Self, KemErrors> {
        let ginv = g.recip().map_err(KemErrors::PolyErrors)?;

        Ok(PrivKey(f.r3_from_rq(), ginv))
    }

    /// Converts a private key, represented as polynomials in the fields `Fq` and `R3`, into a byte array.
    ///
    /// # Returns
    ///
    /// Returns a byte array representing the private key.
    ///
    /// # Example
    ///
    /// ```
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::priv_key::PrivKey;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let fq = Rq::from(short_random(&mut rng).unwrap());
    /// let mut g3: R3;
    /// // Compute the private key
    /// let priv_key = loop {
    ///     g3 = R3::from(random_small(&mut rng));
    ///     match PrivKey::compute(&fq, &g3) {
    ///         Ok(s) => break s,
    ///         Err(_) => continue,
    ///     };
    /// };
    /// let sk_as_bytes = priv_key.to_bytes();
    /// let from_bytes = PrivKey::import(&sk_as_bytes).unwrap();
    /// ```
    ///
    pub fn to_bytes(&self) -> [u8; SECRETKEYS_BYTES] {
        let mut sk = [0u8; SECRETKEYS_BYTES];

        sk[..R3_BYTES].copy_from_slice(&self.1.to_bytes());
        sk[R3_BYTES..].copy_from_slice(&self.0.to_bytes());

        sk
    }

    /// Imports bytes and converts them into two key polynomials.
    ///
    /// # Arguments
    ///
    /// * `bytes`: A slice of bytes containing the data to import.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing two key polynomials - `f` and `ginv` (Self).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntrulp::poly::rq::Rq;
    /// use ntrulp::poly::r3::R3;
    /// use ntrulp::key::priv_key::PrivKey;
    /// use rand::RngCore;
    /// use ntrulp::rng::{random_small, short_random};
    ///
    /// let mut rng = rand::thread_rng();
    /// // Create an Fq polynomial fq and a g3 polynomial g3
    /// let fq = Rq::from(short_random(&mut rng).unwrap());
    /// let mut g3: R3;
    /// // Compute the private key
    /// let priv_key = loop {
    ///     g3 = R3::from(random_small(&mut rng));
    ///     match PrivKey::compute(&fq, &g3) {
    ///         Ok(s) => break s,
    ///         Err(_) => continue,
    ///     };
    /// };
    /// let sk_as_bytes = priv_key.to_bytes();
    /// let imported_sk = PrivKey::import(&sk_as_bytes).unwrap();
    /// ```
    ///
    /// # Notes
    ///
    /// Import privateKey as bytes format and convert it to poly
    ///
    pub fn import(sk: &[u8; SECRETKEYS_BYTES]) -> Result<Self, KemErrors> {
        let ginv_bytes: [u8; R3_BYTES] = sk[..R3_BYTES]
            .try_into()
            .or(Err(KemErrors::InvalidR3GInvrBytes))?;

        let f_bytes: [u8; R3_BYTES] = sk[R3_BYTES..]
            .try_into()
            .or(Err(KemErrors::InvalidR3FBytes))?;

        let ginv = R3::from(r3::r3_decode(&ginv_bytes));
        let f = R3::from(r3::r3_decode(&f_bytes));

        Ok(PrivKey(f, ginv))
    }
}

impl TryFrom<[u8; SECRETKEYS_BYTES]> for PrivKey {
    type Error = KemErrors;
    fn try_from(value: [u8; SECRETKEYS_BYTES]) -> Result<Self, Self::Error> {
        Self::import(&value)
    }
}

#[cfg(test)]
mod test_private_key {
    use super::*;
    use crate::rng::{random_small, short_random};

    #[test]
    fn test_import_export() {
        let mut rng = rand::thread_rng();

        for _ in 0..2 {
            let f: Rq = Rq::from(short_random(&mut rng).unwrap());
            let g: R3 = R3::from(random_small(&mut rng));
            let secret_key = match PrivKey::compute(&f, &g) {
                Ok(sk) => sk,
                Err(_) => continue,
            };
            let bytes = secret_key.to_bytes();
            let new_secret_key = match PrivKey::import(&bytes) {
                Ok(v) => v,
                Err(_) => continue,
            };

            assert_eq!(new_secret_key.0.coeffs, secret_key.0.coeffs);
            assert_eq!(new_secret_key.1.coeffs, secret_key.1.coeffs);
        }
    }
}
