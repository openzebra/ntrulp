use crate::poly::poly::NtruIntPoly;

#[derive(Debug)]
pub struct NtruPrimePubKey {
    pub h: NtruIntPoly,
}

impl NtruPrimePubKey {
    pub fn empty() -> Self {
        let h = NtruIntPoly::empty();

        NtruPrimePubKey { h }
    }
}
