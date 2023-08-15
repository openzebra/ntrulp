use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PubKey {
    pub h: PolyInt<u16>,
}

// TODO: make ToString impl

impl PubKey {
    pub fn empty() -> Self {
        let h = PolyInt::empty();

        PubKey { h }
    }
}
