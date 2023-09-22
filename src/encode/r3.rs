#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, R3_BYTES, W};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, R3_BYTES, W};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, R3_BYTES, W};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, R3_BYTES, W};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, R3_BYTES, W};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, R3_BYTES, W};
use crate::random::CommonRandom;
use crate::random::NTRURandom;

pub const BITS_SIZE: usize = 6;
pub const ENTROPY_NUMS: usize = 6;

fn convert_to_ternary(num: u8) -> [i8; BITS_SIZE] {
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

fn convert_to_decimal(ternary: [i8; BITS_SIZE]) -> u8 {
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

pub fn r3_encode(f: &[i8; P]) -> [u8; R3_BYTES] {
    let mut s = [0u8; R3_BYTES];
    let mut fi = 0;

    for i in 0..P / 4 {
        let mut x = f[fi] + 1;
        fi += 1;
        x += (f[fi] + 1) << 2;
        fi += 1;
        x += (f[fi] + 1) << 4;
        fi += 1;
        x += (f[fi] + 1) << 6;
        fi += 1;

        s[i] = x as u8;
    }

    s[P / 4] = (f[fi] + 1) as u8;

    s
}

pub fn r3_decode(s: &[u8; R3_BYTES]) -> [i8; P] {
    let mut f = [0i8; P];
    let mut x: u8;
    let mut i = 0;
    let swap = move |x: u8| -> i8 {
        let r = (x & 3) as i8 - 1;

        r
    };

    while i < P / 4 {
        x = s[i];
        f[i * 4] = swap(x);
        x >>= 2;
        f[i * 4 + 1] = swap(x);
        x >>= 2;
        f[i * 4 + 2] = swap(x);
        x >>= 2;
        f[i * 4 + 3] = swap(x);
        i += 1;
    }

    x = s[i];
    f[i * 4] = swap(x);

    f
}

pub fn r3_decode_chunks(bytes: &[u8]) -> Vec<i8> {
    let mut output: Vec<i8> = Vec::new();

    for byte in bytes {
        let bits = convert_to_ternary(*byte);

        output.extend(bits);
    }

    output
}

pub fn r3_encode_chunks(r3: &[i8]) -> Vec<u8> {
    const EMPTY: [i8; BITS_SIZE] = [0i8; BITS_SIZE];
    let mut output: Vec<u8> = Vec::new();

    for chunk in r3.chunks(BITS_SIZE) {
        let bits: [i8; BITS_SIZE] = chunk.try_into().unwrap_or(EMPTY);
        let byte = convert_to_decimal(bits);

        output.push(byte);
    }

    output
}

pub fn r3_merge_w_chunks<const P: usize>(chunks: &[[i8; P]], size: &[usize]) -> Vec<i8> {
    let mut out = Vec::new();

    for (index, chunk) in chunks.iter().enumerate() {
        let point = size[index];

        out.extend_from_slice(&chunk[..point]);
    }

    out
}

// size is array with last index
pub fn r3_split_w_chunks(input: &[i8], rng: &mut NTRURandom) -> (Vec<[i8; P]>, Vec<usize>) {
    // TODO: add entropy bytes for more hard statistical analysis
    // const LIMIT: usize = W - ENTROPY_NUMS;
    let mut chunks: Vec<[i8; P]> = Vec::new();
    let mut size = Vec::new();
    let mut part = [0i8; P];
    let mut sum = 0usize;
    let mut i = 0;

    for value in input {
        sum += value.abs() as usize;

        if sum <= W {
            part[i] = *value;
            i += 1;
        } else {
            size.push(i);
            i = 0;
            chunks.push(part);
            part = [0i8; P];
            part[i] = *value;
            sum = value.abs() as usize;
            i += 1;
        }
    }

    if sum != W {
        let num = rng.random_sign();

        size.push(i);
        i += 1;

        for _ in sum..W {
            part[i] = num;
            sum += 1;
            i += 1;
        }

        chunks.push(part);
    } else {
        chunks.push(part);
        size.push(i);
    }

    (chunks, size)
}

#[cfg(test)]
mod r3_encoder_tests {
    use super::*;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;
    use rand::Rng;

    #[test]
    fn test_bit_convert() {
        for n in 0..u8::MAX {
            let bits = convert_to_ternary(n);
            let out = convert_to_decimal(bits);

            assert_eq!(n, out);
        }
    }

    #[test]
    fn test_r3_encode() {
        let mut random: NTRURandom = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }

    #[test]
    fn test_r3_encode_decode_chunks() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let bytes: Vec<u8> = (0..1000).map(|_| rng.gen::<u8>()).collect();

            let r3 = r3_decode_chunks(&bytes);
            let out = r3_encode_chunks(&r3);

            assert_eq!(out, bytes);
        }
    }

    #[test]
    fn test_encode_decode_bytes_by_chunks_spliter_merge() {
        let mut rng = rand::thread_rng();
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..100 {
            let rand_len = rng.gen_range(5..1000);
            let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
            let r3 = r3_decode_chunks(&bytes);
            let (chunks, size) = r3_split_w_chunks(&r3, &mut random);
            let merged = r3_merge_w_chunks(&chunks, &size);

            let mut r3_sum = 0usize;
            for el in &r3 {
                r3_sum += el.abs() as usize;
            }

            let mut m_sum = 0usize;
            for el in &merged {
                m_sum += el.abs() as usize;
            }

            assert_eq!(r3_sum, m_sum);

            assert_eq!(merged.len(), r3.len());
            assert_eq!(merged, r3);
        }
    }

    #[test]
    fn test_spliter() {
        let mut rng = rand::thread_rng();
        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..10 {
            let rand_len = rng.gen_range(5..1000);
            let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
            let r3 = r3_decode_chunks(&bytes);
            let (chunks, size) = r3_split_w_chunks(&r3, &mut random);

            for (chunk, index) in chunks.iter().zip(size) {
                let sum = chunk.iter().map(|&x| x.abs() as i32).sum::<i32>();

                assert_eq!(sum as usize, W);
                assert_eq!(chunk.len(), P);
                assert!(index <= P);
            }
        }
    }
}
