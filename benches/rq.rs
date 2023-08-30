use criterion::{criterion_group, criterion_main, Criterion};
use ntrulpr::{
    kem::{r3::R3, rq::Rq},
    ntru::ntrup::NTRUPrime,
    random::{CommonRandom, NTRURandom},
};

fn rq_benchmark(cb: &mut Criterion) {
    const P: usize = 761;
    const Q: usize = 4591;
    const W: usize = 286;
    const Q12: usize = (Q - 1) / 2;

    let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();

    ntrup.key_pair_gen().unwrap();

    let mut rng: NTRURandom<P> = NTRURandom::new();
    let r3: R3<P, Q, Q12> = R3::from(rng.random_small().unwrap());
    let rq: Rq<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap());

    cb.bench_function("rq_mull_r3: p=761", |b| {
        b.iter(|| {
            rq.mult_small(&r3);
        });
    });

    cb.bench_function("rq_recip3: p=761", |b| {
        b.iter(|| {
            rq.recip3().unwrap();
        });
    });

    cb.bench_function("rq_mult3: p=761", |b| {
        b.iter(|| {
            rq.mult3();
        });
    });
}

criterion_group!(benches, rq_benchmark);
criterion_main!(benches);
