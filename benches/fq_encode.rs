use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::random::{CommonRandom, NTRURandom};

use ntrulp::encode::rq;

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = NTRURandom::new();
    let fq = rng.short_random().unwrap();

    cb.bench_function("new_fq", |b| {
        b.iter(|| {});
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
