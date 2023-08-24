const V: u32 = 0x80000000;

pub fn uint32_divmod_uint14(x: u32, m: u16) -> (u32, u16) {
    let mut v = V;
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
    mask = if sub_x >> 31 != 0 { u32::MAX } else { 0 };

    let added_x = sub_x.wrapping_add(mask & m as u32);
    let final_q = q.wrapping_add(mask);

    (final_q, added_x as u16)
}

pub fn int32_divmod_uint14(x: i32, m: u16) -> (u32, u32) {
    let px = V;
    let (mut uq, ur) = uint32_divmod_uint14(px.wrapping_add(x as u32), m);
    let mut ur = ur as u32;
    let (uq2, ur2) = uint32_divmod_uint14(px, m);

    ur = ur.wrapping_sub(ur2 as u32);
    uq = uq.wrapping_sub(uq2);

    let mask: u32 = if ur >> 15 != 0 { u32::MAX } else { 0 };

    ur = ur.wrapping_add(mask & m as u32);
    uq = uq.wrapping_add(mask);

    (uq, ur)
}

pub fn int32_mod_uint14(x: i32, m: u16) -> u32 {
    int32_divmod_uint14(x, m).1
}

#[test]
fn test_int32_divmod_uint14() {
    assert_eq!(int32_divmod_uint14(100, 30), (3, 10));
    assert_eq!(int32_divmod_uint14(-100, 30), (4294967292, 20)); // Assuming V = 0
}

#[test]
fn test_uint32_divmod_uint14() {
    assert_eq!(uint32_divmod_uint14(100, 30), (3, 10));
    assert_eq!(uint32_divmod_uint14(223, 300), (0, 223));
    assert_eq!(uint32_divmod_uint14(V, 3000), (715827, 2648));
}
