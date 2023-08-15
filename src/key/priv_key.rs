use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PrivKey {
    pub f: PolyInt<i8>,
    pub g_inv: PolyInt<i16>,
}

impl PrivKey {
    pub fn empty() -> Self {
        let f = PolyInt::empty();
        let g_inv = PolyInt::empty();

        PrivKey { f, g_inv }
    }
}
