// TODO: make it for not only 761
pub fn freeze(a: i32) -> i16 {
    let mut b = a;

    b -= 4_591 * ((228 * b) >> 20);
    b -= 4_591 * ((58_470 * b + 134_217_728) >> 28);

    b as i16
}

const P: u16 = 761;
const Q: u32 = 4591;
const W: usize = 286;
const Q12: u16 = (Q as u16 - 1) / 2;

// fn int32_mod_uint14(x: i32, m: u16) -> (u32, u16) {
//     int32_divmod_uint14(x, m)
// }
//
// fn fq_freeze(x: i32) -> i16 {
//     let (_, r) = int32_mod_uint14(x + Q12 as i32, Q as u16);
//
//     (r as i16).wrapping_sub(Q12 as i16)
// }
//
// #[test]
// fn test_freeze() {
//     // let res = uint32_divmod_uint14(2147498264u32, 4591u16);
//     // dbg!(res);
//     // let native = freeze(12321);
//     // dbg!(native);
//     // let new_v = fq_freeze(12321);
//     // dbg!(new_v);
// }
