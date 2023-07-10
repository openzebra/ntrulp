mod config;
mod math;
mod ntru;

#[cfg(test)]
mod tests {
    use std::dbg;

    use super::*;
    use math::finite_field::GF;
    use num::BigInt;

    #[test]
    fn it_works() {
        let (round1, p, q, w) = config::params::SNTRUP4591761;
        let hash_bytes: [u8; 0] = [];
        let mut usecache = !round1;

        assert!(math::prime::is_prime(p));
        assert!(math::prime::is_prime(q));
        assert!(w > 0);
        assert!(2 * p >= 3 * w);
        assert!(q >= 16 * w + 1);
        assert!(q % 6 == 1); // spec allows 5 but these tests do not
        assert!(p % 4 == 1); // spec allows 3 but ref C code does not

        if round1 {
            // encodings defined only for (761,4591)
            usecache = false;
            assert!(p == config::params::SNTRUP4591761.1);
            assert!(q == config::params::SNTRUP4591761.2);
        }

        let f3 = GF::new(1, 3);
        let q12 = BigInt::from((q - 1) / 2);
    }
}
