use crate::params::params::P;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[cfg(target_pointer_width = "64")]
#[inline]
fn gen_range_p_rand08_compat(rng: &mut ChaCha20Rng) -> usize {
    const RANGE: usize = P;
    const ZONE: usize = (RANGE << RANGE.leading_zeros()).wrapping_sub(1);
    loop {
        let v = rng.next_u64() as usize;
        let tmp = (v as u128).wrapping_mul(RANGE as u128);
        let hi = (tmp >> 64) as usize;
        let lo = tmp as usize;
        if lo <= ZONE {
            return hi;
        }
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn gen_range_p_rand08_compat(rng: &mut ChaCha20Rng) -> usize {
    const RANGE: usize = P;
    const ZONE: usize = (RANGE << RANGE.leading_zeros()).wrapping_sub(1);
    loop {
        let v = rng.next_u32() as usize;
        let tmp = (v as u64).wrapping_mul(RANGE as u64);
        let hi = (tmp >> 32) as usize;
        let lo = tmp as usize;
        if lo <= ZONE {
            return hi;
        }
    }
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("ntrulp shuffle only supports 32-bit and 64-bit targets (cipher compat with 0.2.3)");

pub fn shuffle_array<T>(arr: &mut [T; P], seed: u64) {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    for i in 0..P {
        let j = gen_range_p_rand08_compat(&mut rng);

        arr.swap(i, j);
    }
}

pub fn unshuffle_array<T>(arr: &mut [T], seed: u64) {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let index_list: [usize; P] =
        core::array::from_fn(|_| gen_range_p_rand08_compat(&mut rng));

    for (i, &j) in index_list.iter().enumerate().rev() {
        arr.swap(i, j);
    }
}

#[cfg(test)]
mod test_shuffle {
    use super::*;

    #[test]
    fn test_shuffle_array() {
        let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());
        let mut arr = [0u8; P];
        let seed: u64 = rng.next_u64();

        rng.fill_bytes(&mut arr);

        let origin_arr = arr;

        shuffle_array(&mut arr, seed);
        assert_ne!(origin_arr, arr);
        unshuffle_array(&mut arr, seed);
        assert_eq!(arr, origin_arr);
    }
}
