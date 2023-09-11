use criterion::{criterion_group, criterion_main, Criterion};
use ntrulpr::{
    kem::{r3::R3, rq::Rq},
    key::pair::KeyPair,
    random::{CommonRandom, NTRURandom},
};

fn key_gen_benchmark(c: &mut Criterion) {
    c.bench_function("keygen: p=761", |b| {
        b.iter(|| {
            const P: usize = 761;
            const Q: usize = 4591;
            const W: usize = 286;
            const Q12: usize = (Q - 1) / 2;
            const RQ_BYTES: usize = 1158;
            const P_PLUS_ONE: usize = P + 1;
            const P_TWICE_MINUS_ONE: usize = P + P - 1;

            let mut random: NTRURandom<P> = NTRURandom::new();
            let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> =
                KeyPair::new();

            loop {
                let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
                let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

                match pair.from_seed(g, f) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        });
    });
    c.bench_function("keygen: p=857", |b| {
        b.iter(|| {
            const P: usize = 857;
            const Q: usize = 5167;
            const W: usize = 322;
            const RQ_BYTES: usize = 1322;
            const Q12: usize = (Q - 1) / 2;
            const P_PLUS_ONE: usize = P + 1;
            const P_TWICE_MINUS_ONE: usize = P + P - 1;

            let mut random: NTRURandom<P> = NTRURandom::new();
            let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> =
                KeyPair::new();

            loop {
                let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
                let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

                match pair.from_seed(g, f) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        });
    });
    c.bench_function("keygen: p=953", |b| {
        b.iter(|| {
            const P: usize = 953;
            const Q: usize = 6343;
            const W: usize = 396;
            const Q12: usize = (Q - 1) / 2;
            const RQ_BYTES: usize = 1505;
            const P_PLUS_ONE: usize = P + 1;
            const P_TWICE_MINUS_ONE: usize = P + P - 1;

            let mut random: NTRURandom<P> = NTRURandom::new();
            let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> =
                KeyPair::new();

            loop {
                let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
                let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

                match pair.from_seed(g, f) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        });
    });
    c.bench_function("keygen: p=1013", |b| {
        b.iter(|| {
            const P: usize = 1013;
            const Q: usize = 7177;
            const W: usize = 448;
            const Q12: usize = (Q - 1) / 2;
            const RQ_BYTES: usize = 1623;
            const P_PLUS_ONE: usize = P + 1;
            const P_TWICE_MINUS_ONE: usize = P + P - 1;

            let mut random: NTRURandom<P> = NTRURandom::new();
            let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> =
                KeyPair::new();

            loop {
                let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
                let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

                match pair.from_seed(g, f) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        });
    });
    c.bench_function("keygen: p=1277", |b| {
        b.iter(|| {
            const P: usize = 1277;
            const Q: usize = 7879;
            const W: usize = 492;
            const Q12: usize = (Q - 1) / 2;
            const RQ_BYTES: usize = 2067;
            const P_PLUS_ONE: usize = P + 1;
            const P_TWICE_MINUS_ONE: usize = P + P - 1;

            let mut random: NTRURandom<P> = NTRURandom::new();
            let mut pair: KeyPair<P, Q, Q12, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE> =
                KeyPair::new();

            loop {
                let f: Rq<P, Q, Q12> = Rq::from(random.short_random(W).unwrap());
                let g: R3<P, Q, Q12> = R3::from(random.random_small().unwrap());

                match pair.from_seed(g, f) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        });
    });
}

criterion_group!(benches, key_gen_benchmark);
criterion_main!(benches);
