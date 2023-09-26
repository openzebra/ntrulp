#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::P;
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::P;
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::P;
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::P;
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::P;
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::P;

use rand::prelude::*;

use rand::{Rng, SeedableRng};

pub fn shuffle_array<T>(arr: &mut [T; P], seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for i in 0..P {
        let j = rng.gen_range(0..P);

        arr.swap(i, j);
    }
}

pub fn unshuffle_array<T>(arr: &mut [T], seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut index_list = [0_usize; P];

    for i in 0..P {
        let delta = rng.gen_range(0..P);

        index_list[i] = delta;
    }

    for i in (0..P).rev() {
        let j = index_list[i];

        arr.swap(i, j);
    }
}

#[test]
fn test_shuffle_array() {
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; P];
    let seed = rng.gen::<u64>();

    rng.fill(&mut arr[..]);
    let origin_arr = arr;

    shuffle_array(&mut arr, seed);
    assert_ne!(origin_arr, arr);
    unshuffle_array(&mut arr, seed);
    assert_eq!(arr, origin_arr);
}
