use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePrivKey {
    pub p: u16,
    pub f: NtruIntPoly,
    pub g_inv: NtruIntPoly,
}
