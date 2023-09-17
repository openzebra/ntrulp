use crate::math::nums::i32_mod_u14;

pub fn freeze(x: i16) -> i8 {
    let r = i32_mod_u14(x as i32 + 1, 3) as i8;

    r - 1
}

pub fn round<const P: usize>(a: &mut [i16; P]) {
    for i in 0..P {
        a[i] = a[i] - freeze(a[i]) as i16;
    }
}

#[test]
fn test_round() {
    use crate::kem::rq::Rq;
    use crate::random::CommonRandom;
    use crate::random::NTRURandom;

    const P: usize = 761;
    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;
    const P_PLUS_ONE: usize = P + 1;

    let mut random: NTRURandom = NTRURandom::new();
    let mut r3: Rq<P, Q, Q12> = Rq::from(random.short_random().unwrap())
        .recip3::<P_PLUS_ONE>()
        .unwrap();

    fn round3(h: &mut [i16; 761]) {
        let f: [i16; 761] = *h;
        for i in 0..761 {
            let inner = 21846i32 * (f[i] + 2295) as i32;
            h[i] = (((inner + 32768) >> 16) * 3 - 2295) as i16;
        }
    }

    let mut new_round = r3.coeffs.clone();

    round3(&mut r3.coeffs);
    round(&mut new_round);

    assert_eq!(new_round, r3.coeffs);
}

#[test]
fn test_freeze() {
    use rand::prelude::*;

    let mut rng = thread_rng();

    pub fn test_freeze(a: i32) -> i8 {
        let b = a - (3 * ((10923 * a) >> 15));
        let c = b - (3 * ((89_478_485 * b + 134_217_728) >> 28));

        c as i8
    }

    for _ in 0..1000 {
        let r = rng.gen::<i16>();

        let t1 = test_freeze(r as i32);
        let t2 = freeze(r);

        assert_eq!(t2, t1);
    }
}
