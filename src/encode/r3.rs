#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, SMALL_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, SMALL_BYTES};

pub fn r3_encode(f: &[i8; P]) -> [u8; SMALL_BYTES] {
    let mut s = [0u8; SMALL_BYTES];
    let mut fi = 0;

    for i in 0..P / 4 {
        let mut x = f[fi] + 1;
        fi += 1;
        x += (f[fi] + 1) << 2;
        fi += 1;
        x += (f[fi] + 1) << 4;
        fi += 1;
        x += (f[fi] + 1) << 6;
        fi += 1;

        s[i] = x as u8;
    }

    s[P / 4] = (f[fi] + 1) as u8;

    s
}

pub fn r3_decode(s: &[u8; SMALL_BYTES]) -> [i8; P] {
    let mut f = [0i8; P];
    let mut x: u8;
    let mut i = 0;
    let swap = move |x: u8| -> i8 {
        let r = (x & 3) as i8 - 1;

        r
    };

    while i < P / 4 {
        x = s[i];
        f[i * 4] = swap(x);
        x >>= 2;
        f[i * 4 + 1] = swap(x);
        x >>= 2;
        f[i * 4 + 2] = swap(x);
        x >>= 2;
        f[i * 4 + 3] = swap(x);
        i += 1;
    }

    x = s[i];
    f[i * 4] = swap(x);

    f
}

#[cfg(test)]
mod r3_encoder_tests {
    use super::*;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[test]
    fn test_r3_encode() {
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }
}
