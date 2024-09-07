use crate::params::params::{P, R3_BYTES};

pub fn r3_encode(f: &[i8; P]) -> [u8; R3_BYTES] {
    let mut s = [0u8; R3_BYTES];

    f.chunks(4).enumerate().for_each(|(i, chunk)| {
        s[i] = chunk
            .iter()
            .enumerate()
            .fold(0u8, |acc, (j, &val)| acc | ((val + 1) as u8) << (j * 2));
    });

    if P % 4 != 0 {
        s[P / 4] = (f[P - 1] + 1) as u8;
    }

    s
}

pub fn r3_decode(s: &[u8; R3_BYTES]) -> [i8; P] {
    let mut f = [0i8; P];
    let mut x: u8;
    let mut i = 0;
    let swap = move |x: u8| -> i8 { (x & 3) as i8 - 1 };

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

#[cfg(test)]
mod r3_encoder_tests {
    use super::*;
    use crate::rng::random_small;

    #[test]
    fn test_r3_encode() {
        let mut rng = rand::thread_rng();
        let r3: [i8; P] = random_small(&mut rng);
        let bytes = r3_encode(&r3);
        let dec = r3_decode(&bytes);

        assert_eq!(dec, r3);
    }
}
