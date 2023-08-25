const V: u32 = 0x80000000;

// return -1 if x!=0; else return 0
pub fn i16_nonzero_mask(x: i16) -> i16 {
    let u = x as u16;
    let mut v = u as u32;

    // v = !v;
    v = if v == 0 { 0 } else { !v.wrapping_add(1) };
    v >>= 31;

    return if v != 0 { -1 } else { 0 };
}

pub fn u32_divmod_u14(x: u32, m: u16) -> (u32, u16) {
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

pub fn i32_divmod_u14(x: i32, m: u16) -> (u32, u32) {
    let px = V;
    let (mut uq, ur) = u32_divmod_u14(px.wrapping_add(x as u32), m);
    let mut ur = ur as u32;
    let (uq2, ur2) = u32_divmod_u14(px, m);

    ur = ur.wrapping_sub(ur2 as u32);
    uq = uq.wrapping_sub(uq2);

    let mask: u32 = if ur >> 15 != 0 { u32::MAX } else { 0 };

    ur = ur.wrapping_add(mask & m as u32);
    uq = uq.wrapping_add(mask);

    (uq, ur)
}

pub fn i32_mod_u14(x: i32, m: u16) -> u32 {
    i32_divmod_u14(x, m).1
}

#[test]
fn test_i32_divmod_u14() {
    assert_eq!(i32_divmod_u14(100, 30), (3, 10));
    assert_eq!(i32_divmod_u14(-100, 30), (4294967292, 20)); // Assuming V = 0
}

#[test]
fn test_u32_divmod_u14() {
    assert_eq!(u32_divmod_u14(100, 30), (3, 10));
    assert_eq!(u32_divmod_u14(223, 300), (0, 223));
    assert_eq!(u32_divmod_u14(V, 3000), (715827, 2648));
}

#[test]
fn test_nonzero_mask() {
    assert_eq!(i16_nonzero_mask(0), 0);
    assert_eq!(i16_nonzero_mask(42), -1);
    assert_eq!(i16_nonzero_mask(-42), -1);
    assert_eq!(i16_nonzero_mask(i16::MIN), -1);
    assert_eq!(i16_nonzero_mask(i16::MAX), -1);
    assert_eq!(i16_nonzero_mask(33), -1);
    assert_eq!(i16_nonzero_mask(-33), -1);
    assert_eq!(i16_nonzero_mask(28), -1);
    assert_eq!(i16_nonzero_mask(-28), -1);
    assert_eq!(i16_nonzero_mask(12345), -1);
    assert_eq!(i16_nonzero_mask(-12345), -1);
}
