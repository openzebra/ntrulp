use crate::math::nums::i32_mod_u14;

pub fn freeze<const Q12: usize, const Q: usize>(x: i32) -> i16 {
    let r = i32_mod_u14(x + Q12 as i32, Q as u16);

    r as i16 - Q12 as i16
}

pub fn recip<const Q12: usize, const Q: usize>(a1: i16) -> i16 {
    let mut i = 1;
    let mut ai = a1;

    while i < Q - 2 {
        ai = freeze::<Q12, Q>((a1 as i32) * (ai as i32)) as i16;
        i += 1;
    }

    ai
}

#[cfg(test)]
mod tests_fq {
    use super::*;

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

    #[test]
    fn test_recip() {
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;

        assert_eq!(recip::<Q12, Q>(42), -1421);
        assert_eq!(recip::<Q12, Q>(-42), 1421);
    }
}
