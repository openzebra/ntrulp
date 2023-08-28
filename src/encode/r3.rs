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
        x = s[i];
        f[i * 4] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 1] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 2] = (x & 3) as i8 - 1;
        x >>= 2;
        f[i * 4 + 3] = (x & 3) as i8 - 1;
        i += 1;
    }

    x = s[i];
    f[i * 4] = (x & 3) as i8 - 1;

    f
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
