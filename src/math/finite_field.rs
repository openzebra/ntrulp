#[derive(Debug, PartialEq)]
pub struct GF(i64, i64);

impl GF {
    pub fn new(value: i64, p: i64) -> Self {
        GF(value.rem_euclid(p), p)
    }

    pub fn add(&self, other: Self) -> Self {
        GF::new(self.0.wrapping_add(other.0), self.1)
    }

    pub fn sub(&self, other: Self) -> Self {
        GF::new(self.0.wrapping_sub(other.0), self.1)
    }

    pub fn mul(&self, other: Self) -> Self {
        GF::new(self.0.wrapping_mul(other.0), self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = GF::new(2, 5);
        let b = GF::new(3, 5);
        let result = a.add(b);

        assert_eq!(result, GF::new(0, 5));
    }

    #[test]
    fn test_subtraction() {
        let a = GF::new(2, 5);
        let b = GF::new(3, 5);
        let result = a.sub(b);

        assert_eq!(result, GF::new(4, 5));

        let p = 654;
        let a = GF::new(100, p);
        let b = GF::new(200, p);
        let result = a.sub(b);

        assert_eq!(result, GF::new(554, p));
    }

    #[test]
    fn test_multiplication() {
        let a = GF::new(2, 5);
        let b = GF::new(3, 5);
        let result = a.mul(b);

        assert_eq!(result, GF::new(1, 5));

        let a = GF::new(43543, 934);
        let b = GF::new(94239, 934);
        let result = a.mul(b);

        assert_eq!(result, GF::new(101, 934));
    }
}
