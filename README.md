
# NTRUP-rs

This repository presents an implementation of high-security prime-degree large-Galois-group inert-modulus ideal-lattice-based cryptography on rust programing langudge.
“Prime degree” etc. are defenses against potential attacks; see [official website](https://www.ntru.org/).

This implementation uses: Fields of the form (Z/q)[x]/(xp −x−1), where p is prime, are used in “NTRU Prime”, introduced in this paper, and have all of our recommended defenses.

#### Parameter set:
* P = 653, Q = 4621, W = 288, RQ_BYTES=994, ROUNDED_BYTES=865
* p = 761, q = 4591, w = 286, RQ_BYTES=1158, ROUNDED_BYTES=1007
* p = 857, q = 5167, w = 322, RQ_BYTES=1322, ROUNDED_BYTES=1152
* p = 953, q = 6343, w = 396, RQ_BYTES=1505, ROUNDED_BYTES=1317
* p = 1013, q = 7177, w = 448, RQ_BYTES=1623, ROUNDED_BYTES=1423
* p = 1277, q = 7879, w = 492, RQ_BYTES=2067, ROUNDED_BYTES=1815

## Encrypt/Decrypt bytes example
```rust
const P: usize = 761;
const W: usize = 286;
const Q: usize = 4591;
const Q12: usize = (Q - 1) / 2;
const P_PLUS_ONE: usize = P + 1;
const RQ_BYTES: usize = 1158;
const P_TWICE_MINUS_ONE: usize = P + P - 1;
const ROUNDED_BYTES: usize = 1007;

let mut rng = rand::thread_rng();
let rand_len = rng.gen_range(5..10_000);
let mut ntrup =
    NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new()
    .unwrap();
let bytes: Vec<u8> = (0..rand_len).map(|_| rng.gen::<u8>()).collect();

ntrup.key_pair_gen().unwrap();

let (pk, _) = ntrup.key_pair.export_pair().unwrap();

let encrypted = ntrup.encrypt(&bytes, &pk).unwrap();
let decrypted = ntrup.decrypt(encrypted).unwrap();

assert_eq!(decrypted, bytes);
```

## TODO
 - add Falcon algorithm for sign,verify signature

## Warnings

#### Implementation 
This implementation has not undergone any security auditing and while care has been taken no guarantees can be made for either correctness or the constant time running of the underlying functions. **Please use at your own risk.**
