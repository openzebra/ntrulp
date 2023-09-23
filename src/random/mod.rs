#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, W};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, W};

use rand::prelude::*;

#[derive(Debug)]
pub enum RandomErrors {
    OverFlow,
    Mod2ShouldZero,
    Mod4ShouldOne,
    OutOfRange,
    SumShouldEqW,
}

pub trait CommonRandom {
    fn random_small(&mut self) -> Result<[i8; P], RandomErrors>;
    fn short_random(&mut self) -> Result<[i16; P], RandomErrors>;
    fn urandom32(&mut self) -> u32;
    fn random_range_3(&mut self) -> i8;
    fn randombytes<const SIZE: usize>(&mut self) -> [u8; SIZE];
    fn random_sign(&mut self) -> i8;
}

enum RngOptions {
    Thread(ThreadRng),
    Seed(StdRng),
}

impl RngOptions {
    pub fn gen_u8(&mut self) -> u8 {
        match self {
            RngOptions::Thread(thread_rng) => thread_rng.gen(),
            RngOptions::Seed(std_rng) => std_rng.gen(),
        }
    }

    pub fn randombytes(&mut self, bytes: &mut [u8]) {
        match self {
            RngOptions::Seed(rng) => {
                rng.fill(bytes);
            }
            RngOptions::Thread(rng) => {
                rng.fill(bytes);
            }
        }
    }

    pub fn random_bool(&mut self) -> bool {
        match self {
            RngOptions::Seed(rng) => rng.gen::<bool>(),
            RngOptions::Thread(rng) => rng.gen::<bool>(),
        }
    }
}

pub struct NTRURandom {
    rng: RngOptions,
}

impl NTRURandom {
    pub fn new() -> Self {
        let rng = thread_rng();

        NTRURandom {
            rng: RngOptions::Thread(rng),
        }
    }

    pub fn from(rng: ThreadRng) -> Self {
        NTRURandom {
            rng: RngOptions::Thread(rng),
        }
    }

    pub fn from_u64(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        NTRURandom {
            rng: RngOptions::Seed(rng),
        }
    }
}

// where
//     N: Copy + Zero + One + FromPrimitive,
impl CommonRandom for NTRURandom {
    fn urandom32(&mut self) -> u32 {
        let c0 = self.rng.gen_u8() as u32;
        let c1 = self.rng.gen_u8() as u32;
        let c2 = self.rng.gen_u8() as u32;
        let c3 = self.rng.gen_u8() as u32;

        c0 + 256 * c1 + 65536 * c2 + 16777216 * c3
    }

    fn random_sign(&mut self) -> i8 {
        if self.rng.random_bool() {
            1
        } else {
            -1
        }
    }

    fn random_range_3(&mut self) -> i8 {
        let r: u32 = self.urandom32();

        (((r & 0x3fffffff) * 3) >> 30) as i8 - 1
    }

    fn randombytes<const SIZE: usize>(&mut self) -> [u8; SIZE] {
        let mut bytes = [0u8; SIZE];

        self.rng.randombytes(&mut bytes);

        bytes
    }

    fn random_small(&mut self) -> Result<[i8; P], RandomErrors> {
        let mut r = [0i8; P];

        for i in 0..P {
            r[i] = self.random_range_3();
        }

        Ok(r)
    }

    fn short_random(&mut self) -> Result<[i16; P], RandomErrors> {
        let mut list = [0u32; P];

        for i in 0..P {
            let value = self.urandom32();

            if i < W {
                list[i] = value & !1
            } else {
                list[i] = (value & !3) | 1
            }
        }

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
            let new_value = element.rem_euclid(4) as i32 - 1;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed() {
        let mut random: NTRURandom = NTRURandom::from_u64(9999);
        let r = random.urandom32();

        assert!(r == 3688594871);
    }

    #[test]
    fn test_random_u32() {
        let mut random: NTRURandom = NTRURandom::new();

        let r = random.urandom32();

        assert!(r <= std::u32::MAX);
    }

    #[test]
    fn test_short_random() {
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..100 {
            let r = random.short_random().unwrap();
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
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..200 {
            let r = random.random_range_3();
            assert!(r <= 1 && r >= -1);
        }
    }

    #[test]
    fn test_small_random() {
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..100 {
            let r = random.random_small().unwrap();

            assert!(r.len() == P);
            assert!(r.contains(&-1) && r.contains(&0) && r.contains(&1));
        }
    }

    #[test]
    fn test_random_bytes() {
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..10 {
            let r = random.randombytes::<P>();

            assert!(r.len() == P);
        }
    }
}
