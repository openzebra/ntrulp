pub fn is_prime(n: usize) -> bool {
    let one = 1;
    let two = 2;
    let zero = 0;

    if n <= one {
        return false;
    }

    let value = n / two + one;
    let mut i = 2;

    while i < value {
        if n % i == zero {
            return false;
        }

        i = i + 1;
    }

    true
}

#[cfg(test)]
mod tests_prime {
    use super::is_prime;

    #[test]
    fn test_prime_numbers() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);

        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
    }

    #[test]
    fn test_non_prime_numbers() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);

        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
    }

    #[test]
    fn test_is_prime_1() {
        // Проверка простых чисел типа
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(29));
        assert!(is_prime(31));
        assert!(is_prime(107));
        assert!(is_prime(109));
        assert!(is_prime(113));
        assert!(is_prime(127));
        assert!(is_prime(131));
        assert!(is_prime(167));
        assert!(is_prime(199));
        assert!(is_prime(4099));
        assert!(is_prime(7919));
        assert!(is_prime(15485867));
    }

    #[test]
    fn test_is_prime_2() {
        // Проверка простых чисел типа
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(29));
        assert!(is_prime(31));
        assert!(is_prime(107));
        assert!(is_prime(109));
        assert!(is_prime(113));
        assert!(is_prime(127));
        assert!(is_prime(131));
        assert!(is_prime(167));
        assert!(is_prime(199));
        assert!(is_prime(4099));
        assert!(is_prime(7919));
        assert!(is_prime(15485867));
    }
}
