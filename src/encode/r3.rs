pub fn r3_encode<const P: usize>(f: &[i8; P]) -> Vec<u8> {
    let lenght = P / 4;
    let mut s = vec![0u8; lenght + 1];
    let mut x: i8;
    let mut f_iter = f.iter();

    for i in 0..lenght {
        x = *f_iter.next().unwrap() + 1;
        x += (f_iter.next().unwrap() + 1) << 2;
        x += (f_iter.next().unwrap() + 1) << 4;
        x += (f_iter.next().unwrap() + 1) << 6;

        s[i] = x as u8;
    }

    x = f_iter.next().unwrap() + 1;
    s[lenght] = x as u8;

    s
}

pub fn r3_decode<const P: usize>(s: &[u8]) -> [i8; P] {
    let mut f = [0i8; P];
    let mut x: u8;
    let mut i = 0;

    while i < P / 4 {
        x = *s.get(i).unwrap_or(&0u8);
        f[i * 4] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 1] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 2] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 3] = (x & 3) as i8 - 1;
        i += 1;
    }

    x = *s.get(i).unwrap_or(&0u8);
    f[i * 4] = (x & 3) as i8 - 1;

    f
}

pub fn r3_split_bytes<const P: usize>(input: &[u8]) -> Vec<Vec<u8>> {
    let chunk_size = P / 4 + 1;
    let mut result: Vec<Vec<u8>> = Vec::new();

    for chunk in input.chunks(chunk_size) {
        let mut padded_chunk: Vec<u8> = chunk.to_vec();

        while padded_chunk.len() < chunk_size {
            padded_chunk.push(0);
        }
        result.push(padded_chunk);
    }

    result
}

#[test]
fn test_chunk_split() {
    use rand::Rng;

    const P: usize = 761;

    let mut rng = rand::thread_rng();
    let array_length: usize = 255 + rng.gen::<u8>() as usize;
    let random_bytes: Vec<u8> = (0..array_length).map(|_| rng.gen::<u8>()).collect();
    let bytes_chunks = r3_split_bytes::<P>(&random_bytes);

    assert_eq!(bytes_chunks.len(), random_bytes.len() / 190 + 1);
}

#[test]
fn test_r3_encode() {
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    const P: usize = 761;

    let mut random: NTRURandom<P> = NTRURandom::new();
    let r3: [i8; P] = random.random_small().unwrap();
    let bytes = r3_encode::<P>(&r3);
    let dec = r3_decode::<P>(&bytes);

    assert_eq!(dec, r3);
}
