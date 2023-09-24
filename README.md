
# NTRUP Rust

This repository presents an implementation of high-security prime-degree large-Galois-group inert-modulus ideal-lattice-based cryptography on rust programing langudge.
“Prime degree” etc. are defenses against potential attacks; see [official website](https://www.ntru.org/).

This implementation uses: Fields of the form (Z/q)[x]/(xp −x−1), where p is prime, are used in “NTRU Prime”, introduced in this paper, and have all of our recommended defenses.

#### Parameter set:
* P = 653, Q = 4621, W = 288
* p = 761, q = 4591, w = 286
* p = 857, q = 5167, w = 322
* p = 953, q = 6343, w = 396
* p = 1013, q = 7177, w = 448
* p = 1277, q = 7879, w = 492

### install
```bash
cargo add ntrulp
```

### Testing

```bash
cargo test
```

```bash
cargo bench
```
## Keys Generation:
```rust
let mut rng = NTRURandom::new();
let f: Rq = Rq::from(rng.short_random().unwrap());
let mut g: R3;
let sk = loop {
    g = R3::from(rng.random_small().unwrap());

    match PrivKey::compute(&f, &g) {
        Ok(s) => break s,
        Err(_) => continue,
    };
};

let pk = PubKey::compute(&f, &g).unwrap();
let imported_pk = PubKey::from_sk(&sk).unwrap();
let pk_bytes = imported_pk.as_bytes();
let from_bytes = PubKey::import(&pk_bytes).unwrap();

assert_eq!(from_bytes.coeffs, pk.coeffs);
```
## Encrypt/Decrypt bytes example
```rust
use std::sync::Arc;

use ntrulp::key::priv_key::PrivKey;
use ntrulp::key::pub_key::PubKey;
use ntrulp::ntru::cipher::{
    bytes_decrypt, parallel_bytes_decrypt, parallel_bytes_encrypt, 
};
use ntrulp::ntru::errors::NTRUErrors;
use ntrulp::random::{CommonRandom, NTRURandom};


fn gen_keys<'a>() -> Result<(Arc<PrivKey>, Arc<PubKey>), NTRUErrors<'a>> {
    let mut rng = NTRURandom::new();
    let mut g: R3;
    let f: Rq = Rq::from(rng.short_random().unwrap());
    let sk = loop {
        g = R3::from(rng.random_small().unwrap());

        match PrivKey::compute(&f, &g) {
            Ok(s) => break s,
            Err(_) => continue,
        };
    };
    let pk = PubKey::compute(&f, &g).unwrap();

    Ok((Arc::new(sk), Arc::new(pk)))
}

let mut rng = NTRURandom::new();
let bytes = Arc::new(rng.randombytes::<1024>().to_vec());
let (sk, pk) = gen_keys().unwrap();

let num_threads = 4;
let encrypted1 = Arc::new(parallel_bytes_encrypt(&mut rng, &bytes, &pk, num_threads).unwrap());
let decrypted0 = parallel_bytes_decrypt(&encrypted1, &sk, num_threads).unwrap();
let decrypted1 = bytes_decrypt(&encrypted1, &sk).unwrap();

assert_eq!(decrypted0, decrypted1);
```

## TODO
 - add Falcon algorithm for sign,verify signature

## Warnings

#### Implementation 
This implementation has not undergone any security auditing and while care has been taken no guarantees can be made for either correctness or the constant time running of the underlying functions. **Please use at your own risk.**
