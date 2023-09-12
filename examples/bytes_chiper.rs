use ntrulp::ntru::ntrup::NTRUPrime;

fn main() {
    // init required params
    const P: usize = 761;
    const W: usize = 286;
    const Q: usize = 4591;
    const Q12: usize = (Q - 1) / 2;
    const P_PLUS_ONE: usize = P + 1;
    const RQ_BYTES: usize = 1158;
    const P_TWICE_MINUS_ONE: usize = P + P - 1;
    const ROUNDED_BYTES: usize = 1007;

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
    let bytes = content.as_bytes();

    // Init instance with params!
    let mut ntrup =
        NTRUPrime::<P, Q, W, Q12, ROUNDED_BYTES, RQ_BYTES, P_PLUS_ONE, P_TWICE_MINUS_ONE>::new()
            .unwrap();

    // generate random keys(privateKey, SecretKey)
    ntrup.key_pair_gen().unwrap();

    let (pk, _) = ntrup.key_pair.export_pair().unwrap();

    // Encrypt with PubKey
    let encrypted = ntrup.encrypt(&bytes, &pk).unwrap();
    // Decrypt with SecretKey
    let decrypted = ntrup.decrypt(encrypted).unwrap();
    let restored_content = String::from_utf8(decrypted).unwrap();

    assert_eq!(restored_content, content);
}
