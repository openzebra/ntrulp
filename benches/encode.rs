use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::encode::sort::crypto_sort_u32;
use rand::Rng;

fn encoder_benchmark(cb: &mut Criterion) {
    const SIZE: usize = 10_000_0;

    let mut seed = [0u32; SIZE];
    let mut rng = rand::thread_rng();

    rng.fill(&mut seed[..]);

    cb.bench_function("sort_int_32", |b| {
        b.iter(|| {
            crypto_sort_u32(&mut seed);
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
