use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePrivKey {
    p: u16,
    f: NtruIntPoly,
    g_inv: NtruIntPoly,
}
