#[cfg(feature = "ntrulpr653")]
pub mod params653 {
    pub const P: usize = 653;
    pub const Q: usize = 4621;
    pub const W: usize = 252;
    pub const DELTA: usize = 289;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 2175;
    pub const TAU1: usize = 113;
    pub const TAU2: usize = 2031;
    pub const TAU3: usize = 290;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 865;
    pub const RQ_BYTES: usize = 994;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}

#[cfg(feature = "ntrulpr761")]
pub mod params761 {
    pub const P: usize = 761;
    pub const Q: usize = 4591;
    pub const W: usize = 250;
    pub const DELTA: usize = 292;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 2156;
    pub const TAU1: usize = 114;
    pub const TAU2: usize = 2007;
    pub const TAU3: usize = 287;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 1007;
    pub const RQ_BYTES: usize = 1158;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}

#[cfg(feature = "ntrulpr857")]
pub mod params857 {
    pub const P: usize = 857;
    pub const Q: usize = 5167;
    pub const W: usize = 281;
    pub const DELTA: usize = 329;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 2433;
    pub const TAU1: usize = 101;
    pub const TAU2: usize = 2265;
    pub const TAU3: usize = 324;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 1152;
    pub const RQ_BYTES: usize = 1322;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}

#[cfg(feature = "ntrulpr953")]
pub mod params953 {
    pub const P: usize = 953;
    pub const Q: usize = 6343;
    pub const W: usize = 345;
    pub const DELTA: usize = 404;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 2997;
    pub const TAU1: usize = 82;
    pub const TAU2: usize = 2798;
    pub const TAU3: usize = 400;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 1317;
    pub const RQ_BYTES: usize = 1505;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}

#[cfg(feature = "ntrulpr1013")]
pub mod params1013 {
    pub const P: usize = 1013;
    pub const Q: usize = 7177;
    pub const W: usize = 392;
    pub const DELTA: usize = 450;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 3367;
    pub const TAU1: usize = 73;
    pub const TAU2: usize = 3143;
    pub const TAU3: usize = 449;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 1423;
    pub const RQ_BYTES: usize = 1623;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}

#[cfg(feature = "ntrulpr1277")]
pub mod params1277 {
    pub const P: usize = 1277;
    pub const Q: usize = 7879;
    pub const W: usize = 492;
    pub const DELTA: usize = 502;
    pub const TAU: usize = 16;
    pub const TAU0: usize = 3724;
    pub const TAU1: usize = 66;
    pub const TAU2: usize = 3469;
    pub const TAU3: usize = 496;
    pub const I: usize = 256;
    pub const SEEDS_BYTES: usize = 32;
    pub const ROUNDED_BYTES: usize = 1815;
    pub const RQ_BYTES: usize = 2067;
    pub const TOP_BYTES: usize = I / 2;
    pub const Q12: usize = (Q - 1) / 2;
    pub const INPUTS_BYTES: usize = (P + 3) / 4;
    pub const SMALL_BYTES: usize = INPUTS_BYTES;
    pub const HASH_BYTES: usize = 32;
    pub const SECRETKEYS_BYTES: usize = SMALL_BYTES * 2;
    pub const PUBLICKEYS_BYTES: usize = SEEDS_BYTES + ROUNDED_BYTES;
    pub const CIPHERTEXTS_BYTES: usize = ROUNDED_BYTES + TOP_BYTES;
}
