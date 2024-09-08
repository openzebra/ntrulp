use crate::params::params::{P, W};

use rand::{Rng, RngCore};

#[derive(Debug, PartialEq, Eq)]
pub enum RandomErrors {
    OverFlow,
    Mod2ShouldZero,
    Mod4ShouldOne,
    OutOfRange,
    SumShouldEqW,
}

/// Generates a 32-bit random unsigned integer using the provided random number generator.
///
/// This function takes an `RngCore` instance and generates a 32-bit unsigned integer
/// by combining four random 8-bit values. It converts each 8-bit value to a 32-bit value
/// and combines them to create the final 32-bit result.
///
/// # Arguments
///
/// * `rng` - A mutable reference to an `RngCore` instance, such as `rand::thread_rng()`
///
/// # Returns
///
/// A 32-bit unsigned integer generated using random values from the provided random number generator.
///
/// # Example
///
/// ```
/// use ntrulp::rng::urandom32;
///
/// let mut rng = rand::thread_rng();
/// let r = urandom32(&mut rng);
///
/// assert!(r <= std::u32::MAX);
/// ```
///
/// This function is a simple way to generate 32-bit random numbers using a random number generator.
pub fn urandom32<R: RngCore>(rng: &mut R) -> u32 {
    let c0 = rng.gen::<u8>() as u32;
    let c1 = rng.gen::<u8>() as u32;
    let c2 = rng.gen::<u8>() as u32;
    let c3 = rng.gen::<u8>() as u32;

    c0 + 256 * c1 + 65536 * c2 + 16777216 * c3
}

pub fn random_sign<R: RngCore>(rng: &mut R) -> i8 {
    if rng.gen::<bool>() {
        1
    } else {
        -1
    }
}

pub fn random_range_3<R: RngCore>(rng: &mut R) -> i8 {
    let r: u32 = urandom32(rng);

    (((r & 0x3fffffff) * 3) >> 30) as i8 - 1
}

pub fn random_small<R: RngCore>(rng: &mut R) -> [i8; P] {
    let mut r = [0i8; P];
    r.iter_mut().for_each(|x| *x = random_range_3(rng));
    r
}

pub fn short_random<R: RngCore>(rng: &mut R) -> Result<[i16; P], RandomErrors> {
    let mut list: [u32; P] = core::array::from_fn(|i| {
        let value = urandom32(rng);
        if i < W {
            value & !1
        } else {
            (value & !3) | 1
        }
    });

    if !list.iter().take(W).all(|&value| value % 2 == 0) {
        return Err(RandomErrors::Mod2ShouldZero);
    }
    if !list.iter().skip(W).all(|&value| value % 4 == 1) {
        return Err(RandomErrors::Mod4ShouldOne);
    }

    list.sort();

    let mut new_list = [0i32; P];
    let mut sum = 0;

    for i in 0..P {
        new_list[i] = list[i] as i32;
    }

    for element in new_list.iter_mut() {
        let new_value = element.rem_euclid(4) - 1;

        if new_value > 1 {
            return Err(RandomErrors::OutOfRange);
        }

        *element = new_value;

        sum += new_value.abs();
    }

    if sum as usize != W {
        return Err(RandomErrors::SumShouldEqW);
    }

    let mut i16_list = [0i16; P];

    for i in 0..P {
        i16_list[i] = new_list[i] as i16;
    }

    Ok(i16_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_short_random() {
        let mut rng = ChaCha20Rng::from_entropy();

        for _ in 0..100 {
            let r = short_random(&mut rng).unwrap();
            let mut sum = 0;

            assert!(r.len() == P);
            assert!(r.contains(&-1) && r.contains(&0) && r.contains(&1));

            for el in r {
                sum += el.abs();
            }

            assert_eq!(sum as usize, W);
        }
    }

    #[test]
    fn test_random_range_3() {
        let mut rng = ChaCha20Rng::from_entropy();

        for _ in 0..200 {
            let r = random_range_3(&mut rng);
            assert!((-1..=1).contains(&r));
        }
    }

    #[test]
    fn test_small_random() {
        let mut rng = ChaCha20Rng::from_entropy();

        for _ in 0..100 {
            let r = random_small(&mut rng);

            assert!(r.len() == P);
            assert!(r.contains(&-1) && r.contains(&0) && r.contains(&1));
        }
    }
}
