#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, W};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, W};

use crate::{
    math::nums::weightw_mask,
    poly::{f3::round, r3::R3, rq::Rq},
};

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

    hr
}
