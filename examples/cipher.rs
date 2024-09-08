use ntrulp::key::kem_error::KemErrors;
use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{r3_encrypt, rq_decrypt, static_bytes_decrypt, static_bytes_encrypt};
use ntrulp::ntru::std_cipher;
use ntrulp::params::params::R3_BYTES;
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::rng::{random_small, short_random};
use rand::RngCore;

fn gen_keys() -> Result<(PrivKey, PubKey), KemErrors> {
    let mut rng = rand::thread_rng();
    let mut g: R3;
    let f: Rq = Rq::from(short_random(&mut rng).unwrap());
    let sk = loop {
        g = R3::from(random_small(&mut rng));

        match PrivKey::compute(&f, &g) {
            Ok(s) => break s,
            Err(_) => continue,
        };
    };
    let pk = PubKey::compute(&f, &g).unwrap();

    Ok((sk, pk))
}

fn main() {
    // create random generator.
    let mut rng = rand::thread_rng();

    let mut bytes = [0u8; R3_BYTES];

    rng.fill_bytes(&mut bytes);

    let (sk, pk) = gen_keys().unwrap();

    // encryption for one thread only.
    let plaintext = Rq::from(short_random(&mut rng).unwrap())
        .r3_from_rq()
        .to_bytes();

    let encrypted = static_bytes_encrypt(&plaintext, &pk);
    let decrypted = static_bytes_decrypt(&encrypted, &sk);

    assert_eq!(decrypted, plaintext);

    let mut origin_plaintext = vec![0u8; 1024];
    rng.fill_bytes(&mut origin_plaintext);

    let mut ciphertext =
        std_cipher::bytes_encrypt(&mut rng, &origin_plaintext, pk.clone()).unwrap();
    let plaintext = std_cipher::bytes_decrypt(&ciphertext, sk.clone()).unwrap();

    assert_eq!(plaintext, origin_plaintext);
}
