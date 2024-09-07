use rand::RngCore;

use crate::encode::{r3, rq};
use crate::key::priv_key::PrivKey;
use crate::key::pub_key::PubKey;
use crate::params::params::{P, R3_BYTES, RQ_BYTES, W};
use crate::{
    math::nums::weightw_mask,
    poly::{f3::round, r3::R3, rq::Rq},
};

pub fn rq_decrypt(c: &Rq, priv_key: &PrivKey) -> R3 {
    let f = &priv_key.0;
    let ginv = &priv_key.1;
    let mut r = [0i8; P];
    let cf = c.mult_r3(f);
    let cf3 = cf.mult_int(3);
    let e = cf3.r3_from_rq();
    let ev = e.mult(ginv);
    let mask: i16 = weightw_mask(ev.as_ref()); // 0 if weight w, else -1

    for (i, r) in r.iter_mut().enumerate() {
        if i < W {
            *r = (((ev.coeffs[i] ^ 1) as i16 & !mask) ^ 1) as i8;
        } else {
            *r = (ev.coeffs[i] as i16 & !mask) as i8;
        }
    }

    R3::from(r)
}

pub fn r3_encrypt(r: &R3, pub_key: &PubKey) -> Rq {
    let mut hr = pub_key.mult_r3(r);

    round(&mut hr.coeffs);

    hr
}

#[cfg(test)]
mod test_cipher {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use crate::rng::{random_small, short_random};

    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        let mut rng = ChaCha20Rng::from_entropy();
        let r: R3 = Rq::from(short_random(&mut rng).unwrap()).r3_from_rq();
        let f: Rq = Rq::from(short_random(&mut rng).unwrap());
        let mut g: R3;
        let sk = loop {
            g = R3::from(random_small(&mut rng));

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).unwrap();
        let encrypted = r3_encrypt(&r, &pk);
        let decrypted = rq_decrypt(&encrypted, &sk);

        assert_eq!(decrypted.coeffs, r.coeffs);
    }
}
