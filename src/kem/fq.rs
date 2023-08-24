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

fn uint32_divmod_uint14(x: u32, m: u16) -> (u32, u16) {
    let mut v = 0x80000000;
    let mut qpart;
    let mask;

    v /= m as u32;

    let mut q = 0;

    qpart = ((x as u64 * v as u64) >> 31) as u32;

    let new_x = x.wrapping_sub(qpart * m as u32);

    q += qpart;

    qpart = (new_x as u64 * v as u64) as u32 >> 31;

    let final_x = new_x.wrapping_sub(qpart * m as u32);

    q += qpart;

    let sub_x = final_x.wrapping_sub(m as u32);

    q += 1;
    mask = if sub_x >> 31 != 0 { 0xFFFFFFFF } else { 0 };

    let added_x = sub_x.wrapping_add(mask & m as u32);
    let final_q = q.wrapping_add(mask);

    (final_q, added_x as u16)
}

fn int32_divmod_uint14(x: i32, m: u16) -> (u32, u16) {
    let px = 0x80000000u32;
    let (mut uq, mut ur) = uint32_divmod_uint14(px + x as u32, m);
    let (uq2, ur2) = uint32_divmod_uint14(px, m);

    ur -= ur2;
    uq -= uq2;

    let mask: u32 = if ur >> 15 != 0 { 0xFFFFFFFF } else { 0 };

    ur += (mask & m as u32) as u16;
    uq += mask;

    // (uq as i32, ur)
    (uq, ur)
}

fn int32_mod_uint14(x: i32, m: u16) -> (u32, u16) {
    int32_divmod_uint14(x, m)
}

fn fq_freeze(x: i32) -> i16 {
    let (_, r) = int32_mod_uint14(x + Q12 as i32, Q as u16);

    (r as i16).wrapping_sub(Q12 as i16)
}

#[test]
fn test_int32_divmod_uint14() {
    let res0 = int32_divmod_uint14(2147, 4591);
    let res1 = int32_divmod_uint14(33, 11);
    let res2 = int32_divmod_uint14(6453, 1123);

    assert_eq!(res0, (0, 2147));
    assert_eq!(res1, (3, 0));
    assert_eq!(res2, (5, 838));
}

#[test]
fn test_freeze() {
    // let res = uint32_divmod_uint14(2147498264u32, 4591u16);
    // dbg!(res);
    let native = freeze(12321);
    dbg!(native);
    let new_v = fq_freeze(12321);
    dbg!(new_v);
}
