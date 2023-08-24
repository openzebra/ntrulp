use crate::kem::fq;

// // h = f*g in the ring Rq
// pub fn mult_small<const P: usize>(h: &mut [i16], f: &[i16], g: &[i8]) {
//     let mut fg = vec![0i16; P + P - 1];
//
//     for i in 0..P {
//         let mut result = i16::default();
//
//         for j in 0..=i {
//             let value = result + f[j] * (g[i - j] as i16);
//             result = fq::freeze(value as i32);
//         }
//
//         fg[i] = result;
//     }
//
//     for i in P..P + P - 1 {
//         let mut result = i16::default();
//
//         for j in i - P + 1..P {
//             let value = result + f[j] * (g[i - j] as i16);
//             result = fq::freeze(value as i32);
//         }
//
//         fg[i] = result;
//     }
//
//     for i in (P..P + P - 2).rev() {
//         fg[i - P] = fq::freeze((fg[i - P] + fg[i]) as i32);
//         fg[i - P + 1] = fq::freeze((fg[i - P + 1] + fg[i]) as i32);
//     }
//
//     for i in 0..P {
//         h[i] = fg[i];
//     }
// }
//
// // h = 3f in Rq
// pub fn mult3<const P: usize>(h: &mut [i16], f: &[i16]) {
//     for i in 0..P {
//         let x = (3 * f[i]) as i32;
//
//         h[i] = fq::freeze(x);
//     }
// }
//
// #[cfg(test)]
// mod test_rq {
//     use super::*;
//
//     #[test]
//     fn test_mult_small() {
//         const P: usize = 9;
//         let f = [0, 0, 1, 0, 0, -1, 0, -1, -1];
//         let g = [-1, 0, -1, 1, -1, 0, 1, 0, 0];
//         let mut h = [i16::default(); P];
//
//         mult_small::<P>(&mut h, &f, &g);
//
//         assert_eq!(h, [2, 2, -2, 0, -1, 0, -2, 2, 1,])
//     }
//
//     #[test]
//     fn test_mult3() {
//         const P: usize = 9;
//         let f = [0, 0, 1, 0, 0, -1, 0, -1, -1];
//         let mut h = [i16::default(); P];
//
//         mult3::<P>(&mut h, &f);
//
//         assert_eq!(h, [0, 0, 3, 0, 0, -3, 0, -3, -3,])
//     }
// }
