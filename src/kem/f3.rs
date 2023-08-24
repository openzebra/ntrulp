use crate::math::nums::int32_mod_uint14;

// pub fn freeze(a: i32) -> i8 {
//     let b = a - (3 * ((10923 * a) >> 15));
//     let c = b - (3 * ((89_478_485 * b + 134_217_728) >> 28));
//
//     c as i8
// }

// static small F3_freeze(int16 x) { return int32_mod_uint14(x + 1, 3) - 1; }

pub fn freeze(x: i16) -> i8 {
    let r = int32_mod_uint14(x as i32 + 1, 3) as i8;

    r - 1
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
