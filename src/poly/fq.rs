use crate::params::params::{Q, Q12};

use crate::math::nums::i32_mod_u14;

pub fn freeze(x: i32) -> i16 {
    let r = i32_mod_u14(x + Q12 as i32, Q as u16);

    r as i16 - Q12 as i16
}

pub fn recip(a1: i16) -> i16 {
    let mut i = 1;
    let mut ai = a1;

    while i < Q - 2 {
        ai = freeze((a1 as i32) * (ai as i32)) as i16;
        i += 1;
    }

    ai
}

#[cfg(test)]
mod tests_fq {
    use super::*;

    #[cfg(feature = "ntrup653")]
    const RQ_FREEZE: i32 = 58_078;
    #[cfg(feature = "ntrup761")]
    const RQ_FREEZE: i32 = 58_470;
    #[cfg(feature = "ntrup857")]
    const RQ_FREEZE: i32 = 51_949;
    #[cfg(feature = "ntrup953")]
    const RQ_FREEZE: i32 = 42_319;
    #[cfg(feature = "ntrup1013")]
    const RQ_FREEZE: i32 = 37_402;
    #[cfg(feature = "ntrup1277")]
    const RQ_FREEZE: i32 = 34_069;

    #[cfg(all(
        not(feature = "ntrup653"),
        not(feature = "ntrup761"),
        not(feature = "ntrup857"),
        not(feature = "ntrup953"),
        not(feature = "ntrup1013"),
        not(feature = "ntrup1277")
    ))]
    const RQ_FREEZE: i32 = 34_069;

    #[test]
    fn test_freeze() {
        fn test_freeze(a: i32) -> i16 {
            let mut b = a;
            let q = Q as i32;
            let rq = RQ_FREEZE;

            b -= q * ((228 * b) >> 20);
            b -= q * ((rq * b + 134_217_728) >> 28);

            b as i16
        }

        for n in 0..i16::MAX {
            let t1 = test_freeze(n as i32);
            let t2 = freeze(n as i32);

            assert_eq!(t2, t1);
        }
    }

    #[test]
    fn test_recip() {
        assert_eq!(recip(42), recip(-42) * -1);
        assert_eq!(recip(-42), recip(42) * -1);
    }
}
