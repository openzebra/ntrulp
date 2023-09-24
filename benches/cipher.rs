use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{
    bytes_decrypt, bytes_encrypt, parallel_bytes_decrypt, parallel_bytes_encrypt, r3_encrypt,
    rq_decrypt,
};
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

    cb.bench_function("r3_encrypt", |b| {
        b.iter(|| {
            r3_encrypt(&r, &pk);
        });
    });
    cb.bench_function("rq_decrypt", |b| {
        b.iter(|| {
            rq_decrypt(&enc, &sk);
        });
    });

    let mut rng1 = NTRURandom::new();
    let ciphertext = rng.randombytes::<1024>();
    let cipher = bytes_encrypt(&mut rng1, &ciphertext, &pk);

    cb.bench_function("bytes_encrypt", |b| {
        b.iter(|| {
            bytes_encrypt(&mut rng1, &ciphertext, &pk);
        });
    });
    cb.bench_function("bytes_decrypt", |b| {
        b.iter(|| {
            bytes_decrypt(&cipher, &sk).unwrap();
        });
    });

    extern crate num_cpus;

    let num_threads = num_cpus::get();
    let mut rng2 = NTRURandom::new();

    let sk = Arc::new(sk);
    let pk = Arc::new(pk);

    let ciphertext = Arc::new(rng.randombytes::<1024>().to_vec());
    let cipher =
        Arc::new(parallel_bytes_encrypt(&mut rng2, &ciphertext, &pk, num_threads).unwrap());

    cb.bench_function("parallel_bytes_encrypt", |b| {
        b.iter(|| {
            parallel_bytes_encrypt(&mut rng2, &ciphertext, &pk, num_threads).unwrap();
        });
    });
    cb.bench_function("parallel_bytes_decrypt", |b| {
        b.iter(|| {
            parallel_bytes_decrypt(&cipher, &sk, num_threads).unwrap();
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
