use std::println;

use crate::config;
use crate::config::params::StartParams;
use crate::math;
use crate::math::finite_field;

pub struct NTRU {
    params: StartParams,
    hash_bytes: Vec<u8>, // TODO: change to ARC<[u8]>
    usecache: bool,
}

pub enum NTRUErrors {}

impl NTRU {
    pub fn from(params: StartParams) -> Self {
        let (round1, p, q, w) = params;
        let hash_bytes = vec![];
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

        NTRU {
            params,
            hash_bytes,
            usecache,
        }
    }

    pub fn zz_from_ff(&self, c: i64, ff: finite_field::GF) -> i32 {
        println!("{}, {:?}", c, ff);
        assert!(ff.has(c), "Element must be in GF(ff)");
        let pff = ff.1;

        println!("{}", pff);

        // let transformed_c = math::modulo::add_by_modulo(c, m3);

        // transformed_c as i32 - 1
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zz_from_ff() {
        let ntru = NTRU::from(config::params::SNTRUP4591761);
        let f3 = finite_field::GF::new(0, 3);
        let result = ntru.zz_from_ff(0, f3);
    }
}
