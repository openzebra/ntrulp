#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, SEEDS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{SECRETKEYS_BYTES, SMALL_BYTES};
use crate::{
    ntru::lpr::generator,
    poly::{f3::round, r3::R3, rq::Rq},
};

fn key_gen(entropy: &R3, g: &Rq) -> Rq {
    let mut ag = g.mult_r3(&entropy);

    round(&mut ag.coeffs);

    ag
}

fn x_key_gen(entropy: &R3, seed: [u8; SEEDS_BYTES]) -> Rq {
    let g = Rq::from(generator(&seed).unwrap());
    key_gen(&entropy, &g)
}
