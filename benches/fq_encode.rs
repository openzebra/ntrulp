use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

use ntrulp::encode::rq;

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = NTRURandom::new();
    let coeffs = rng.short_random().unwrap();
    let rq = Rq::from(coeffs).recip::<1>().unwrap();
    let bytes0 = rq::encode(&rq.coeffs);

    cb.bench_function("fast_encode", |b| {
        b.iter(|| {
            rq::encode(&rq.coeffs);
        });
    });
    cb.bench_function("fast_decode", |b| {
        b.iter(|| {
            rq::decode(&bytes0);
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
