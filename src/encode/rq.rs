use crate::math::nums::{u32_divmod_u14, u32_mod_u14};

fn encode(out: &mut Vec<u8>, r: &[u16], m: &[u16], len: usize) {
    if len == 1 {
        let mut r_val = r[0];
        let mut m_val = m[0];
        while m_val > 1 {
            out.push(r_val as u8);
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
            while m_val >= 16384 {
                out.push(r_val as u8);
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
        encode(out, &r2, &m2, (len + 1) / 2);
    }
}

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

pub fn rq_encode<const P: usize, const Q: usize, const Q12: usize>(rq: &[i16; P]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut r = [0u16; P];
    let m = [Q as u16; P];

    for i in 0..P {
        r[i] = (rq[i] + Q12 as i16) as u16;
    }

    encode(&mut out, &r, &m, P);

    out
}

pub fn rq_decode<const P: usize, const Q: usize, const Q12: usize>(s: &[u8]) -> [i16; P] {
    let mut rq = [0i16; P];
    let mut r = [0u16; P];
    let m = [Q as u16; P];

    decode(&mut r, &s, &m, P);

    for i in 0..P {
        rq[i] = (r[i] as i16) - Q12 as i16;
    }

    rq
}

#[test]
fn test_encode() {
    use crate::kem::rq::Rq;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    const P: usize = 761;
    const W: usize = 286;
    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;

    let mut random: NTRURandom<P> = NTRURandom::new();
    let rq: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap()).recip3().unwrap();
    let out = rq_encode::<P, Q, Q12>(&rq.coeffs);
    let dec = rq_decode::<P, Q, Q12>(&out);

    assert_eq!(dec, rq.coeffs);
}
