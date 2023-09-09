use crate::math::nums::{u32_divmod_u14, u32_mod_u14};

// TODO: target for improve!, add guard to avoid endless
fn encode(out: &mut [u8], index: &mut usize, r: &[u16], m: &[u16], len: usize) {
    if len == 1 {
        let mut r_val = r[0] as u32;
        let mut m_val = m[0] as u32;

        while m_val > 1 {
            match out.get_mut(*index) {
                Some(v) => {
                    *v = r_val as u8;
                    *index += 1;
                }
                None => continue,
            };
            r_val >>= 8;
            m_val = (m_val + 255) >> 8;
        }
    }

    if len > 1 {
        let mut r2 = vec![0; (len + 1) / 2];
        let mut m2 = vec![0; (len + 1) / 2];
        let mut i = 0;
        while i < len - 1 {
            let m0 = m[i] as u32;
            let mut r_val = r[i] as u32 + (r[i + 1] as u32) * m0;
            let mut m_val = (m[i + 1] as u32) * m0;

            // while_inser(r_val, m_val, &mut out);
            while m_val >= 16384 {
                match out.get_mut(*index) {
                    Some(v) => {
                        *v = r_val as u8;
                        *index += 1;
                    }
                    None => continue,
                };
                r_val >>= 8;
                m_val = (m_val + 255) >> 8;
            }
            r2[i / 2] = r_val as u16;
            m2[i / 2] = m_val as u16;
            i += 2;
        }
        if i < len {
            r2[i / 2] = r[i];
            m2[i / 2] = m[i];
        }

        encode(out, index, &r2, &m2, (len + 1) / 2);
    }
}

// TODO: target for improve!, add guard to avoid endless
fn decode(out: &mut [u16], slice: &[u8], m: &[u16], len: usize) {
    let mut s = slice;

    if len == 1 {
        if m[0] == 1 {
            out[0] = 0;
        } else if m[0] <= 256 {
            out[0] = u32_mod_u14(s[0] as u32, m[0]);
        } else {
            out[0] = u32_mod_u14((s[0] as u32) + (((s[1] as u16) << 8) as u32), m[0]);
        }
    }
    if len > 1 {
        let mut r2 = vec![0u16; (len + 1) / 2];
        let mut m2 = vec![0u16; (len + 1) / 2];
        let mut bottomr = vec![0u16; len / 2];
        let mut bottomt = vec![0u32; len / 2];
        let mut i = 0;
        while i < len - 1 {
            let m_val = (m[i] as u32) * (m[i + 1] as u32);
            if m_val > 256 * 16383 {
                bottomt[i / 2] = 256 * 256;
                bottomr[i / 2] = (s[0] as u16) + 256 * (s[1] as u16);
                s = &s[2..];
                m2[i / 2] = ((((m_val + 255) >> 8) + 255) >> 8) as u16;
            } else if m_val >= 16384 {
                bottomt[i / 2] = 256;
                bottomr[i / 2] = s[0] as u16;
                s = &s[1..];
                m2[i / 2] = ((m_val + 255) >> 8) as u16;
            } else {
                bottomt[i / 2] = 1;
                bottomr[i / 2] = 0;
                m2[i / 2] = m_val as u16;
            }

            i += 2;
        }
        if i < len {
            m2[i / 2] = m[i];
        }
        decode(&mut r2, &s, &m2, (len + 1) / 2);
        i = 0;
        while i < len - 1 {
            let r = bottomr[i / 2] as u32 + bottomt[i / 2] * r2[i / 2] as u32;
            let (mut r1, r0) = u32_divmod_u14(r, m[i]);

            r1 = u32_mod_u14(r1, m[i + 1]) as u32;
            out[i] = r0;
            out[i + 1] = r1 as u16;
            i += 2;
        }
        if i < len {
            out[i] = r2[i / 2];
        }
    }
}

pub fn rq_encode<const P: usize, const Q: usize, const Q12: usize, const RQ_BYTES: usize>(
    rq: &[i16; P],
) -> [u8; RQ_BYTES] {
    let mut out = [0u8; RQ_BYTES];
    let mut r = [0u16; P];
    let m = [Q as u16; P];

    for i in 0..P {
        r[i] = (rq[i] + Q12 as i16) as u16;
    }

    encode(&mut out, &mut 0, &r, &m, P);

    out
}

