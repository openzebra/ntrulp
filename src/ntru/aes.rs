#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, SEEDS_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, SEEDS_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, SEEDS_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, SEEDS_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, SEEDS_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, SEEDS_BYTES};

use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Crypter, Mode};

fn crypto_stream_xor(
    out: &mut [u8],
    input: &[u8],
    nonce: &[u8],
    key: &[u8],
) -> Result<(), ErrorStack> {
    let cipher = Cipher::aes_256_ctr();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(nonce))?;

    // Выполняем шифрование
    crypter.pad(false);
    crypter.update(input, out)?;
    crypter.finalize(out)?;

    Ok(())
}

#[test]
fn test_open_ssl() {
    let key: [u8; SEEDS_BYTES] = [
        151, 153, 142, 125, 236, 255, 169, 87, 52, 34, 151, 78, 0, 78, 108, 210, 125, 3, 77, 69,
        58, 198, 92, 127, 159, 29, 250, 66, 226, 127, 151, 93,
    ];
    let nonce = [0u8; SEEDS_BYTES / 2];
    let mut buffer = [0u8; P * 4];
    let input = [0u8; P * 4];

    crypto_stream_xor(&mut buffer, &input, &nonce, &key).unwrap();

    println!("{:?}", buffer);
}
