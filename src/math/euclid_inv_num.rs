use num::traits::Euclid;
use num::{CheckedSub, FromPrimitive};
use std::cmp::PartialOrd;
use std::ops::{AddAssign, Div, Mul};

// a * x ≡ 1 (mod modulo)
pub fn euclid_num_mod_inverse<T>(a: T, modulus: T) -> T
where
    T: Copy
        + Euclid
        + PartialOrd<T>
        + Div<Output = T>
        + CheckedSub
        + Mul<Output = T>
        + FromPrimitive
        + AddAssign
        + std::fmt::Debug,
{
    let zero = T::from_u8(0).unwrap();
    let mut x = zero;
    let mut y = T::from_u8(1).unwrap();
    let mut last_x = T::from_u8(1).unwrap();
    let mut last_y = zero;
    let mut a = a;
    let mut b = modulus;
    let zero = T::from_u8(0).unwrap();

    while b != zero {
        let quotient = a / b;
        let remainder = a.rem_euclid(&b);

        a = b;
        b = remainder;

        let tmp = x;

        x = last_x.checked_sub(&(quotient * x)).unwrap_or(zero);
        last_x = tmp;

        let tmp = y;
        y = last_y.checked_sub(&(quotient * y)).unwrap_or(zero);
        last_y = tmp;
    }

    if last_x < zero {
        last_x += modulus;
    }

    last_x
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
        assert_eq!(euclid_num_mod_inverse(-1, 3), 2); //  2 * (-1) = -2 ≡ 1 (mod 3)
    }
}
