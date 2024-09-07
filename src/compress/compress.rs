use rand::RngCore;

use crate::{
    encode::shuffle::{shuffle_array, unshuffle_array},
    params::{
        params::P,
        params1277::{DIFFICULT, W},
    },
    rng::random_sign,
};

pub const BITS_SIZE: usize = 6;
const EMPTY: [i8; BITS_SIZE] = [0i8; BITS_SIZE];

pub fn convert_to_ternary(num: u8) -> [i8; BITS_SIZE] {
    let mut result = [0i8; BITS_SIZE];
    let mut n = num;

    for i in (0..BITS_SIZE).rev() {
        let digit = n % 3;
        result[i] = match digit {
            0 => 0,
            1 => 1,
            2 => -1,
            _ => unreachable!(),
        };
        n /= 3;
    }

    result
}

pub fn convert_to_decimal(ternary: [i8; BITS_SIZE]) -> u8 {
    let mut result = 0i16;

    for &digit in &ternary {
        let x = match digit {
            0 => 0,
            1 => 1,
            -1 => 2,
            _ => unreachable!(),
        };

        result = result * 3 + x as i16;
    }

    result as u8
}

pub fn r3_encode_chunks(r3: &[i8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for chunk in r3.chunks(BITS_SIZE) {
        let bits: [i8; BITS_SIZE] = chunk.try_into().unwrap_or(EMPTY);
        let byte = convert_to_decimal(bits);

        output.push(byte);
    }

    output
}

pub fn r3_decode_chunks(bytes: &[u8]) -> Vec<i8> {
    let mut output: Vec<i8> = Vec::new();

    for byte in bytes {
        let bits = convert_to_ternary(*byte);

        output.extend(bits);
    }

    output
}

pub fn r3_merge_w_chunks(chunks: &[[i8; P]], size: &[usize], seed: u64) -> Vec<i8> {
    let mut out = Vec::new();

    for (index, chunk) in chunks.iter().enumerate() {
        let seed = seed + index as u64;
        let point = size[index];
        let mut part: [i8; P] = *chunk;

        unshuffle_array::<i8>(&mut part, seed);
        out.extend_from_slice(&part[..point]);
    }

    out
}

pub fn r3_split_w_chunks<R: RngCore>(input: &[i8], rng: &mut R) -> (Vec<[i8; P]>, Vec<usize>, u64) {
    const LIMIT: usize = W - DIFFICULT;

    let origin_seed: u64 = rng.next_u64() - (input.len() / P) as u64;
    let mut seed = origin_seed;
    let mut chunks: Vec<[i8; P]> = Vec::new();
    let mut size: Vec<usize> = Vec::new();
    let mut part = [0i8; P];

    let mut sum: usize = 0;
    let mut input_ptr: usize = 0;
    let mut part_ptr: usize = 0;

    while input_ptr != input.len() {
        while sum != LIMIT {
            let value = match input.get(input_ptr) {
                Some(v) => *v,
                None => break,
            };

            sum += value.unsigned_abs() as usize;
            input_ptr += 1;
            part[part_ptr] = value;
            part_ptr += 1;
        }

        size.push(part_ptr);

        while sum != W {
            let value = random_sign(rng);

            part[part_ptr] = value;
            sum += 1;
            part_ptr += 1;
        }

        shuffle_array(&mut part, seed);
        chunks.push(part);

        part = [0i8; P];
        seed += 1;
        part_ptr = 0;
        sum = 0;
    }

    (chunks, size, origin_seed)
}

#[cfg(test)]
mod r3_compressro_test {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    #[test]
    fn test_bit_convert() {
        for n in 0..u8::MAX {
            let bits = convert_to_ternary(n);
            let out = convert_to_decimal(bits);
            let bits0 = convert_to_ternary(out);

            assert_eq!(n, out);
            assert_eq!(bits0, bits);
        }
    }

    #[test]
    fn test_r3_encode_decode_chunks() {
        let mut rng = ChaCha20Rng::from_entropy();

        for _ in 0..10 {
            let bytes: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();

            let r3 = r3_decode_chunks(&bytes);
            let out = r3_encode_chunks(&r3);

            assert_eq!(out, bytes);
        }
    }

    #[test]
    fn test_encode_decode_bytes_by_chunks_spliter_merge() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let rand_len = rng.gen_range(5..1000);
            let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
            let r3 = r3_decode_chunks(&bytes);
            let (chunks, size, seed) = r3_split_w_chunks(&r3, &mut rng);
            let merged = r3_merge_w_chunks(&chunks, &size, seed);

            let mut r3_sum = 0usize;
            for el in &r3 {
                r3_sum += el.unsigned_abs() as usize;
            }

            let mut m_sum = 0usize;

            for el in &merged {
                m_sum += el.unsigned_abs() as usize;
            }

            assert_eq!(r3_sum, m_sum);
            assert_eq!(size.len(), chunks.len());
            assert_eq!(merged.len(), r3.len());
            assert_eq!(merged, r3);
        }
    }

    #[test]
    fn test_spliter() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let rand_len = rng.gen_range(5..1000);
            let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
            let r3 = r3_decode_chunks(&bytes);
            let (chunks, size, _) = r3_split_w_chunks(&r3, &mut rng);

            for (chunk, index) in chunks.iter().zip(size) {
                let sum = chunk.iter().map(|&x| x.abs() as i32).sum::<i32>();

                assert_eq!(sum as usize, W);
                assert_eq!(chunk.len(), P);
                assert!(index <= P);
            }
        }
    }
}
