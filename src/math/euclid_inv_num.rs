use num::CheckedSub;
use num::{FromPrimitive, One, Zero};
use std::cmp::PartialOrd;
use std::ops::{AddAssign, Div, Mul, Rem};

// a * x ≡ 1 (mod modulo)
pub fn euclid_num_mod_inverse<T>(mut a: T, modulus: T) -> T
where
    T: Copy
        + One
        + Zero
        + PartialOrd<T>
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + CheckedSub
        + FromPrimitive
        + AddAssign,
{
    let zero = T::from_u8(0).unwrap();
    let mut x = T::zero();
    let mut lastx = T::one();
    let mut y = T::one();
    let mut lasty = T::zero();
    let mut b = modulus;

    while b != T::zero() {
        let quotient = a / b;

        let temp = a;
        a = b;
        b = temp % b;

        let temp = x;
        x = lastx.checked_sub(&(quotient * x)).unwrap_or(zero);
        lastx = temp;

        let temp = y;
        y = lasty.checked_sub(&(quotient * y)).unwrap_or(zero);
        lasty = temp;
    }

    if lastx < T::zero() {
        lastx += modulus;
    }

    lastx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclid_num_mod_inverse() {
        assert_eq!(euclid_num_mod_inverse(7175, 9829), 2885); // 7175 * 2885 ≡ 1 (mod 9829)
        assert_eq!(euclid_num_mod_inverse(2, 5), 3); // 2 * 3 ≡ 1 (mod 5)
        assert_eq!(euclid_num_mod_inverse(3, 7), 5); // 3 * 5 ≡ 1 (mod 7)
        assert_eq!(euclid_num_mod_inverse(4, 11), 3); // 4 * 3 ≡ 1 (mod 11)
        assert_eq!(euclid_num_mod_inverse(5, 17), 7); // 5 * 7 ≡ 1 (mod 17)
    }
}
