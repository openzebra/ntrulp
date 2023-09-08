pub const BITS_SIZE: usize = 6;

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

pub fn r3_encode<const P: usize>(f: &[i8; P]) -> Vec<u8> {
    let lenght = P / 4;
    let mut s = vec![0u8; lenght + 1];
    let mut x: i8;
    let mut f_iter = f.iter();

    for i in 0..lenght {
        // TODO: Remove unwrap.
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
    let swap = move |x: u8| -> i8 {
        let r = (x & 3) as i8 - 1;

        r
    };

    while i < P / 4 {
        x = *s.get(i).unwrap_or(&0u8);
        f[i * 4] = swap(x);
        x >>= 2;
        f[i * 4 + 1] = swap(x);
        x >>= 2;
        f[i * 4 + 2] = swap(x);
        x >>= 2;
        f[i * 4 + 3] = swap(x);
        i += 1;
    }

    x = *s.get(i).unwrap_or(&0u8);
    f[i * 4] = swap(x);

    f
}

// SIZEP = P / BITS_SIZE
pub fn r3_decode_bytes<const P: usize, const SIZEP: usize>(input: &[u8; SIZEP]) -> [i8; P] {
    let mut out = [0i8; P];

    for i in 0..SIZEP {
        let bits = convert_to_ternary(input[i]);

        for j in 0..BITS_SIZE {
            let index = i * BITS_SIZE + j;

            match out.get_mut(index) {
                Some(e) => *e = bits[j],
                None => continue,
            }
        }
    }

    out
}

// SIZEP = P / BITS_SIZE
pub fn r3_encode_bytes<const P: usize, const SIZEP: usize>(input: &[i8; P]) -> [u8; SIZEP] {
    let mut out = [0u8; SIZEP];
    let mut out_index = 0;

    for i in (0..P).step_by(BITS_SIZE) {
        if out_index == SIZEP {
            break;
        }
        let mut chunk = [0i8; BITS_SIZE];

        for j in 0..BITS_SIZE {
            chunk[j] = input[i + j];
        }

        out[out_index] = convert_to_decimal(chunk);

        out_index += 1;
    }

    out
}

// size is array with last index
pub fn r3_split_w_chunks<const P: usize, const W: usize>(
    input: &[i8],
) -> (Vec<[i8; P]>, Vec<usize>) {
    let mut chunks: Vec<[i8; P]> = Vec::new();
    let mut part = [0i8; P];
    let mut sum = 0u16;
    let mut i = 0;
    let mut size = Vec::new();

    for value in input {
        if sum + value.abs() as u16 <= W as u16 {
            part[i] = *value;
            sum += value.abs() as u16;
        } else {
            size.push(i - 1);
            i = 0;
            chunks.push(part);
            part = [0i8; P];
            part[i] = *value;
            sum = value.abs() as u16;
        }
        i += 1;
    }

    (chunks, size)
}

pub fn r3_decode_chunks(bytes: &[u8]) -> Vec<i8> {
    let mut output: Vec<i8> = Vec::new();

    for byte in bytes {
        let bits = convert_to_ternary(*byte);

        output.extend(bits);
    }

    output
}

#[test]
fn test_bit_convert() {
    for n in 0..u8::MAX {
        let bits = convert_to_ternary(n);
        let out = convert_to_decimal(bits);

        assert_eq!(n, out);
    }
}

#[test]
fn test_bytes_encode_decode() {
    use rand::Rng;

    const P: usize = 761;
    const SIZEP: usize = P / BITS_SIZE;

    for _ in 0..10 {
        let mut bytes = [0u8; SIZEP];

        rand::thread_rng().fill(&mut bytes[..]);

        let r3 = r3_decode_bytes::<P, SIZEP>(&bytes);
        let dec = r3_encode_bytes::<P, SIZEP>(&r3);

        assert_eq!(dec, bytes);
    }
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

#[test]
fn test_spliter() {
    use rand::Rng;

    const P: usize = 761;
    const W: usize = 286;

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let bytes: Vec<u8> = (0..1000).map(|_| rng.gen::<u8>()).collect();
        let r3 = r3_decode_chunks(&bytes);
        let (chunks, size) = r3_split_w_chunks::<P, W>(&r3);

        for (chunk, index) in chunks.iter().zip(size) {
            let sum = chunk.iter().map(|&x| x.abs() as i32).sum::<i32>();

            assert_eq!(sum as usize, W);
            assert_eq!(chunk.len(), P);
            assert_eq!(chunk[index + 1], 0);
            assert!(index <= P);
        }
    }
}
