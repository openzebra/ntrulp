use crate::{
    kem::{r3::R3, rq::Rq},
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

    pub fn encrypt(&self) {}

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
        const P: usize = 761;
        const Q: usize = 4591;
        const W: usize = 286;
        const Q12: usize = (Q - 1) / 2;

        let ntrup: NTRUPrime<P, Q, W, Q12> = NTRUPrime::new().unwrap();
    }
}
