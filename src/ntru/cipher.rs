#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{
    CIPHERTEXTS_BYTES, HASH_BYTES, I, P, PUBLICKEYS_BYTES, ROUNDED_BYTES, SEEDS_BYTES, W,
};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, PUBLICKEYS_BYTES, ROUNDED_BYTES, W};

use crate::{
    encode::{rq, top::top_encode},
    math::nums::weightw_mask,
    poly::{f3::round, r3::R3, rq::Rq},
};

use super::{errors::NTRUErrors, lpr::generator};

pub fn rq_decrypt(c: &Rq, f: &Rq, ginv: &R3) -> R3 {
    let mut r = [0i8; P];
    let cf: Rq = c.mult_r3(&f.r3_from_rq());
    let cf3: Rq = cf.mult3();
    let e: R3 = cf3.r3_from_rq();
    let ev: R3 = e.mult(&ginv);
    #[allow(unused_assignments)]
    let mut mask: i16 = 0;

    mask = weightw_mask(&ev.coeffs); // 0 if weight w, else -1

    for i in 0..W {
        r[i] = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
    }

    for i in W..P {
        r[i] = (ev.coeffs[i] as i16 & !mask) as i8;
    }

    R3::from(r)
}

pub fn r3_encrypt(r: &R3, h: &Rq) -> Rq {
    let mut hr = h.mult_r3(&r);

    round(&mut hr.coeffs);

    Rq::from(hr.coeffs)
}

pub fn x_encrypt(
    b: &mut [i16; P],
    t: &mut [i8; I],
    r: &R3,
    pk_seed: &[u8; SEEDS_BYTES],
    a: &[i16; P],
) {
    let g = generator(pk_seed);
    let b = [0i8; P];
}

pub fn z_encrypt(
    r: &R3,
    pk: &[u8; PUBLICKEYS_BYTES],
) -> Result<[u8; CIPHERTEXTS_BYTES + HASH_BYTES], NTRUErrors<'static>> {
    let pk_seed_slice: [u8; SEEDS_BYTES] = match &pk[..SEEDS_BYTES].try_into() {
        Ok(s) => *s,
        Err(_) => return Err(NTRUErrors::PubKey("Incorrect PubKey Seed")),
    };
    let pk_slice: [u8; ROUNDED_BYTES] = match &pk[SEEDS_BYTES..].try_into() {
        Ok(s) => *s,
        Err(_) => return Err(NTRUErrors::PubKey("Incorrect PubKey")),
    };
    let a = rq::rq_rounded_decode(&pk_slice);
    let mut b = [0i16; P];
    let mut t = [0i8; I];
    let mut out = [0u8; CIPHERTEXTS_BYTES + HASH_BYTES];

    x_encrypt(&mut b, &mut t, &r, &pk_seed_slice, &a);
    out[ROUNDED_BYTES..].copy_from_slice(&rq::rq_rounded_encode(&b)[..]);
    top_encode(&mut out, &t);

    Ok(out)
}
