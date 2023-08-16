use crate::poly::PolyInt;

#[derive(Debug)]
pub struct PubKey {
    pub h: PolyInt<u16>,
}

// TODO: make ToString impl

impl PubKey {
    pub fn from(rqg: &PolyInt<i16>, f: &PolyInt<i8>) -> Self {
        let rq_f3 = f.clone().mult_int(3);
        // let h = rqg.pol

        PubKey {
            h: PolyInt::empty(),
        }
    }
}
