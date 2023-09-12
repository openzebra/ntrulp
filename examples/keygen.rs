use ntrulp::kem::r3::R3;
use ntrulp::kem::rq::Rq;
use ntrulp::key::pair::KeyPair;
use ntrulp::random::CommonRandom;
use ntrulp::random::NTRURandom;

fn main() {
    // init required params
    const P: usize = 761;
    const Q: usize = 4591;
    const W: usize = 286;
    const P_TWICE_MINUS_ONE: usize = P + P - 1;
    const Q12: usize = (Q - 1) / 2;
    const P_PLUS_ONE: usize = P + 1;
    const RQ_BYTES: usize = 1158;

    // INIT RNG.
    let mut random: NTRURandom<P> = NTRURandom::new();
    let mut pair0: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
    let mut pair1: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();
    let mut pair2: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> = KeyPair::new();

    // create entropy f/g
    let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
    let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

    pair0.from_seed(g, f).unwrap();

    let (pk, sk) = pair0.export_pair().unwrap();

    // Try import (more faser)
    pair1.import_pair(&pk, &sk);
    // try import by SK
    pair2.import_sk(&sk).unwrap();

    // verify keypair
    assert!(pair0.verify());
    assert!(pair1.verify());

    assert_eq!(&pair0.pub_key.h.coeffs, &pair1.pub_key.h.coeffs);
    assert_eq!(&pair0.priv_key.f.coeffs, &pair1.priv_key.f.coeffs);
    assert_eq!(&pair0.priv_key.ginv.coeffs, &pair1.priv_key.ginv.coeffs);

    assert_eq!(&pair0.pub_key.h.coeffs, &pair2.pub_key.h.coeffs);
    assert_eq!(&pair0.priv_key.f.coeffs, &pair2.priv_key.f.coeffs);
    assert_eq!(&pair0.priv_key.ginv.coeffs, &pair2.priv_key.ginv.coeffs);
}
