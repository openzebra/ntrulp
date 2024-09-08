use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{r3_encrypt, rq_decrypt, static_bytes_decrypt, static_bytes_encrypt};
use ntrulp::ntru::std_cipher;
use ntrulp::params::params::R3_BYTES;
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::rng::{random_small, short_random};
use rand::RngCore;

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let r: R3 = Rq::from(short_random(&mut rng).unwrap()).r3_from_rq();
    let f: Rq = Rq::from(short_random(&mut rng).unwrap());
    let g: R3 = R3::from(random_small(&mut rng));
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

    let mut ciphertext: [u8; R3_BYTES] = r.to_bytes();
    rng.fill_bytes(&mut ciphertext);
    let cipher = static_bytes_encrypt(&ciphertext, &pk);

    cb.bench_function("static_bytes_encrypt", |b| {
        b.iter(|| {
            static_bytes_encrypt(&ciphertext, &pk);
        });
    });
    cb.bench_function("static_bytes_decrypt", |b| {
        b.iter(|| {
            static_bytes_decrypt(&cipher, &sk);
        });
    });

    let mut origin_plaintext = [0u8; 1024];
    rng.fill_bytes(&mut origin_plaintext);
    let origin_plaintext = origin_plaintext.to_vec();

    let cipher = std_cipher::bytes_encrypt(&mut rng, &origin_plaintext, pk.clone()).unwrap();

    cb.bench_function("parallel_bytes_encrypt", |b| {
        b.iter(|| {
            std_cipher::bytes_encrypt(&mut rng, &origin_plaintext, pk.clone()).unwrap();
        });
    });
    cb.bench_function("parallel_bytes_decrypt", |b| {
        b.iter(|| {
            std_cipher::bytes_decrypt(&cipher, sk.clone()).unwrap();
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);
