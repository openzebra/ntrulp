#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, Q, RQ_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, Q, RQ_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, Q, RQ_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, Q, RQ_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, Q, RQ_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, Q, RQ_BYTES};
use crate::poly::fq;

const Q_SHIFT: i32 = Q as i32 / 2;
const END_BYTES: usize = RQ_BYTES / 8;
const MOD: usize = RQ_BYTES % 8;

pub fn encode(input: &[i16; P]) -> [u8; RQ_BYTES] {
    let mut out = [0u8; RQ_BYTES];
    let mut f0: i32;
    let mut f1: i32;
    let mut f2: i32;
    let mut f3: i32;
    let mut f4: i32;

    let mut j = 0;
    let mut k = 0;

    for _ in 0..END_BYTES {
        f0 = input[j] as i32 + Q_SHIFT;
        f1 = (input[j + 1] as i32 + Q_SHIFT) * 3;
        f2 = (input[j + 2] as i32 + Q_SHIFT) * 9;
        f3 = (input[j + 3] as i32 + Q_SHIFT) * 27;
        f4 = (input[j + 4] as i32 + Q_SHIFT) * 81;

        j += 5;

        f0 += f1 << 11;
        out[k] = f0 as u8;
        f0 >>= 8;
        out[k + 1] = f0 as u8;
        f0 >>= 8;
        f0 += f2 << 6;
        out[k + 2] = f0 as u8;
        f0 >>= 8;
        out[k + 3] = f0 as u8;
        f0 >>= 8;
        f0 += f3 << 1;
        out[k + 4] = f0 as u8;
        f0 >>= 8;
        f0 += f4 << 4;
        out[k + 5] = f0 as u8;
        f0 >>= 8;
        out[k + 6] = f0 as u8;
        f0 >>= 8;
        out[k + 7] = f0 as u8;
        k += 8;
    }

    if MOD == 0 {
        return out;
    } else if MOD == 2 {
        f0 = input[j] as i32 + Q_SHIFT;

        out[k] = f0 as u8;
        out[k + 1] = (f0 >> 8) as u8;
    } else if MOD == 3 {
        f0 = input[j] as i32 + Q_SHIFT;
        f1 = (input[j + 1] as i32 + Q_SHIFT) * 3;
        f2 = (input[j + 2] as i32 + Q_SHIFT) * 9;

        f0 += f1 << 11;

        out[k] = f0 as u8;
        f0 >>= 8;
        out[k + 1] = f0 as u8;
        f0 >>= 8;
        f0 += f2 << 6;
        out[k + 2] = f0 as u8;
    }

    out
}

pub fn decode(input: &[u8; RQ_BYTES]) -> [i16; P] {
    let mut out = [0i16; P];
    let mut f0: u32;
    let mut f1: u32;
    let mut f2: u32;
    let mut f3: u32;
    let mut f4: u32;

    let mut c0: u32;
    let mut c1: u32;
    let mut c2: u32;
    let mut c3: u32;
    let mut c4: u32;
    let mut c5: u32;
    let mut c6: u32;
    let mut c7: u32;

    let mut j = 0;
    let mut k = 0;

    for _ in 0..END_BYTES {
        c0 = input[j] as u32;
        c1 = input[j + 1] as u32;
        c2 = input[j + 2] as u32;
        c3 = input[j + 3] as u32;
        c4 = input[j + 4] as u32;
        c5 = input[j + 5] as u32;
        c6 = input[j + 6] as u32;
        c7 = input[j + 7] as u32;

        j += 8;
        c6 += c7 << 8;
        f4 = (103_564 * c6 + 405 * (c5 + 1)) >> 19;
        c5 += c6 << 8;
        c5 -= (f4 * 81) << 4;
        c4 += c5 << 8;
        f3 = (9_709 * (c4 + 2)) >> 19;
        c4 -= (f3 * 27) << 1;
        c3 += c4 << 8;
        f2 = (233_017 * c3 + 910 * (c2 + 2)) >> 19;
        c2 += c3 << 8;
        c2 -= (f2 * 9) << 6;
        c1 += c2 << 8;
        f1 = (21_845 * (c1 + 2) + 85 * c0) >> 19;
        c1 -= (f1 * 3) << 3;
        c0 += c1 << 8;
        f0 = c0;

        out[k] = fq::freeze((f0 + Q as u32 - Q_SHIFT as u32) as i32);
        out[k + 1] = fq::freeze((f1 + Q as u32 - Q_SHIFT as u32) as i32);
        out[k + 2] = fq::freeze((f2 + Q as u32 - Q_SHIFT as u32) as i32);
        out[k + 3] = fq::freeze((f3 + Q as u32 - Q_SHIFT as u32) as i32);
        out[k + 4] = fq::freeze((f4 + Q as u32 - Q_SHIFT as u32) as i32);

        k += 5;
    }

    if MOD == 0 {
        return out;
    } else if MOD == 2 {
        c0 = input[j] as u32;
        c1 = input[j + 1] as u32;
        c0 += c1 << 8;

        out[k] = fq::freeze((c0 + Q as u32 - Q_SHIFT as u32) as i32);
    } else if MOD == 3 {
        c0 = input[j] as u32;
        c1 = input[j + 1] as u32;
        c2 = input[j + 2] as u32;

        c1 += c2 << 8;
        f1 = (21_845 * (c1 + 2) + 85 * c0) >> 19;
        c1 -= (f1 * 3) << 3;
        c0 += c1 << 8;
        f0 = c0;

        out[k] = fq::freeze((f0 + Q as u32 - Q_SHIFT as u32) as i32);
        out[k + 1] = fq::freeze((f1 + Q as u32 - Q_SHIFT as u32) as i32);
    }

    out
}

#[cfg(test)]
mod tests_fq {
    use super::*;
    use crate::{
        poly::rq::Rq,
        random::{CommonRandom, NTRURandom},
    };

    #[test]
    fn test_encode_decode() {
        let mut rng = NTRURandom::new();

        for _ in 0..1 {
            let coeffs = rng.short_random().unwrap();
            let rq = Rq::from(coeffs);

            let bytes = encode(&rq.coeffs);
            let res = decode(&bytes);

            // println!("{:?}",bytes);

            assert_eq!(rq.coeffs, res);
        }
    }
}
