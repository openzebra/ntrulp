use crate::{
    kem::{f3::round, r3::R3, rq::Rq},
    key::pair::KeyPair,
    math,
    random::{CommonRandom, NTRURandom},
};

#[derive(Debug)]
pub enum NTRUPrimeErrors {
    PMustBePrimeNumber,
    QMustbePrimeNumber,
    WCannotBeLessZero,
    DubblePShouldBeMoreOrEqTripleW,
    QShouldBeMoreOrEq17MulWPlusOne,
    QModeSixShouldBeEqOne,
    KeyPairGen,
}

pub struct NTRUPrime<const P: usize, const Q: usize, const W: usize, const Q12: usize> {
    pub key_pair: KeyPair<P, Q, Q12>,
    rng: NTRURandom<P>,
}

impl<const P: usize, const Q: usize, const W: usize, const Q12: usize> NTRUPrime<P, Q, W, Q12> {
    pub fn new() -> Result<Self, NTRUPrimeErrors> {
        if !math::prime::is_prime(P) {
            return Err(NTRUPrimeErrors::PMustBePrimeNumber);
        }
        if !math::prime::is_prime(Q) {
            return Err(NTRUPrimeErrors::QMustbePrimeNumber);
        }
        if !(W > 0) {
            return Err(NTRUPrimeErrors::WCannotBeLessZero);
        }
        if !(2 * P >= 3 * W) {
            return Err(NTRUPrimeErrors::DubblePShouldBeMoreOrEqTripleW);
        }
        if !(Q >= 16 * W + 1) {
            return Err(NTRUPrimeErrors::QShouldBeMoreOrEq17MulWPlusOne);
        }
        if !(Q % 6 == 1) {
            // spec allows 5 but these tests do not
            return Err(NTRUPrimeErrors::QModeSixShouldBeEqOne);
        }

        let rng: NTRURandom<P> = NTRURandom::new();
        let key_pair: KeyPair<P, Q, Q12> = KeyPair::new();

        Ok(NTRUPrime { rng, key_pair })
    }

    pub fn encrypt(&self, r: R3<P, Q, Q12>) -> Rq<P, Q, Q12> {
        let h = &self.key_pair.pub_key.h;
        let hr = h.mult_small(&r);
        let hr_rounded = round(hr.get_coeffs());

        Rq::from(hr_rounded)
    }

    pub fn decrypt(&self) {}

    pub fn key_pair_gen(&mut self) -> Result<(), NTRUPrimeErrors> {
        const MAX_TRY: usize = 100;

        let mut k: usize = 0;

        loop {
            if k >= MAX_TRY {
                return Err(NTRUPrimeErrors::KeyPairGen);
            }

            let short_entropy = match self.rng.short_random(W) {
                Ok(s) => s,
                Err(_) => {
                    k += 1;
                    continue;
                }
            };
            let small_entropy = match self.rng.random_small() {
                Ok(s) => s,
                Err(_) => {
                    k += 1;
                    continue;
                }
            };
            let f: Rq<P, Q, Q12> = Rq::from(short_entropy);
            let g: R3<P, Q, Q12> = R3::from(small_entropy);

            match self.key_pair.from_seed(g, f) {
                Ok(_) => self.key_pair.verify(),
                Err(_) => {
                    k += 1;
                    continue;
                }
            };

            break;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ntrup::NTRUPrime;

    #[test]
    fn test_init_params() {
        NTRUPrime::<761, 4591, 286, 4590>::new().unwrap();
        NTRUPrime::<857, 5167, 322, 5166>::new().unwrap();
        NTRUPrime::<653, 4621, 288, 4620>::new().unwrap();
        NTRUPrime::<953, 6343, 396, 6342>::new().unwrap();
        NTRUPrime::<1013, 7177, 448, 7176>::new().unwrap();
        NTRUPrime::<1277, 7879, 492, 7878>::new().unwrap();
    }

    #[test]
    fn test_gen_key_pair() {
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;

        let ntrup: NTRUPrime<P, Q, W, Q12> = NTRUPrime::new().unwrap();

        assert!(ntrup.key_pair.verify());
    }
}
