use rand::RngCore;

use crate::{
    encode::shuffle::{shuffle_array, unshuffle_array},
    params::params::{DIFFICULT, P, R3_BYTES, W},
    rng::random_sign,
};

use super::error::CompressError;

pub const BITS_SIZE: usize = 6;
const SYS_SIZE: usize = std::mem::size_of::<usize>();

fn byte_to_usize_vec(list: &[u8]) -> Vec<usize> {
    let num_elements = list.len() / SYS_SIZE;
    let mut vec = Vec::with_capacity(num_elements);
    for chunk in list.chunks_exact(SYS_SIZE) {
        let mut bytes = [0; SYS_SIZE];
        bytes.copy_from_slice(chunk);
        vec.push(usize::from_ne_bytes(bytes));
    }
    vec
}

pub fn pack_bytes(mut bytes: Vec<u8>, size: Vec<usize>, seed: u64) -> Vec<u8> {
    let size_bytes_len = size.len() * SYS_SIZE;
    let additional_size = size_bytes_len + SYS_SIZE + 8;
    bytes.reserve(additional_size);

    for &s in &size {
        bytes.extend_from_slice(&s.to_ne_bytes());
    }
    let size_len_bytes = size_bytes_len.to_ne_bytes();
    bytes.extend_from_slice(&size_len_bytes);
    bytes.extend_from_slice(&seed.to_ne_bytes());

    bytes
}

pub fn unpack_bytes(bytes: &[u8]) -> Result<(Vec<u8>, Vec<usize>, u64), CompressError> {
    const X2_SYS_SIZE: usize = SYS_SIZE * 2;

    let bytes_len = bytes.len();
    let seed_bytes: [u8; 8] = bytes[bytes_len - 8..]
        .try_into()
        .or(Err(CompressError::SeedSliceError))?;
    let size_bytes_len: [u8; SYS_SIZE] = bytes[bytes_len - X2_SYS_SIZE..bytes_len - SYS_SIZE]
        .try_into()
        .or(Err(CompressError::SizeSliceError))?;
    let size_len = usize::from_ne_bytes(size_bytes_len);
    let seed = u64::from_ne_bytes(seed_bytes);

    if bytes_len < size_len || (bytes_len / size_len) < R3_BYTES {
        return Err(CompressError::ByteslengthError);
    }

    let size_bytes = &bytes[bytes_len - size_len - X2_SYS_SIZE..(bytes_len - X2_SYS_SIZE)];
    let size = byte_to_usize_vec(size_bytes);

    let bytes_data = &bytes[..bytes_len - size_len - X2_SYS_SIZE];

    Ok((bytes_data.to_vec(), size, seed))
}

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
    let num_chunks = (r3.len() + BITS_SIZE - 1) / BITS_SIZE;
    let mut output = Vec::with_capacity(num_chunks);

    for start in (0..r3.len()).step_by(BITS_SIZE) {
        let end = (start + BITS_SIZE).min(r3.len());
        let mut bits = [0i8; BITS_SIZE];
        for (i, &val) in r3[start..end].iter().enumerate() {
            bits[i] = val;
        }
        let byte = convert_to_decimal(bits);
        output.push(byte);
    }

    output
}

pub fn r3_decode_chunks(bytes: &[u8]) -> Vec<i8> {
    let mut output = Vec::with_capacity(bytes.len() * BITS_SIZE);

    for &byte in bytes {
        let bits = convert_to_ternary(byte);
        output.extend_from_slice(&bits);
    }

    output
}

pub fn r3_merge_w_chunks(chunks: &[[i8; P]], size: &[usize], seed: u64) -> Vec<i8> {
    let total_size: usize = size.iter().sum();
    let mut out = Vec::with_capacity(total_size);

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

    use crate::params::params1277::RQ_BYTES;

    use super::*;

    fn usize_vec_to_bytes(list: &[usize]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(list.len() * SYS_SIZE);
        for &x in list {
            bytes.extend_from_slice(&x.to_ne_bytes());
        }
        bytes
    }

    #[test]
    fn pack_unpack_bytes() {
        let mut rng = ChaCha20Rng::from_entropy();
        let bytes: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();
        let unlimted_poly = r3_decode_chunks(&bytes);
        let (chunks, size, seed) = r3_split_w_chunks(&unlimted_poly, &mut rng);
        let mut bytes: Vec<u8> = Vec::with_capacity(P * size.len());

        for _ in chunks {
            let mut rq_bytes: [u8; RQ_BYTES] = [0u8; RQ_BYTES];
            rng.fill_bytes(&mut rq_bytes);
            bytes.extend(rq_bytes);
        }

        let packed = pack_bytes(bytes.clone(), size.clone(), seed);
        let unpack_bytes = unpack_bytes(&packed).unwrap();

        assert_eq!(unpack_bytes.0, bytes);
        assert_eq!(unpack_bytes.1, size);
        assert_eq!(unpack_bytes.2, seed);
    }

    #[test]
    fn test_u64_convert() {
        let mut rng = ChaCha20Rng::from_entropy();
        let usize_list: Vec<usize> = (0..1024).map(|_| rng.gen()).collect();
        let bytes = usize_vec_to_bytes(&usize_list);
        let out = byte_to_usize_vec(&bytes);

        assert_eq!(out, usize_list);
    }

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
