use num::{FromPrimitive, One, Zero};
use rand::prelude::*;

#[derive(Debug)]
pub enum RandomErrors {
    OverFlow,
    Mod2ShouldZero,
    Mod4ShouldOne,
    OutOfRange,
    SumShouldEqW,
}

pub trait CommonRandom<const SIZE: usize> {
    fn random_small_vec<N: Copy + Zero + One + FromPrimitive>(
        &mut self,
    ) -> Result<[N; SIZE], RandomErrors>;
    fn short_random<N: Copy + Zero + One + FromPrimitive>(
        &mut self,
        w: usize,
    ) -> Result<[N; SIZE], RandomErrors>;
    fn random_u32(&mut self) -> u32;
    fn random_range_3(&mut self) -> u8;
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
}

pub struct NTRURandom<const SIZE: usize> {
    rng: RngOptions,
}

impl<const SIZE: usize> NTRURandom<SIZE> {
    pub fn new() -> Self {
        let rng = thread_rng();

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
impl<const SIZE: usize> CommonRandom<SIZE> for NTRURandom<SIZE> {
    fn random_u32(&mut self) -> u32 {
        let c0 = self.rng.gen_u8() as u32;
        let c1 = self.rng.gen_u8() as u32;
        let c2 = self.rng.gen_u8() as u32;
        let c3 = self.rng.gen_u8() as u32;

        c0 + 256 * c1 + 65536 * c2 + 16777216 * c3
    }

    fn random_range_3(&mut self) -> u8 {
        let r: u32 = <NTRURandom<SIZE> as CommonRandom<SIZE>>::random_u32(self);

        (((r & 0x3fffffff) * 3) >> 30) as u8
    }

    fn random_small_vec<N: Copy + Zero + One + FromPrimitive>(
        &mut self,
    ) -> Result<[N; SIZE], RandomErrors> {
        let mut r = [N::zero(); SIZE];

        for i in 0..SIZE {
            let r3 = <NTRURandom<SIZE> as CommonRandom<SIZE>>::random_range_3(self);

            r[i] = N::from_u8(r3).ok_or(RandomErrors::OverFlow)?;
        }

        Ok(r)
    }

    fn short_random<N: Copy + Zero + One + FromPrimitive>(
        &mut self,
        w: usize,
    ) -> Result<[N; SIZE], RandomErrors> {
        let mut list = [0u32; SIZE];

        for i in 0..SIZE {
            let value = <NTRURandom<SIZE> as CommonRandom<SIZE>>::random_u32(self);

            if i < w {
                list[i] = value & !1
            } else {
                list[i] = (value & !3) | 1
            }
        }

        if !list.iter().take(w).all(|&value| value % 2 == 0) {
            return Err(RandomErrors::Mod2ShouldZero);
        }
        if !list.iter().skip(w).all(|&value| value % 4 == 1) {
            return Err(RandomErrors::Mod4ShouldOne);
        }

        list.sort();

        let mut new_list = [0i32; SIZE];

        for i in 0..SIZE {
            new_list[i] = list[i] as i32;
        }

        drop(list);

        let mut sum = 0;

        for element in new_list.iter_mut() {
            let new_value = element.rem_euclid(4) as i32 - 1;

            if new_value > 1 {
                return Err(RandomErrors::OutOfRange);
            }

            *element = new_value;

            sum += new_value.abs();
        }

        if sum as usize != w {
            return Err(RandomErrors::SumShouldEqW);
        }

        let mut u8_list = [N::zero(); SIZE];

        for i in 0..SIZE {
            u8_list[i] = N::from_i32(new_list[i] + 1).ok_or(RandomErrors::OverFlow)?;
        }

        drop(new_list);

        Ok(u8_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::params::SNTRP_761;

    #[test]
    fn test_seed() {
        const SIZE: usize = 716;
        let mut random: NTRURandom<SIZE> = NTRURandom::from_u64(9999);
        let r = random.random_u32();

        assert!(r == 3688594871);
    }

    #[test]
    fn test_random_u32() {
        const SIZE: usize = 0;
        let mut random: NTRURandom<SIZE> = NTRURandom::new();

        let r = random.random_u32();

        assert!(r <= std::u32::MAX)
    }

    #[test]
    fn test_random_small_vec() {
        const SIZE: usize = 9000;
        let mut random: NTRURandom<SIZE> = NTRURandom::new();

        let r = random.random_small_vec::<u8>().unwrap();

        assert!(r.len() == 9000)
    }

    #[test]
    fn test_random_range_3() {
        const SIZE: usize = 0;
        let mut random: NTRURandom<SIZE> = NTRURandom::new();
        let r = random.random_range_3();

        assert!(r <= 2);
    }

    #[test]
    fn test_shot_random() {
        const SIZE: usize = 761;
        let mut random: NTRURandom<SIZE> = NTRURandom::new();
        let values = random.short_random::<u8>(SIZE);

        assert!(values.is_ok());

        let values = values.unwrap();

        assert!(values.len() == SNTRP_761.p);
    }
}
