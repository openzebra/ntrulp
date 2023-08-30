use criterion::{criterion_group, criterion_main, Criterion};
use ntrulpr::{
    kem::{r3::R3, rq::Rq},
    ntru::ntrup::NTRUPrime,
    random::{CommonRandom, NTRURandom},
};

fn encrypt_benchmark(cb: &mut Criterion) {
    const P: usize = 761;
    const Q: usize = 4591;
    const W: usize = 286;
    const Q12: usize = (Q - 1) / 2;

    let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();

    ntrup.key_pair_gen().unwrap();

    let mut rng: NTRURandom<P> = NTRURandom::new();
    let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();

    cb.bench_function("encrypt: p=761", |b| {
        b.iter(|| {
            let encrypted = ntrup.encrypt(&c);
            let decrypted = ntrup.decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });
}

criterion_group!(benches, encrypt_benchmark);
criterion_main!(benches);
