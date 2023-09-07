fn convert_to_ternary(num: u8) -> [i8; 4] {
    let mut result = [0i8; 4];
    let mut n = num;

    for i in (0..4).rev() {
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

pub fn r3_split_w_chunks<const P: usize, const W: usize>(input: &[i8]) -> Vec<[i8; P]> {
    let mut chunks: Vec<[i8; P]> = vec![];
    let mut part = [0i8; P];
    let mut sum = 0u16;
    let mut i = 0;

    for value in input {
        if sum + value.abs() as u16 <= W as u16 {
            part[i] = *value;
            sum += value.abs() as u16;
        } else {
            i = 0;
            chunks.push(part);
            part = [0i8; P];
            part[i] = *value;
            sum = value.abs() as u16;
        }
        i += 1;
    }

    chunks
}

// split a byte 00 = 0, 01 = -1, 11=1
pub fn r3_decode_chunks<const P: usize, const W: usize>(bytes: &[u8]) -> Vec<i8> {
    let mut output: Vec<i8> = Vec::new();

    for byte in bytes {
        let bits = convert_to_ternary(*byte);

        output.extend(&bits);
    }

    output
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

    let number = 54;
    let ternary_representation = convert_to_ternary(number);
    println!("{:?}", ternary_representation);
}

#[test]
fn test_spliter() {
    use rand::Rng;

    const P: usize = 761;
    const W: usize = 286;

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let bytes: Vec<u8> = (0..1000).map(|_| rng.gen::<u8>()).collect();
        let r3 = r3_decode_chunks::<P, W>(&bytes);
        let chunks = r3_split_w_chunks::<P, W>(&r3);

        for c in chunks {
            let sum = c.iter().map(|&x| x.abs() as i32).sum::<i32>();

            assert_eq!(sum as usize, W);
            assert_eq!(c.len(), P);
        }
    }
}

#[test]
fn test_r3() {
    const P: usize = 761;
    const Q: usize = 4591;
    const W: usize = 286;
    const Q12: usize = (Q - 1) / 2;

    let content = "
In the realm of digital night, Satoshi did conceive,
A currency of cryptic might, for all to believe.
In code and chains, he wove the tale,
Of Bitcoin's birth, a revolution set to sail.

A name unknown, a face unseen,
Satoshi, a genius, behind the crypto machine.
With whitepaper in hand and vision so clear,
He birthed a new era, without any fear.

Decentralized ledger, transparent and free,
Bitcoin emerged, for the world to see.
Mining for coins, nodes in a network,
A financial system, no central clerk.

The world was skeptical, yet curiosity grew,
As Bitcoin's value steadily blew.
From pennies to thousands, a meteoric rise,
Satoshi's creation took us by surprise.

But Nakamoto vanished, into the digital mist,
Leaving behind a legacy, a cryptocurrency twist.
In the hearts of hodlers, Satoshi's name lives on,
A symbol of innovation, in the crypto dawn.
";
    let bytes = content.as_bytes();
    let r3 = r3_decode_chunks::<P, W>(&bytes);

    // for c in r3 {
    //     print!("{c}, ");
    // }
}
