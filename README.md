
# NTRUP-rs

This repository presents an implementation of high-security prime-degree large-Galois-group inert-modulus ideal-lattice-based cryptography on rust programing langudge.
“Prime degree” etc. are defenses against potential attacks; see [official website](https://www.ntru.org/).

This implementation uses: Fields of the form (Z/q)[x]/(xp −x−1), where p is prime, are used in “NTRU Prime”, introduced in this paper, and have all of our recommended defenses.

#### Parameter set:
* p = 653, q = 4621, w = 288
* p = 761, q = 4591, w = 286
* p = 857, q = 5167, w = 322
* p = 953, q = 6343, w = 396
* p = 1013, q = 7177, w = 448
* p = 1277, q = 7879, w = 492

## Usage example
```rust
const P: usize = 761;
const Q: usize = 4591;
const W: usize = 286;
const Q12: usize = (Q - 1) / 2;

let mut ntrup = NTRUPrime::<P, Q, W, Q12>::new().unwrap();

ntrup.key_pair_gen().unwrap();

let mut rng: NTRURandom<P> = NTRURandom::new();
let c: R3<P, Q, Q12> = Rq::from(rng.short_random(W).unwrap()).r3_from_r();
let encrypted = ntrup.encrypt(&c);
let decrypted = ntrup.decrypt(&encrypted);

assert_eq!(decrypted.get_coeffs(), c.get_coeffs())
```

## Warnings

#### Implementation 
This implementation has not undergone any security auditing and while care has been taken no guarantees can be made for either correctness or the constant time running of the underlying functions. **Please use at your own risk.**
