use std::cmp::PartialOrd;
use std::ops::{Add, Mul, Sub};

use num::traits::Euclid;
use num::FromPrimitive;

#[derive(Debug, PartialEq)]
pub struct GF<T> {
    pub v: T,
    pub p: T,
}

impl<T> GF<T>
where
    T: Copy
        + Euclid
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + PartialOrd<T>
        + FromPrimitive,
{
    pub fn new(value: T, p: T) -> Self {
        GF {
            v: value.rem_euclid(&p),
            p,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        GF::new(self.v.add(other.v), self.p)
    }

    pub fn sub(&self, other: Self) -> Self {
        let result = if self.v >= other.v {
            self.v - other.v
        } else {
            self.v + (self.p - other.v)
        };

        GF::new(result, self.p)
    }

    pub fn mul(&self, other: Self) -> Self {
        GF::new(self.v * other.v, self.p)
    }

    pub fn has(&self, n: T) -> bool {
        self.p > n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a: GF<u32> = GF::new(2, 5);
        let b: GF<u32> = GF::new(3, 5);
        let result = a.add(b);

        assert_eq!(result, GF::new(0, 5));
    }

    #[test]
    fn test_subtraction() {
        let a: GF<i64> = GF::new(2, 5);
        let b: GF<i64> = GF::new(3, 5);
        let result = a.sub(b);

        assert_eq!(result, GF::new(4, 5));

        let p = 654;
        let a: GF<i16> = GF::new(100, p);
        let b: GF<i16> = GF::new(200, p);
        let result = a.sub(b);

        assert_eq!(result, GF::new(554, p));
    }

    #[test]
    fn test_multiplication() {
        let a: GF<u8> = GF::new(2, 5);
        let b = GF::new(3, 5);
        let result = a.mul(b);

        assert_eq!(result, GF::new(1, 5));

        let a: GF<u64> = GF::new(43543, 934);
        let b: GF<u64> = GF::new(94239, 934);
        let result = a.mul(b);

        assert_eq!(result, GF::new(101, 934));
    }

    #[test]
    fn test_has() {
        let gf_654: GF<u16> = GF::new(1, 654);

        assert!(gf_654.has(100));
        assert!(!gf_654.has(656));

        let ff3: GF<u16> = GF::new(1, 3);

        assert!(ff3.has(0));
        assert!(!ff3.has(3));

        let gf = GF::new(5, 10);
        assert!(gf.has(0));
        assert!(gf.has(4));
        assert!(!gf.has(14));
        assert!(!gf.has(10));
    }

    #[test]
    fn test_new() {
        let gf = GF::new(5, 10);
        assert_eq!(gf.v, 5);
        assert_eq!(gf.p, 10);
    }

    #[test]
    fn test_add() {
        let gf1 = GF::new(5, 10);
        let gf2 = GF::new(8, 10);

        let result = gf1.add(gf2);
        assert_eq!(result.v, 3);
        assert_eq!(result.p, 10);
    }

    #[test]
    fn test_sub() {
        let gf1 = GF::new(5, 10);
        let gf2 = GF::new(8, 10);

        let result = gf1.sub(gf2);
        assert_eq!(result.v, 7);
        assert_eq!(result.p, 10);
    }

    #[test]
    fn test_sub_overflow() {
        let gf1: GF<u8> = GF::new(1, 3);
        let gf2: GF<u8> = GF::new(2, 3);

        let result = gf1.sub(gf2);

        assert!(result == GF::new(2, 3));
    }
}
