use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePubKey {
    p: u16,
    h: NtruIntPoly,
}
