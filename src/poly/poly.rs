use rand::prelude::*;

#[derive(Debug)]
pub struct NtruIntPoly {
    pub n: usize,
    pub coeffs: Vec<i16>,
}

pub fn ntruprime_mult_poly(
    a: &NtruIntPoly,
    b: &NtruIntPoly,
    c: &mut NtruIntPoly,
    modulus: u16,
) -> bool {
    let n = a.n;

    if n != b.n {
        return false;
    }

    c.n = n;
    c.coeffs = vec![0; n];

    for k in 0..n {
        let mut ck1 = 0;

        for i in 0..=k {
            ck1 += (a.coeffs[i] as u64) * (b.coeffs[k - i] as u64);
        }

        let mut ck2 = 0;

        for i in (k + 1)..n {
            ck2 += (a.coeffs[i] as u64) * (b.coeffs[k + n - i] as u64);
        }

        let ck = c.coeffs[k] as u64 + ck1 + ck2;

        c.coeffs[k] = (ck % (modulus as u64)) as i16;

        if k < n - 1 {
            let ck = c.coeffs[k + 1] as u64 + ck2;

            c.coeffs[k + 1] = (ck % (modulus as u64)) as i16;
        }
    }

    true
}

impl NtruIntPoly {
    // Add here random method
    pub fn new(n: usize) -> Self {
        let mut rng = thread_rng();
        let coeffs: Vec<i16> = (0..n).map(|_| rng.gen::<u32>() as i16).collect();

        NtruIntPoly { n, coeffs }
    }
}

#[test]
fn test_ntru_poly() {
    let mut poly = NtruIntPoly::new(761);

    dbg!(poly);
}
