use std::cmp::PartialOrd;
use std::ops::{Add, Div, Rem};

use num::FromPrimitive;

pub fn is_prime<T>(n: T) -> bool
where
    T: Copy
        + Div<Output = T>
        + Rem<Output = T>
        + Add<Output = T>
        + PartialOrd<T>
        + FromPrimitive
        + std::fmt::Debug,
{
    let one = T::from_u8(1).unwrap();
    let two = T::from_u8(2).unwrap();
    let zero = T::from_u8(0).unwrap();

    if n <= one {
        return false;
    }

    let value = n / two + one;

    let mut i = T::from_usize(2).unwrap();

    while i < value {
        if n % i == zero {
            return false;
        }

        i = i.add(one);
    }

    true
}

#[cfg(test)]
mod tests_prime {
    use super::is_prime;

    #[test]
    fn test_prime_numbers() {
        assert_eq!(is_prime::<u8>(2), true);
        assert_eq!(is_prime::<u8>(3), true);
        assert_eq!(is_prime::<u8>(5), true);
        assert_eq!(is_prime::<u8>(7), true);
        assert_eq!(is_prime::<u8>(11), true);

        assert_eq!(is_prime::<u16>(2), true);
        assert_eq!(is_prime::<u16>(3), true);
        assert_eq!(is_prime::<u16>(5), true);
        assert_eq!(is_prime::<u16>(7), true);
        assert_eq!(is_prime::<u16>(11), true);
    }

    #[test]
    fn test_non_prime_numbers() {
        assert_eq!(is_prime::<u8>(0), false);
        assert_eq!(is_prime::<u8>(1), false);
        assert_eq!(is_prime::<u8>(4), false);
        assert_eq!(is_prime::<u8>(6), false);
        assert_eq!(is_prime::<u8>(8), false);

        assert_eq!(is_prime::<u16>(0), false);
        assert_eq!(is_prime::<u16>(1), false);
        assert_eq!(is_prime::<u16>(4), false);
        assert_eq!(is_prime::<u16>(6), false);
        assert_eq!(is_prime::<u16>(8), false);
    }

    #[test]
    fn test_is_prime_u128() {
        // Проверка простых чисел типа u128
        assert!(is_prime(2u128));
        assert!(is_prime(3u128));
        assert!(is_prime(5u128));
        assert!(is_prime(7u128));
        assert!(is_prime(11u128));
        assert!(is_prime(13u128));
        assert!(is_prime(17u128));
        assert!(is_prime(19u128));
        assert!(is_prime(23u128));
        assert!(is_prime(29u128));
        assert!(is_prime(31u128));
        assert!(is_prime(107u128));
        assert!(is_prime(109u128));
        assert!(is_prime(113u128));
        assert!(is_prime(127u128));
        assert!(is_prime(131u128));
        assert!(is_prime(167u128));
        assert!(is_prime(199u128));
        assert!(is_prime(4099u128));
        assert!(is_prime(7919u128));
        assert!(is_prime(15485867u128));
    }

    #[test]
    fn test_is_prime_u64() {
        // Проверка простых чисел типа u64
        assert!(is_prime(2u64));
        assert!(is_prime(3u64));
        assert!(is_prime(5u64));
        assert!(is_prime(7u64));
        assert!(is_prime(11u64));
        assert!(is_prime(13u64));
        assert!(is_prime(17u64));
        assert!(is_prime(19u64));
        assert!(is_prime(23u64));
        assert!(is_prime(29u64));
        assert!(is_prime(31u64));
        assert!(is_prime(107u64));
        assert!(is_prime(109u64));
        assert!(is_prime(113u64));
        assert!(is_prime(127u64));
        assert!(is_prime(131u64));
        assert!(is_prime(167u64));
        assert!(is_prime(199u64));
        assert!(is_prime(4099u64));
        assert!(is_prime(7919u64));
        assert!(is_prime(15485867u64));
    }
}
