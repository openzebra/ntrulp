use ntrulp::kem::r3::R3;
use ntrulp::kem::rq::Rq;
use ntrulp::ntru::ntrup::NTRUPrime;
use ntrulp::random::CommonRandom;
use ntrulp::random::NTRURandom;

// This method need only for encrypt little content
// in single thread, for use it need understand and learn how works NTRUP
fn main() {
    // init required params
    const P: usize = 761;
    const W: usize = 286;
    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;
    const RQ_BYTES: usize = 1158;
    const ROUNDED_BYTES: usize = 1007;
    const P_PLUS_ONE: usize = P + 1;
    const P_TWICE_MINUS_ONE: usize = P + P - 1;

    let mut ntrup =
        NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new()
            .unwrap();

    ntrup.key_pair_gen().unwrap();

    let mut rng: NTRURandom<P> = NTRURandom::new();
    let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

    let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
    let decrypted = ntrup.rq_decrypt(&encrypted);

    assert_eq!(decrypted.coeffs, c.coeffs);
}
