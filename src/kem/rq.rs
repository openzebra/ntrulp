use super::{errors::KemErrors, f3, r3::R3};
use crate::{
    kem::fq,
    math::nums::{i16_negative_mask, i16_nonzero_mask},
};

#[derive(Debug)]
pub struct Rq<const P: usize, const Q: usize, const Q12: usize> {
    pub coeffs: [i16; P],
}

impl<const P: usize, const Q: usize, const Q12: usize> Rq<P, Q, Q12> {
    pub fn new() -> Self {
        Self { coeffs: [0i16; P] }
    }

    pub fn from(coeffs: [i16; P]) -> Self {
        Self { coeffs }
    }

    pub fn eq_one(&self) -> bool {
        for i in 1..self.coeffs.len() {
            if self.coeffs[i] != 0 {
                return false;
            }
        }

        self.coeffs[0] != 0
    }

    pub fn eq_zero(&self) -> bool {
        for c in self.coeffs {
            if c != 0 {
                return false;
            }
        }

        true
    }

    // h = f*g in the ring Rq
    pub fn mult_r3<const P_TWICE_MINUS_ONE: usize>(&self, gq: &R3<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let mut out = [0i16; P];
        let f = self.coeffs;
        let g = gq.coeffs;
        let mut fg = [0i16; P_TWICE_MINUS_ONE];

        let quotient = |r: i16, f: i16, g: i8| {
            let value = r + f * g as i16;
            fq::freeze::<Q12, Q>(value as i32)
        };

        for i in 0..P {
            let mut result = 0i16;

            for j in 0..=i {
                result = quotient(result, f[j], g[i - j]);
            }

            fg[i] = result;
        }

        for i in P..P_TWICE_MINUS_ONE {
            let mut result = 0i16;

            for j in (i - P + 1)..P {
                result = quotient(result, f[j], g[i - j]);
            }

            fg[i] = result;
        }

        for i in (P..=(P + P - 2)).rev() {
            // TODO: -1530 = f * 1/f.
            // TODO: for diff params it diff result!
            fg[i - P] = fq::freeze::<Q12, Q>((fg[i - P] + fg[i]) as i32);
            fg[i - P + 1] = fq::freeze::<Q12, Q>((fg[i - P + 1] + fg[i]) as i32);
        }

        // out[..P].clone_from_slice(&fg[..P]);
        out[..P].copy_from_slice(&fg[..P]);

        Rq::from(out)
    }

    // out = 1/(3*in) in Rq
    pub fn recip3<const P_PLUS_ONE: usize>(&self) -> Result<Rq<P, Q, Q12>, KemErrors> {
        let input = self.coeffs;
        let mut out = [0i16; P];
        let mut f = [0i16; P_PLUS_ONE];
        let mut g = [0i16; P_PLUS_ONE];
        let mut v = [0i16; P_PLUS_ONE];
        let mut r = [0i16; P_PLUS_ONE];
        let mut delta: i16;
        let mut swap: i16;
        let mut t: i16;
        let mut f0: i32;
        let mut g0: i32;
        let scale: i16;

        let quotient = |out: &mut [i16], f0: i32, g0: i32, fv: &[i16]| {
            for i in 0..P_PLUS_ONE {
                let x = f0 * out[i] as i32 - g0 * fv[i] as i32;
                out[i] = fq::freeze::<Q12, Q>(x);
            }
        };

        r[0] = fq::recip::<Q12, Q>(3);
        f[0] = 1;
        f[P - 1] = -1;
        f[P] = -1;

        for i in 0..P {
            g[P - 1 - i] = input[i] as i16;
        }

        g[P] = 0;
        delta = 1;

        for _ in 0..2 * P - 1 {
            for i in (1..=P).rev() {
                v[i] = v[i - 1];
            }
            v[0] = 0;

            swap = i16_negative_mask(-delta) & i16_nonzero_mask(g[0]);
            delta ^= swap & (delta ^ -delta);
            delta += 1;

            for i in 0..P + 1 {
                t = swap & (f[i] ^ g[i]);
                f[i] ^= t;
                g[i] ^= t;
                t = swap & (v[i] ^ r[i]);
                v[i] ^= t;
                r[i] ^= t;
            }

            f0 = f[0] as i32;
            g0 = g[0] as i32;

            quotient(&mut g, f0, g0, &f);
            quotient(&mut r, f0, g0, &v);

            for i in 0..P {
                g[i] = g[i + 1];
            }

            g[P] = 0;
        }

        scale = fq::recip::<Q12, Q>(f[0]);

        for i in 0..P {
            let x = scale as i32 * (v[P - 1 - i] as i32);
            out[i] = fq::freeze::<Q12, Q>(x) as i16;
        }

        if i16_nonzero_mask(delta) == 0 {
            Ok(Rq::from(out))
        } else {
            Err(KemErrors::NoSolutionRecip3)
        }
    }

