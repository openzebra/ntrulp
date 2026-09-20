use crate::params::params::{Q, Q12};

/// Reduces `x` into `(-Q12..Q12)` via a constant-modulus Barrett reduction.
///
/// Equivalent to `i32_mod_u14(x + Q12, Q) - Q12`: the `+Q12` shift inside the
/// reduction makes the result the centered representative of `x` mod `Q`.
/// The i64 intermediate keeps this exact for every `i32` input, and it is
/// far cheaper than the shift-subtract divmod loop it replaces.
pub fn freeze(x: i32) -> i16 {
    const K: u32 = 40;
    const M: i64 = (1i64 << K) / Q as i64;
    let q = Q as i64;

    let c = x as i64 + Q12 as i64;
    let mut b = c - q * ((c * M) >> K);
    b -= q & -((b >= q) as i64);
    b += q & -((b < 0) as i64);

    (b - Q12 as i64) as i16
}

/// Computes `a^(-1) mod q` via Fermat's little theorem using
/// square-and-multiply: O(log q) multiplications instead of O(q).
pub fn recip(a1: i16) -> i16 {
    let mut result: i16 = 1;
    let mut base = freeze(a1 as i32);
    let mut exponent = Q - 2;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = freeze(result as i32 * base as i32);
        }
        exponent >>= 1;
        if exponent > 0 {
            base = freeze(base as i32 * base as i32);
        }
    }

    result
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
        assert_eq!(recip(42), -recip(-42));
        assert_eq!(recip(-42), -recip(42));
    }
}
