use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePubKey {
    pub p: u16,
    pub h: NtruIntPoly,
}
