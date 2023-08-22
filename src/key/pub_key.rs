use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PubKey<const SIZE: usize> {
    pub h: PolyInt<u16, SIZE>,
}

// TODO: make ToString impl

impl<const SIZE: usize> PubKey<SIZE> {
    pub fn new() -> Self {
        PubKey {
            h: PolyInt::<u16, SIZE>::new(),
        }
    }
}
