#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{SECRETKEYS_BYTES, SMALL_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{SECRETKEYS_BYTES, SMALL_BYTES};
use crate::poly::{f3::round, r3::R3, rq::Rq};

// /* (G,A),a = KeyGen(G); leaves G unchanged */
// static void KeyGen(int16 *A, int8 *a, const int16 *G) {
//   int16 aG[P];
//
//   Short_random(a);
//   Rq_mult_small(aG, G, a);
//   Round(A, aG);
// }

fn key_gen(entropy: &R3, g: &Rq) -> Rq {
    let mut ag = g.mult_r3(&entropy);

    round(&mut ag.coeffs);

    ag
}
