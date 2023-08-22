use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PrivKey<const SIZE: usize> {
    pub f: PolyInt<u8, SIZE>,
    pub g_inv: PolyInt<u16, SIZE>,
}

// TODO: make ToString impl

impl<const SIZE: usize> PrivKey<SIZE> {
    pub fn empty() -> Self {
        let f: PolyInt<u8, SIZE> = PolyInt::new();
        let g_inv: PolyInt<u16, SIZE> = PolyInt::new();

        PrivKey { f, g_inv }
    }
}
