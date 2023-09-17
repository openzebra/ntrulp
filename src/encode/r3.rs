use crate::params::params::{P, SMALL_BYTES};

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
    use super::{r3_decode, r3_encode};
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    #[cfg(feature = "ntrulpr653")]
    #[test]
    fn test_r3_encode_653() {
        use crate::params::params::P;

        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[cfg(feature = "ntrulpr761")]
    #[test]
    fn test_r3_encode_761() {
        use crate::params::params::P;
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[cfg(feature = "ntrulpr857")]
    #[test]
    fn test_r3_encode_857() {
        use crate::params::params::P;
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[cfg(feature = "ntrulpr953")]
    #[test]
    fn test_r3_encode_953() {
        use crate::params::params::P;
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[cfg(feature = "ntrulpr1013")]
    #[test]
    fn test_r3_encode_1013() {
        use crate::params::params::P;
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[cfg(feature = "ntrulpr1277")]
    #[test]
    fn test_r3_encode_1277() {
        use crate::params::params::P;
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }
}
