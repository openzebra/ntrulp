pub struct Params {
    pub round1: bool,
    pub p: usize,
    pub q: usize,
    pub w: usize,
    pub delta: usize,
    pub tau: usize, // commonnly tau=16
    pub tau0: usize,
    pub tau1: usize,
    pub tau2: usize,
    pub tau3: usize,
    pub i: usize,
    pub top_bytes: usize,    // i/2
    pub inputs_bytes: usize, // I / 8 or (p + 3) / 4
    pub seeds_bytes: usize,  // random seeds
    pub small_bytes: usize,  // (p + 3) / 4
    pub hash_bytes: usize,   // sha_len / 2 or same confirm_bytes
    pub secretkeys_bytes: usize,
    pub publickeys_bytes: usize,
    pub ciphertexts_bytes: usize,
}

#[cfg(feature = "ntrulpr4591761")]
pub const PARAMS: Params = Params {
    round1: true, // round1
    p: 761,       // p
    q: 4591,      // q
    w: 250,       // w
    delta: 292,   // delta
    tau0: 2156,   // tau0
    tau1: 114,    // tau1
    tau2: 2007,   // tau2
    tau3: 287,    // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr761")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 761,        // p
    q: 4591,       // q
    w: 250,        // w
    delta: 292,    // delta
    tau0: 2156,    // tau0
    tau1: 114,     // tau1
    tau2: 2007,    // tau2
    tau3: 287,     // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr653")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 653,        // p
    q: 4621,       // q
    w: 252,        // w
    delta: 289,    // delta
    tau0: 2175,    // tau0
    tau1: 113,     // tau1
    tau2: 2031,    // tau2
    tau3: 290,     // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr857")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 857,        // p
    q: 5167,       // q
    w: 281,        // w
    delta: 329,    // delta
    tau0: 2433,    // tau0
    tau1: 101,     // tau1
    tau2: 2265,    // tau2
    tau3: 324,     // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr953")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 953,        // p
    q: 6343,       // q
    w: 345,        // w
    delta: 404,    // delta
    tau0: 2997,    // tau0
    tau1: 82,      // tau1
    tau2: 2798,    // tau2
    tau3: 400,     // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr1013")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 1013,       // p
    q: 7177,       // q
    w: 392,        // w
    lpr: true,     // lpr
    delta: 450,    // delta
    tau0: 3367,    // tau0
    tau1: 73,      // tau1
    tau2: 3143,    // tau2
    tau3: 449,     // tau3
    seeds_bytes: 32,
};

#[cfg(feature = "ntrulpr1277")]
pub const PARAMS: Params = Params {
    round1: false, // round1
    p: 1277,       // p
    q: 7879,       // q
    w: 429,        // w
    delta: 502,    // delta
    tau0: 3724,    // tau0
    tau1: 66,      // tau1
    tau2: 3469,    // tau2
    tau3: 496,     // tau3
    seeds_bytes: 32,
};
