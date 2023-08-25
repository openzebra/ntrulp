use crate::math::nums::i32_mod_u14;

#[derive(Debug)]
pub struct Fq<const P: usize> {
    coeffs: [i16; P],
}

pub fn freeze<const Q12: usize, const Q: usize>(x: i32) -> i16 {
    let r = i32_mod_u14(x + Q12 as i32, Q as u16);

    (r as i16).wrapping_sub(Q12 as i16)
}

#[test]
fn test_freeze() {
    use rand::prelude::*;

    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;

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