/// TODO: Add const because s=1158 elements!.
pub fn rq_decode<const P: usize, const Q: usize, const Q12: usize, const RQ_BYTES: usize>(
    s: &[u8],
) -> [i16; P] {
    let mut rq = [0i16; P];
    let mut r = [0u16; P];
    let m = [Q as u16; P];

    decode(&mut r, &s, &m, P);

    for i in 0..P {
        rq[i] = (r[i] as i16) - Q12 as i16;
    }

    rq
}

pub fn rq_rounded_decode<
    const P: usize,
    const Q: usize,
    const Q12: usize,
    const ROUNDED_BYTES: usize,
>(
    s: &[u8; ROUNDED_BYTES],
) -> [i16; P] {
    let mut rq = [0i16; P];
    let mut r = [0u16; P];
    let m = [(Q as u16 + 2) / 3; P];

    decode(&mut r, s, &m, P);

    for i in 0..P {
        rq[i] = (r[i] as i16 * 3) - Q12 as i16;
    }

    rq
}

pub fn rq_rounded_encode<
    const P: usize,
    const Q: usize,
    const Q12: usize,
    const ROUNDED_BYTES: usize,
>(
    rq: &[i16; P],
) -> [u8; ROUNDED_BYTES] {
    let mut s = [0u8; ROUNDED_BYTES];
    let mut r = [0u16; P];
    let mut m = [0u16; P];

    for i in 0..P {
        let v32 = (rq[i] + Q12 as i16) as u32;
        r[i] = ((v32 * 10923) >> 15) as u16;
    }

    for i in 0..P {
        m[i] = (Q as u16 + 2) / 3;
    }

    encode(&mut s, &mut 0, &r, &m, P);

    s
}

#[cfg(test)]
mod rq_encoder_tests {
    use super::*;
    use crate::kem::rq::Rq;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;
    use rand::Rng;

    #[test]
    fn test_rq_encode_rq_decode_761() {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1158;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rq_encode_rq_decode_858() {
        const P: usize = 857;
        const Q: usize = 5167;
        const W: usize = 322;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1322;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rq_encode_rq_decode_653() {
        const P: usize = 653;
        const Q: usize = 4621;
        const W: usize = 288;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 994;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rq_encode_rq_decode_953() {
        const P: usize = 953;
        const Q: usize = 6343;
        const W: usize = 396;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1505;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rq_encode_rq_decode_1013() {
        const P: usize = 1013;
        const Q: usize = 7177;
        const W: usize = 448;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1623;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rq_encode_rq_decode_1277() {
        const P: usize = 1277;
        const Q: usize = 7879;
        const W: usize = 492;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 2067;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
        let out = rq_encode::<P, Q, Q12, RQ_BYTES>(&rq.coeffs);
        let dec = rq_decode::<P, Q, Q12, RQ_BYTES>(&out);

        assert_eq!(dec, rq.coeffs);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_761() {
        const P: usize = 761;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 1007;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];

        rng.fill(&mut bytes[..]);
        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes);
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_858() {
        const P: usize = 857;
        const Q: usize = 5167;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 1152;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];

        rng.fill(&mut bytes[..]);

        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes);
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_653() {
        const P: usize = 653;
        const Q: usize = 4621;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 865;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];

        rng.fill(&mut bytes[..]);

        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes);
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_953() {
        const P: usize = 953;
        const Q: usize = 6343;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 1317;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];
        rng.fill(&mut bytes[..]);

        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes);
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_1013() {
        const P: usize = 1013;
        const Q: usize = 7177;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 1423;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];
        rng.fill(&mut bytes[..]);

        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes);
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }

    #[test]
    fn test_rounded_rq_encode_rq_decode_1277() {
        const P: usize = 1277;
        const Q: usize = 7879;
        const Q12: usize = (Q - 1) / 2;
        const ROUNDED_BYTES: usize = 1815;

        let mut rng = rand::thread_rng();
        let mut bytes: [u8; ROUNDED_BYTES] = [0u8; ROUNDED_BYTES];
        rng.fill(&mut bytes[..]);

        let rq = rq_rounded_decode::<P, Q, Q12, ROUNDED_BYTES>(&bytes.into());
        let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);

        assert_eq!(rq.len(), P);
        assert_eq!(dec.len(), ROUNDED_BYTES);
    }
}
