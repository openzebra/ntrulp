#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, TOP_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};

// pub fn top_encode(s: &[u8], t: &[i8]) {
//     for i in 0..TOP_BYTES {
//         let v = t[2 * i] + (t[2 * i + 1] << 4);
//
//         s[i] = v as u8;
//     }
// }
