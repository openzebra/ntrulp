use crate::math::nums::int32_mod_uint14;

#[derive(Debug)]
pub struct Fq<const P: usize> {
    coeffs: [i16; P],
}

pub fn freeze<const Q12: u16, const Q: u16>(x: i32) -> i16 {
    let r = int32_mod_uint14(x + Q12 as i32, Q as u16);

    (r as i16).wrapping_sub(Q12 as i16)
}

#[test]
fn test_freeze() {
    use rand::prelude::*;

    const Q: u16 = 4591;
    const Q12: u16 = (Q - 1) / 2;

    let mut rng = thread_rng();

    fn test_freeze(a: i32) -> i16 {
        let mut b = a;

        b -= 4_591 * ((228 * b) >> 20);
        b -= 4_591 * ((58_470 * b + 134_217_728) >> 28);

        b as i16
    }

    for _ in 0..1000 {
        let r = rng.gen::<i16>() as i32;

        let t1 = test_freeze(r);
        let t2 = freeze::<Q12, Q>(r);

        assert_eq!(t2, t1);
    }
}
