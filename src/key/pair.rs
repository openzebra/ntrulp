use crate::key::{private::NtruPrimePrivKey, public::NtruPrimePubKey};

#[derive(Debug)]
pub struct NtruPrimeKeyPair {
    pub private: NtruPrimePrivKey,
    pub public: NtruPrimePubKey,
}
