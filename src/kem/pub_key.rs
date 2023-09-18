use crate::poly::rq::Rq;

#[derive(Debug)]
pub struct PubKey {
    pub h: Rq,
}

impl PubKey {
    pub fn new() -> Self {
        let h = Rq::new();

        Self { h }
    }

    pub fn from(h: Rq) -> Self {
        Self { h }
    }
}
