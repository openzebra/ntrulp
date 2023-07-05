pub fn is_prime<T: Into<u128>>(n: T) -> bool {
    let grow_num: u128 = n.into();

    if grow_num <= 1 {
        return false;
    }

    for i in 2..(grow_num / 2 + 1) {
        if grow_num % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
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
}
