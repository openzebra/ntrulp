use crate::ntru::errors::NTRUErrors;

pub fn r3_encode<const P: usize>(f: &[i8; P]) -> Result<Vec<u8>, NTRUErrors<'static>> {
    let lenght = P / 4;
    let mut s = vec![0u8; lenght + 1];
    let mut x: i8;
    let mut f_iter = f.iter();

    let mut next_f = move || match f_iter.next() {
        Some(v) => Ok(v + 1),
        None => Err(NTRUErrors::R3EncodeError("input array is not enough big")),
    };

    for i in 0..lenght {
        x = next_f()?;
        x += next_f()? << 2;
        x += next_f()? << 4;
        x += next_f()? << 6;

        s[i] = x as u8;
    }

    x = next_f()?;
    s[lenght] = x as u8;

    Ok(s)
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

#[cfg(test)]
mod r3_encoder_tests {
    use super::*;

    #[test]
    fn test_r3_encode() {
        use crate::random::CommonRandom;
        use crate::random::NTRURandom;

        const P: usize = 761;

        let mut random: NTRURandom<P> = NTRURandom::new();
        let r3: [i8; P] = random.random_small().unwrap();
        let bytes = r3_encode::<P>(&r3).unwrap();
        let dec = r3_decode::<P>(&bytes);

        assert_eq!(dec, r3);
    }
}
