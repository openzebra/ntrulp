use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{r3_encrypt, rq_decrypt};
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = NTRURandom::new();
    let r: R3 = Rq::from(rng.short_random().unwrap()).r3_from_rq();
    let f: Rq = Rq::from(rng.short_random().unwrap());
    let g: R3 = R3::from(rng.random_small().unwrap());
    let sk = PrivKey::compute(&f, &g).unwrap();
    let pk = PubKey::compute(&f, &g).unwrap();
    let enc = r3_encrypt(&r, &pk);

    cb.bench_function("encrypt", |b| {
        b.iter(|| {
            r3_encrypt(&r, &pk);
        });
    });
    cb.bench_function("decrypt", |b| {
        b.iter(|| {
            rq_decrypt(&enc, &sk);
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