    // h = 3f in Rq
    pub fn mult3(&self) -> Rq<P, Q, Q12> {
        let mut out = [0i16; P];

        for i in 0..P {
            let x = (3 * self.coeffs[i]) as i32;

            out[i] = fq::freeze::<Q12, Q>(x);
        }

        Rq::from(out)
    }

    pub fn r3_from_rq(&self) -> R3<P, Q, Q12> {
        let mut out = [0i8; P];

        for i in 0..P {
            out[i] = f3::freeze(self.coeffs[i])
        }

        R3::from(out)
    }
}

#[cfg(test)]
mod test_rq {
    use super::*;
    use crate::params::params::{P, Q, Q12};
    use crate::random::{CommonRandom, NTRURandom};

    #[cfg(feature = "ntrulpr761")]
    #[test]
    fn test_recip3_761() {
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut random: NTRURandom = NTRURandom::new();

        for _ in 0..2 {
            let rq: Rq<P, Q, Q12> = Rq::from(random.short_random().unwrap());

            let out = rq.recip3::<P_PLUS_ONE>().unwrap();
            let h = out.mult_r3::<P_TWICE_MINUS_ONE>(&rq.r3_from_rq());

            assert!(h.eq_one());
        }
    }

    #[test]
    fn test_recip3_exact_match_with_sage() {
        const P: usize = 761;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;

        let zero_r3: Rq<P, Q, Q12> = Rq::new();
        let r3: Rq<P, Q, Q12> = Rq::from([
            1, 0, 0, 0, 0, 0, 1, 0, -1, -1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0,
            1, 0, -1, 1, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 1,
            0, 0, 1, -1, -1, -1, 0, 1, 1, 1, -1, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 1, -1, 0, 0, 0, -1,
            0, 0, 1, 0, -1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, -1, -1, 1, -1, 0, -1, 0, 1,
            0, 0, 0, 0, 0, -1, 1, 0, 1, -1, -1, 1, 0, 0, 0, -1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1,
            0, -1, 0, 0, 0, 0, -1, -1, 1, 1, -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, -1, 0, 1, 0, 0,
            1, -1, 0, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, -1, 0, 1, 1, 0, 0, 0, -1, -1,
            0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, -1, 0, -1, 0, 1, 1, 1, -1, 0,
            1, 1, -1, 0, 1, 0, 1, 0, 1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 0, 0, 0, -1, 0, -1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, 1, 1, 0,
            0, -1, 0, -1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, -1, 1, 1, 0, 0, 0, 0, 0, 1, -1, -1,
            1, -1, 0, 0, 0, -1, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, -1, 1, -1, 0, 0, -1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, -1, 0, -1, -1, -1, 0, 1, -1, -1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,
            -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, 0, -1, -1, -1, 0, 1, 1, -1, 0,
            0, 0, -1, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 0, -1, -1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1,
            0, 1, 0, 0, -1, -1, -1, 0, 1, -1, 0, -1, -1, 0, 0, 0, 1, 0, -1, -1, 0, 0, 1, 1, 0, 0,
            0, 0, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 1, 0, 0, 1,
            0, 0, 1, 0, 0, 0, 1, 0, -1, -1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, -1, 0, 0, 0,
            0, 0, 0, 1, -1, 0, 0, 1, 0, 0, 0, -1, 0, -1, 0, 0, 1, 0, 0, 1, 0, 1, 1, -1, 0, 0, 0, 0,
            -1, 0, 0, -1, 0, 0, 1, 0, -1, 0, -1, 1, 0, 0, 0, 0, -1, -1, 1, 0, 1, 0, 0, -1, 0, 0, 1,
            0, 1, -1, 0, 0, 1, -1, -1, 0, 0, 1, 0, 1, -1, 1, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0,
            -1, -1, 0, 0, -1, 0, -1, -1, -1, 1, 0, 1, 1, 0, 0, 0, 0, 1, -1, 0, -1, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, 1, 1, 0, 0, -1, 1, 1, 0,
            0, 0, -1, 0, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0,
            0, 0, 0, 1, 1, 0, -1, 1, 0, -1, 0, 0, 0, 0, 1, 1, 1, 0, 0, -1, 0, 1, 0, -1, 0, 0, -1,
            0, -1, 1, 0, 0, 0, 0, 0, 1, -1, 1, 0, 0, 1, -1, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0,
            0, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0,
            0, -1, -1, -1, 0, 0, 0, 0, -1, -1, 0, 1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 1,
        ]);

        let out = r3.recip3::<P_PLUS_ONE>().unwrap();

        assert!(zero_r3.recip3::<P_PLUS_ONE>().is_err());

        assert_eq!(
            out.coeffs,
            [
                -1332, -599, 1628, 2057, -1137, -318, -1943, -1693, 690, -2276, 1635, 1114, -1716,
                -2181, -2229, -394, -1054, 521, 10, 2235, -603, -377, 704, 561, 1540, 868, -1969,
                -1150, -311, 828, -645, 388, 607, -355, -331, 1396, 1362, 2278, -25, 1532, 654,
                -1219, 595, 1943, -1917, -1756, 1058, 1395, -1520, 753, -1747, 2125, 1281, 298,
                1611, 953, 947, 1431, 272, -1678, -1333, 2276, 2041, 8, 271, -289, 1386, -1250,
                1575, 1364, -47, -2044, 2079, 407, 844, 1522, -449, -826, -2250, -389, -1692, -667,
                -663, -1427, 943, -680, 143, 1865, -1629, -101, -756, 758, 1126, 517, 837, -1701,
                1764, 826, -273, 560, -1223, -677, 549, 2128, -911, 1637, -216, 1642, 173, 2017,
                -373, -295, 1399, -1842, 1689, 1532, -1566, -873, 393, -758, 1619, 843, 2244,
                -1088, 251, -1484, 308, 1410, 73, 1729, -1068, 541, -2126, 1985, -1771, -1735, 587,
                310, -1360, -1590, -9, 431, 1913, -383, -841, 221, -1357, -1959, -613, 1718, 394,
                2111, -720, -1621, -1298, 761, 1876, 2034, 698, -856, 2294, 1764, 1781, -1945,
                1681, -1868, 2239, -1154, -1168, 1729, -471, 1817, 1740, 1428, 1675, 1313, 839,
                -2179, 2182, 725, -2137, -637, -1288, -784, -1722, 205, -1658, 26, 734, 1556, -177,
                1674, -1517, 1973, 1161, -2219, 671, -90, -1394, -1686, 375, -31, 1139, -1755, 920,
                -276, -2185, 312, 752, 425, -2147, -76, 1178, 2027, 1353, 1066, -336, 1968, 1112,
                -567, 184, -879, 1122, -1063, 455, -1920, -1827, 1559, -1408, -2212, 192, -1021,
                -1702, 1072, -1700, -409, 1691, -981, -375, 263, 819, 37, -1266, -583, -676, 2133,
                -1337, -1553, 1513, -457, 1621, -1180, 1690, 743, 824, -217, 206, -555, -1121,
                2221, 1003, 1018, 887, 465, 1293, -1998, -2001, -1922, 118, 2233, 981, -2186, -842,
                -2288, 1340, -1843, 1723, -2269, -868, 2039, 965, -866, -2094, 2266, -914, -996,
                1281, -2157, -841, -423, 1823, -2070, -21, 802, 1975, -165, -1389, 1518, -1401,
                -1289, -1267, -793, 1883, 2140, 561, -1303, 1796, 1428, -648, 2157, -971, 866,
                1273, -1399, 184, 1304, -305, 32, -1825, 896, 2008, 579, 2068, -2268, 1715, -239,
                -1313, 2142, -277, -2126, 572, -2054, -131, -193, 340, -2262, -1871, 1324, 389,
                -1502, -958, -804, 183, 964, -1708, 560, -921, 1953, -1188, -2250, 20, -906, 979,
                416, 2096, -1746, -296, 2035, -1964, -1305, 189, 1655, 1370, -1360, 262, -2232,
                -10, -1119, -1120, 352, -114, -489, 1946, -436, 1990, -2282, 1038, 546, 553, -264,
                111, 1635, -1852, 475, -1051, 712, -13, 2166, -2082, 1514, -2091, -148, -783, -678,
                -1602, -2272, -1925, -2061, 1772, -1174, 416, 1598, 585, -2223, 1335, 933, -1096,
                121, -7, 1462, 47, 1591, 1227, -461, -1542, -213, -1018, -123, -1005, 175, 1511,
                -391, -1915, 249, 41, -2211, 504, 2088, 1305, -2261, 966, -1845, -1793, -2283,
                1978, 232, -1923, 360, -161, -1590, 1257, 714, 2105, -2097, 1487, -794, -171, 1830,
                -1407, 18, -2009, 1980, -2075, -1835, 772, -67, 2112, -1297, -446, 1039, 2092, 520,
                1878, 1559, 1396, -1377, -247, -989, 2147, 493, -2112, 213, 651, 1247, 1126, 2043,
                -1573, -814, -1199, -31, 761, 1892, -1901, 131, -1106, 2018, -1043, -1064, 2191,
                1572, -1337, -1821, -1016, -536, -126, 955, 7, -1283, 469, -1209, 401, 1749, -992,
                -1037, 1002, 1345, 780, 1443, -1849, -1603, 2209, -563, -1372, 1396, -1484, -608,
                2123, 102, 67, 2090, -666, 1452, -1353, 19, -1903, -944, 221, -6, -1916, 59, 1398,
                1120, -632, -1748, 643, -1222, -27, -1837, 1350, -1416, -1830, -1451, -984, 117,
                -1474, -955, -1667, -200, -215, 236, 183, 56, -334, 1387, 1268, 2120, 1186, -517,
                1878, 26, -822, 925, -560, 99, 1220, 31, 1552, 894, -1229, 1646, -1862, 2060, 252,
                2083, 1016, -1088, 1245, -224, -2027, 1051, 26, -2062, 370, 652, -1004, 1404, 595,
                1253, 1805, -221, -1121, -2103, -1150, -1622, -10, 1206, 1737, -1843, 830, 1574,
                -17, 697, -2008, -1160, -983, -1512, -2133, 707, -1071, 2281, -863, 1270, 2194,
                373, -469, -537, -190, 1021, 1847, -2114, 384, 1875, -368, 912, -1548, 1330, -1595,
                7, 1876, -473, -1399, -1091, 2057, 2153, -1778, 878, -1011, -32, -1238, -355, 1827,
                559, 1459, 2058, -1975, -470, 2213, -1019, -1934, -371, -2226, 1235, -836, -2208,
                564, -1622, -1134, 1766, 1331, 1315, -1869, -1298, -392, -403, -31, -279, -618,
                1514, -397, 98, 1545, -1182, -422, 450, 155, 868, -2000, -281, 662, 879, 1489,
                -1380, -1918, 1174, -607, -2216, -1631, 883, -814, 2262, -247, -968, 93, 898, 895,
                1613, -135, -901, 279, 602, -1865, 1907, -1086, 1106, -281, 30, -1225, 1027, 1170,
                1708, -1392, 213, -763, -706, 1836, -1156, -343, -1162, -740, -2263, 1971, -1036,
                1436, -1764, -1386, -2069, -1942, -448, -1920, 238, -2273, 805, -239, 1828, -1231,
                -548, -976, -629, -1043, -933, -779, -224, 1287, -1747, -384, -1548, -151, 1845,
                1096, -1108, 2254, -761, -159, 749, 285, -1574, -100, 2087, -1057, 25, 862
            ]
        );
    }
}
