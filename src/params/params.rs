#[derive(Debug)]
pub struct NTRUParams {
    pub p: usize,
    pub q: usize,
    pub w: usize,
    // TODO: Add lrp other params
}

impl NTRUParams {
    pub fn from(p: usize, q: usize, w: usize) -> Self {
        NTRUParams { p, q, w }
    }
}
