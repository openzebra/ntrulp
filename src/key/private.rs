use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePrivKey {
    pub f: NtruIntPoly,
    pub g_inv: NtruIntPoly,
}

impl NtruPrimePrivKey {
    pub fn empty() -> Self {
        let f = NtruIntPoly::empty();
        let g_inv = NtruIntPoly::empty();

        NtruPrimePrivKey { f, g_inv }
    }
}
