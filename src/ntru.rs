use crate::config::params::StartParams;

pub struct NTRU {
    params: StartParams,
    hash_bytes: Vec<u8>,
    usecache: bool,
}

pub enum NTRUErrors {}

impl NTRU {
    pub fn from(params: StartParams) {}
}
