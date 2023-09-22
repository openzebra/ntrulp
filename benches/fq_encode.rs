use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

use ntrulp::encode::rq;

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = NTRURandom::new();
    let coeffs = rng.short_random().unwrap();
    let rq = Rq::from(coeffs).recip3().unwrap();
    let bytes = rq::encode(&rq.coeffs);

    cb.bench_function("encode", |b| {
        b.iter(|| {
            rq::encode(&rq.coeffs);
        });
    });
    cb.bench_function("decode", |b| {
        b.iter(|| {
            rq::decode(&bytes);
        });
    });
    cb.bench_function("native_decode", |b| {
        b.iter(|| {
            let mut list = Vec::new();

            for v in rq.coeffs {
                list.extend_from_slice(&v.to_be_bytes());
            }
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
