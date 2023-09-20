#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, TAU0, TAU1};
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

// static int8 Top(Fq C) { return (TAU1 * (int32)(C + TAU0) + 16384) >> 15; }

pub fn top(c: i16) -> i8 {
    let tau0 = TAU0 as i32;
    let tau1 = TAU1 as i32;
    let c32 = c as i32;
    let value = (tau1 * (c32 + tau0) + 16384) >> 15;

    value as i8
}

#[cfg(test)]
mod tests_top {
    use super::*;

    #[test]
    fn test_top() {
        assert_eq!(top(4325), 23);
        assert_eq!(top(0), 8);
        assert_eq!(top(-30), 7);
        assert_eq!(top(i16::MAX), 121);
        assert_eq!(top(i16::MIN), -106);
    }
}
