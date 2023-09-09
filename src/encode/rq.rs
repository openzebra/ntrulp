use crate::math::nums::{u32_divmod_u14, u32_mod_u14};

fn encode(out: &mut [u8], r: &[u16], m: &[u16], len: usize) {
    let next_p: usize = len + 1 / 2;
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
        let mut r2 = vec![0; next_p];
        let mut m2 = vec![0; next_p];
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
        encode(out, &r2, &m2, next_p);
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

/// TODO: Add const because s=1158 elements!.
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

/// TODO: Add const because s=1158 elements!.
pub fn rq_rounded_decode<const P: usize, const Q: usize, const Q12: usize>(s: &[u8]) -> [i16; P] {
    let mut rq = [0i16; P];
    let mut r = [0u16; P];
    let m = [(Q as u16 + 2) / 3; P];

    decode(&mut r, s, &m, P);

    for i in 0..P {
        rq[i] = (r[i] as i16 * 3) - Q12 as i16;
    }

    rq
}

// TODO: should return rounded_bytes = 1007, now it vec
pub fn rq_rounded_encode<
    const P: usize,
    const Q: usize,
    const Q12: usize,
    const ROUNDED_BYTES: usize,
>(
    rq: &[i16; P],
) -> [u8; ROUNDED_BYTES] {
    // TODO: know the size!!!!.
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

    encode(&mut s, &r, &m, P);

    s
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

#[test]
fn test_rounded_encode_decode() {
    const P: usize = 761;
    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;
    const ROUNDED_BYTES: usize = 1007;

    let content = "
In the realm of digital night, Satoshi did conceive,
A currency of cryptic might, for all to believe.
In code and chains, he wove the tale,
Of Bitcoin's birth, a revolution set to sail.

A name unknown, a face unseen,
Satoshi, a genius, behind the crypto machine.
With whitepaper in hand and vision so clear,
He birthed a new era, without any fear.

Decentralized ledger, transparent and free,
Bitcoin emerged, for the world to see.
Mining for coins, nodes in a network,
A financial system, no central clerk.

The world was skeptical, yet curiosity grew,
As Bitcoin's value steadily blew.
From pennies to thousands, a meteoric rise,
Satoshi's creation took us by surprise.

But Nakamoto vanished, into the digital mist,
Leaving behind a legacy, a cryptocurrency twist.
In the hearts of hodlers, Satoshi's name lives on,
A symbol of innovation, in the crypto dawn.
";
    let len_slice = content.as_bytes().len();
    let mut bytes = [0u8; 1047];

    for i in 0..len_slice {
        bytes[i] = content.as_bytes()[i];
    }

    let rq = rq_rounded_decode::<P, Q, Q12>(&bytes);
    let dec = rq_rounded_encode::<P, Q, Q12, ROUNDED_BYTES>(&rq);
    let utf8_string = std::str::from_utf8(&dec[..len_slice]).unwrap();

    assert_eq!(content, utf8_string);
}
