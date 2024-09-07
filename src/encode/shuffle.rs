use crate::params::params::P;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub type SeedType = [u8; 32];

pub fn shuffle_array<T>(arr: &mut [T; P], seed: &SeedType) {
    let mut rng = ChaCha20Rng::from_seed(*seed);

    for i in 0..P {
        let j = rng.gen_range(0..P);

        arr.swap(i, j);
    }
}

pub fn unshuffle_array<T>(arr: &mut [T], seed: &SeedType) {
    let mut rng = ChaCha20Rng::from_seed(*seed);
    let index_list: [usize; P] = core::array::from_fn(|_| rng.gen_range(0..P));

    for (i, &j) in index_list.iter().enumerate().rev() {
        arr.swap(i, j);
    }
}

#[cfg(test)]
mod test_shuffle {
    use rand::RngCore;

    use super::*;

    #[test]
    fn test_shuffle_array() {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut arr = [0u8; P];
        let mut seed = [0u8; 32];

        rng.fill_bytes(&mut arr);
        rng.fill_bytes(&mut seed);

        let origin_arr = arr;

        shuffle_array(&mut arr, &seed);
        assert_ne!(origin_arr, arr);
        unshuffle_array(&mut arr, &seed);
        assert_eq!(arr, origin_arr);
    }
}
