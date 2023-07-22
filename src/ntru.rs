use crate::config;
use crate::config::params::StartParams;
use crate::math;
use crate::math::finite_field::GF;

pub struct NTRU {
    pub params: StartParams,
    hash_bytes: Vec<u8>, // TODO: change to ARC<[u8]>
    usecache: bool,
    fq: GF<u64>,
    q12: u16,
}

pub enum NTRUErrors {}

impl NTRU {
    pub fn from(params: StartParams) -> Self {
        let (round1, p, q, w) = params;
        let hash_bytes = vec![];
        let mut usecache = !round1;

        // assert!(math::prime::is_prime(p));
        // assert!(math::prime::is_prime(q));
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

        let fq = GF::new(1, q as u64);
        let q12 = (q - 1) / 2;

        NTRU {
            params,
            hash_bytes,
            usecache,
            fq,
            q12,
        }
    }

    pub fn zz_from_ff(&self, c: u64, ff: &GF<u64>) -> u64 {
        assert!(ff.has(c), "Element must be in GF(ff)");

        let pff = ff.p;
        let value = GF::new(c.wrapping_add(1), pff).v;

        value.wrapping_sub(value)
    }

    pub fn zz_from_fq(&self, c: u64) -> u64 {
        let (_, _, q, _) = self.params;
        let q = q as u64;

        assert!(c < q, "Element must be in Fq");

        let q12 = self.q12 as u64;
        let sum = c.wrapping_add(q12);

        sum.wrapping_sub(q12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //
    // #[test]
    // fn zz_from_ff() {
    //     let ntru = NTRU::from(config::params::SNTRUP4591761);
    //
    //     assert!(
    //         ntru.zz_from_ff(0, &GF::new(3, 3)) == 0,
    //         "ZZ(0) from GF(3) should be 0"
    //     );
    //     assert!(
    //         ntru.zz_from_ff(100, &GF::new(100, 100)) == 100,
    //         "ZZ(100) from GF(3) should be 100"
    //     );
    //     assert!(
    //         ntru.zz_from_ff(100, &GF::new(999, 999)) == 100,
    //         "ZZ(100) from GF(3) should be 100"
    //     );
    // }
}
