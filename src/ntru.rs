use std::ops::Add;
use std::println;

use crate::config;
use crate::config::params::StartParams;
use crate::math;
use crate::math::finite_field::GF;

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

    pub fn zz_from_ff(&self, c: i64, ff: &GF) -> i64 {
        assert!(ff.has(c), "Element must be in GF(ff)");
        let pff = ff.1;

        GF::new(c.add(1), pff).0 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zz_from_ff() {
        let ntru = NTRU::from(config::params::SNTRUP4591761);

        assert!(
            ntru.zz_from_ff(0, &GF::new(3, 3)) == 0,
            "ZZ(0) from GF(3) should be 0"
        );
        assert!(
            ntru.zz_from_ff(100, &GF::new(100, 100)) == 100,
            "ZZ(100) from GF(3) should be 100"
        );
        assert!(
            ntru.zz_from_ff(100, &GF::new(999, 999)) == 100,
            "ZZ(100) from GF(3) should be 100"
        );
    }
}
