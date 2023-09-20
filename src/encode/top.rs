#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{I, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{I, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{I, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{I, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{I, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{I, TAU0, TAU1, TOP_BYTES};

pub fn top_encode<const SIZE: usize>(s: &mut [u8; SIZE], t: &[i8; I]) {
    for i in 0..TOP_BYTES {
        let v = t[2 * i] + (t[2 * i + 1] << 4);

        s[i] = v as u8;
    }
}

pub fn top_decode<const SIZE: usize>(t: &mut [i8; I], s: [u8; SIZE]) {
    for i in 0..TOP_BYTES {
        t[2 * i] = (s[i] & 15) as i8;
        t[2 * i + 1] = (s[i] >> 4) as i8;
    }
}

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
