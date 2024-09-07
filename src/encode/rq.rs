use crate::params::params::{P, RQ_BYTES};

pub type Rq = [i16; P];
pub type RqEncoded = [u8; RQ_BYTES];

pub fn encode(input: &Rq) -> RqEncoded {
    let mut bytes = [0u8; RQ_BYTES];

    input
        .iter()
        .zip(bytes.chunks_exact_mut(2))
        .for_each(|(&value, chunk)| {
            chunk.copy_from_slice(&value.to_be_bytes());
        });

    bytes
}

pub fn decode(input: &RqEncoded) -> Rq {
    let mut coeffs = [0i16; P];

    input
        .chunks_exact(2)
        .zip(coeffs.iter_mut())
        .for_each(|(chunk, coeff)| {
            *coeff = i16::from_be_bytes([chunk[0], chunk[1]]);
        });

    coeffs
}

#[cfg(test)]
mod tests_fq {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use crate::rng::short_random;

    use super::*;

    // #[test]
    // fn test_encode_decode() {
    //     let mut rng = ChaCha20Rng::from_entropy();
    //
    //     for _ in 0..100 {
    //         let coeffs = short_random(&mut rng).unwrap();
    //         let rq = Rq::from(coeffs);
    //
    //         let bytes = encode(&rq.coeffs);
    //         let res = decode(&bytes);
    //
    //         assert_eq!(rq.coeffs, res);
    //     }
    // }
}
