use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

fn main() {
    let mut rng = NTRURandom::new();
    let f: Rq = Rq::from(rng.short_random().unwrap());
    let mut g: R3;
    let sk = loop {
        // use a loop because there are no guarantees that
        // the random number generator will produce the correct
        // combination that can enter and combine with f.
        g = R3::from(rng.random_small().unwrap());

        match PrivKey::compute(&f, &g) {
            Ok(s) => break s,
            Err(_) => continue,
        };
    };

    // if you have f, and g use compute, because it is faster!
    let pk = PubKey::compute(&f, &g).unwrap();

    // create PubKey from secret key.
    let imported_pk = PubKey::from_sk(&sk).unwrap();

    // convert to bytes
    let pk_bytes = imported_pk.as_bytes();

    // restore from bytes.
    let from_bytes = PubKey::import(&pk_bytes).unwrap();

    assert_eq!(from_bytes.coeffs, pk.coeffs);
}
