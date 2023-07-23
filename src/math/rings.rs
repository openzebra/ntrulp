use num::traits::Euclid;
use num::FromPrimitive;
use std::cmp::PartialOrd;
use std::io::{Error, ErrorKind};
use std::ops::Mul;
use std::ops::{Add, Sub};

use crate::math::finite_field::GF;

pub fn zz_from_ff<T>(c: T, ff: &GF<T>) -> Result<T, Error>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + PartialOrd
        + Euclid
        + FromPrimitive,
{
    if ff.has(c) {
        // TODO: //refactor
        let one = T::from_u8(1).unwrap();

        Ok(c.add(one).sub(one))
    } else {
        let custom_error = Error::new(ErrorKind::Other, "Element must be in GF(ff)");

        Err(custom_error)
    }
}

pub fn zz_from_fq<T>(q: T, q12: T, c: T, fq: &GF<T>) -> Result<T, Error>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + PartialOrd
        + Euclid
        + Mul<Output = T>
        + FromPrimitive,
{
    // TODO: Refactor
    if c < q && fq.has(c) {
        let sum = c.add(q12);

        Ok(sum.sub(q12))
    } else {
        let custom_error = Error::new(ErrorKind::Other, "Element must be in Fq");

        Err(custom_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zz_from_ff() {
        let ff3 = GF::new(0, 3);
        let c = 2;
        let result = zz_from_ff(c, &ff3).unwrap();

        assert!(result == 2);
    }

    #[test]
    fn test_err_zz_from_ff() {
        let ff3 = GF::new(0, 3);
        let c = 3;
        let result = zz_from_ff(c, &ff3);

        assert!(result.is_err());

        let c = 4;
        let result = zz_from_ff(c, &ff3);

        assert!(result.is_err());
    }

    #[test]
    fn test_zz_from_fq() {
        let q = 4591;
        let q12 = (q - 1) / 2;
        let c = 239;
        let fq: GF<u64> = GF::new(0, q);
        let res = zz_from_fq(q, q12, c, &fq);

        assert!(res.unwrap() == c);
    }

    #[test]
    fn test_err_zz_from_fq() {
        let q = 4591;
        let q12 = (q - 1) / 2;
        let c = q + 1;
        let fq: GF<u64> = GF::new(0, q);
        let res = zz_from_fq(q, q12, c, &fq);

        assert!(res.is_err());
    }
}
