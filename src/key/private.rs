use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePrivKey {
    pub f: NtruIntPoly,
    pub g_inv: NtruIntPoly,
}
