use criterion::{criterion_group, criterion_main, Criterion};
use ntrulpr::{
    kem::{r3::R3, rq::Rq},
    ntru::ntrup::NTRUPrime,
    random::{CommonRandom, NTRURandom},
};

fn encrypt_benchmark(cb: &mut Criterion) {
    use rand::Rng;

    cb.bench_function("r3/rq/encrypt/decrypt: p=761", |b| {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1158;
        const ROUNDED_BYTES: usize = 1007;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("r3/rq/encrypt/decrypt: p=857", |b| {
        const P: usize = 857;
        const W: usize = 322;
        const Q: usize = 5167;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1322;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1152;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("r3/rq/encrypt/decrypt: p=653", |b| {
        const P: usize = 653;
        const Q: usize = 4621;
        const W: usize = 288;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 994;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 865;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("r3/rq/encrypt/decrypt: p=953", |b| {
        const P: usize = 953;
        const Q: usize = 6343;
        const W: usize = 396;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1505;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1317;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("r3/rq/encrypt/decrypt: p=1013", |b| {
        const P: usize = 1013;
        const Q: usize = 7177;
        const W: usize = 448;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1623;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1423;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("r3/rq/encrypt/decrypt: p=1277", |b| {
        const P: usize = 1277;
        const Q: usize = 7879;
        const W: usize = 492;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 2067;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1815;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng: NTRURandom<P> = NTRURandom::new();
        let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_rq();
        b.iter(|| {
            let encrypted = ntrup.r3_encrypt(&c, &ntrup.key_pair.pub_key.h);
            let decrypted = ntrup.rq_decrypt(&encrypted);

            assert_eq!(decrypted.coeffs, c.coeffs);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=761", |b| {
        const P: usize = 761;
        const W: usize = 286;
        const Q: usize = 4591;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1158;
        const ROUNDED_BYTES: usize = 1007;
        const P_PLUS_ONE: usize = P + 1;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=857", |b| {
        const P: usize = 857;
        const W: usize = 322;
        const Q: usize = 5167;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1322;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1152;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=653", |b| {
        const P: usize = 653;
        const Q: usize = 4621;
        const W: usize = 288;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 994;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 865;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=953", |b| {
        const P: usize = 953;
        const Q: usize = 6343;
        const W: usize = 396;
        const P_PLUS_ONE: usize = P + 1;
        const Q12: usize = (Q - 1) / 2;
        const RQ_BYTES: usize = 1505;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1317;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=1013", |b| {
        const P: usize = 1013;
        const Q: usize = 7177;
        const W: usize = 448;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 1623;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1423;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });

    cb.bench_function("bytes:threads:encrypt/decrypt: p=1277", |b| {
        const P: usize = 1277;
        const Q: usize = 7879;
        const W: usize = 492;
        const Q12: usize = (Q - 1) / 2;
        const P_PLUS_ONE: usize = P + 1;
        const RQ_BYTES: usize = 2067;
        const P_TWICE_MINUS_ONE: usize = P + P - 1;
        const ROUNDED_BYTES: usize = 1815;

        let mut ntrup =
            NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new(
            )
            .unwrap();

        ntrup.key_pair_gen(rand::thread_rng()).unwrap();

        let mut rng = rand::thread_rng();
        let rand_len = rng.gen_range(5..10_000);
        let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();
        let (pk, _) = ntrup.key_pair.export_pair().unwrap();

        b.iter(|| {
            let encrypted = ntrup.encrypt(&bytes, &pk);

            ntrup.decrypt(encrypted);
        });
    });
}

criterion_group!(benches, encrypt_benchmark);
criterion_main!(benches);
