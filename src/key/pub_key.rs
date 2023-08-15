use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PubKey {
    pub h: PolyInt<u16>,
}

impl PubKey {
    pub fn empty() -> Self {
        let h = PolyInt::empty();

        PubKey { h }
    }
}
