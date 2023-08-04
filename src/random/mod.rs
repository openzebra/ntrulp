use rand::prelude::*;
use std::io::Error;

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
        let part_size = n / 3;
        let chunk1 = vec![0i8; part_size];
        let chunk2 = vec![1i8; part_size * 2];
        let chunk3 = vec![-1i8; part_size * 2 - n];
        let rand_indices: Vec<u32> = (0..n).map(|_| self.random_u32()).collect();
        let mut coeffs: Vec<i8> = Vec::with_capacity(n);

        coeffs.extend(chunk1);
        coeffs.extend(chunk2);
        coeffs.extend(chunk3);

        let mut rand_idx = 0;

        for i in (1..n).rev() {
            let j = rand_indices[rand_idx] % (i + 1) as u32;

            coeffs.swap(i, j as usize);
            rand_idx += 1;
        }

        Ok(coeffs)
    }
}
