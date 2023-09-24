use std::sync::Arc;

use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{
    bytes_decrypt, bytes_encrypt, parallel_bytes_decrypt, parallel_bytes_encrypt, r3_encrypt,
    rq_decrypt,
};
use ntrulp::ntru::errors::NTRUErrors;
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

fn gen_keys<'a>() -> Result<(Arc<PrivKey>, Arc<PubKey>), NTRUErrors<'a>> {
    let mut rng = NTRURandom::new();
    let f: Rq = Rq::from(rng.short_random().unwrap());
    let g: R3 = R3::from(rng.random_small().unwrap());
    let sk = PrivKey::compute(&f, &g).unwrap();
    let pk = PubKey::compute(&f, &g).unwrap();

    Ok((Arc::new(sk), Arc::new(pk)))
}

fn main() {
    // let create content which should be encrypted
    let content = "
In the realm of digital night, Satoshi did conceive,
A currency of cryptic might, for all to believe.
In code and chains, he wove the tale,
Of Bitcoin's birth, a revolution set to sail.

A name unknown, a face unseen,
Satoshi, a genius, behind the crypto machine.
With whitepaper in hand and vision so clear,
He birthed a new era, without any fear.

Decentralized ledger, transparent and free,
Bitcoin emerged, for the world to see.
Mining for coins, nodes in a network,
A financial system, no central clerk.

The world was skeptical, yet curiosity grew,
As Bitcoin's value steadily blew.
From pennies to thousands, a meteoric rise,
Satoshi's creation took us by surprise.

But Nakamoto vanished, into the digital mist,
Leaving behind a legacy, a cryptocurrency twist.
In the hearts of hodlers, Satoshi's name lives on,
A symbol of innovation, in the crypto dawn.
";
    // Convert utf8 string to bytes.
    let bytes = Arc::new(content.as_bytes().to_vec());

    // create random generator.
    let mut rng = NTRURandom::new();

    let (sk, pk) = gen_keys().unwrap();

    // encryption for one thread only.
    let encrypted0 = bytes_encrypt(&mut rng, &bytes, &pk);

    // amount of CPU threads
    let num_threads = 4;

    // encryption with 4 threads
    let encrypted1 = Arc::new(parallel_bytes_encrypt(&mut rng, &bytes, &pk, num_threads).unwrap());

    // decryption with 4 threads
    let decrypted0 = parallel_bytes_decrypt(&encrypted1, &sk, num_threads).unwrap();

    // decryption one thread
    let decrypted1 = bytes_decrypt(&encrypted0, &sk).unwrap();

    assert_eq!(decrypted0, decrypted1);

    // This example is not required, only if you wnat works with stack
    // or make modify encode and decode algorithms
    //
    // generate a random poly in field F3.
    let r: R3 = Rq::from(rng.short_random().unwrap()).r3_from_rq();

    // encryption r with pubKey in field Fq
    let cipher_rq = r3_encrypt(&r, &pk);
    // decrypt rq cipher
    let decrypted = rq_decrypt(&cipher_rq, &sk);

    assert_eq!(r.coeffs, decrypted.coeffs);
}
