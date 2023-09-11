use crate::{
    kem::{f3::round, r3::R3, rq::Rq},
    math::nums::weightw_mask,
};

pub fn rq_decrypt<
    const P: usize,
    const Q: usize,
    const W: usize,
    const Q12: usize,
    const P_TWICE_MINUS_ONE: usize,
>(
    c: &Rq<P, Q, Q12>,
    f: &Rq<P, Q, Q12>,
    ginv: &R3<P, Q, Q12>,
) -> R3<P, Q, Q12> {
    let mut r = [0i8; P];
    let cf: Rq<P, Q, Q12> = c.mult_r3::<P_TWICE_MINUS_ONE>(&f.r3_from_rq());
    let cf3: Rq<P, Q, Q12> = cf.mult3();
    let e: R3<P, Q, Q12> = cf3.r3_from_rq();
    let ev: R3<P, Q, Q12> = e.mult(&ginv);
    #[allow(unused_assignments)]
    let mut mask: i16 = 0;

    mask = weightw_mask::<P, W>(&ev.coeffs); // 0 if weight w, else -1

    for i in 0..W {
        r[i] = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
    }

    for i in W..P {
        r[i] = (ev.coeffs[i] as i16 & !mask) as i8;
    }

    R3::from(r)
}

pub fn r3_encrypt<
    const P: usize,
    const Q: usize,
    const Q12: usize,
    const P_TWICE_MINUS_ONE: usize,
>(
    r: &R3<P, Q, Q12>,
    h: &Rq<P, Q, Q12>,
) -> Rq<P, Q, Q12> {
    let mut hr = h.mult_r3::<P_TWICE_MINUS_ONE>(&r);

    round(&mut hr.coeffs);

    Rq::from(hr.coeffs)
}
