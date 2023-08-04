use rand::prelude::*;
use std::io::{Error, ErrorKind};

pub trait CommonRandom {
    fn random_small_vec(&mut self, n: usize) -> Vec<i8>;
    fn small_fisher_yates_shuffle(&mut self, n: usize) -> Result<Vec<i8>, Error>;
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

    pub fn from_u64(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        NTRURandom {
            rng: RngOptions::Seed(rng),
        }
    }
}

impl CommonRandom for NTRURandom {
    fn random_u32(&mut self) -> u32 {
        let c0 = self.rng.gen_u8() as u32;
        let c1 = self.rng.gen_u8() as u32;
        let c2 = self.rng.gen_u8() as u32;
        let c3 = self.rng.gen_u8() as u32;

        c0 + 256 * c1 + 65536 * c2 + 16777216 * c3
    }

    fn random_range_3(&mut self) -> u8 {
        let r: u32 = <NTRURandom as CommonRandom>::random_u32(self);

        (((r & 0x3fffffff) * 3) >> 30) as u8
    }

    fn random_small_vec(&mut self, n: usize) -> Vec<i8> {
        let r: Vec<i8> = vec![0u8; n]
            .iter_mut()
            .map(|_| (<NTRURandom as CommonRandom>::random_range_3(self)) as i8 - 1)
            .collect();

        r
    }

    fn small_fisher_yates_shuffle(&mut self, n: usize) -> Result<Vec<i8>, Error> {
        if n < 9 {
            return Err(Error::new(ErrorKind::Other, "n should be >= 9"));
        }

        let total_chunks = 3;
        let part_size = n / total_chunks;
        let remainder = n % total_chunks;
        let chunk1 = vec![0i8; part_size];
        let chunk2 = vec![1i8; part_size];
        let chunk3 = vec![-1i8; part_size];
        let rand_indices: Vec<u32> = (0..n).map(|_| self.random_u32()).collect();
        let mut coeffs: Vec<i8> = Vec::with_capacity(n);

        coeffs.extend(chunk1);
        coeffs.extend(chunk2);
        coeffs.extend(chunk3);

        for _ in 0..remainder {
            coeffs.push((self.random_range_3() as i8) - 1);
        }

        let mut rand_idx = 0;

        for i in (1..n).rev() {
            let j = rand_indices[rand_idx] % (i + 1) as u32;

            coeffs.swap(i, j as usize);
            rand_idx += 1;
        }

        Ok(coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_u32() {
        let mut random = NTRURandom::new();

        let r = random.random_u32();

        assert!(r <= std::u32::MAX)
    }

    #[test]
    fn test_random_small_vec() {
        let mut random = NTRURandom::new();

        let r = random.random_small_vec(9000);

        assert!(r.len() == 9000)
    }

    #[test]
    fn test_random_range_3() {
        let mut random = NTRURandom::new();
        let r = random.random_range_3();

        assert!(r <= 2);
    }

    #[test]
    fn test_small_fisher_yates_shuffle() {
        for size in 100..1000 {
            let mut random = NTRURandom::new();
            let r = random.small_fisher_yates_shuffle(size);

            assert!(r.is_ok());

            let r = r.unwrap();

            assert!(r.len() == size);
            assert!(r.contains(&1));
            assert!(r.contains(&-1));
            assert!(r.contains(&0));
        }
    }
}
