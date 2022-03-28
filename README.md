# Silent Payment

## Documentation

This libraty implements the [Basic scheme](https://gist.github.com/RubenSomsen/c43b79517e7cb701ebf77eec6dbb46b8#basic-scheme) section of [Silent Payments](https://gist.github.com/RubenSomsen/c43b79517e7cb701ebf77eec6dbb46b8) article.

## Usage Example

`cargo.toml`
```yaml
[package]
name = "test-lib"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"silent-payment-lib" = { git = "https://github.com/w0xlt/silent-payment-lib.git" }
secp256k1 = { version = "0.22.1", features = [ "rand-std", "bitcoin_hashes", "std" ] }

```

`main.rs`
```rust
use std::cmp::Ordering;

use secp256k1::{Secp256k1, KeyPair, SecretKey, PublicKey};
use silent_payment_lib::{sender, recipient};

fn main() {
    let secp = Secp256k1::new();

    let recipient_output_key_pair = KeyPair::new(&secp, &mut secp256k1::rand::thread_rng());
    let recipient_output_secret_key = SecretKey::from_keypair(&recipient_output_key_pair);
    let recipient_output_public_key = PublicKey::from_keypair(&recipient_output_key_pair);


    let sender_input_key_pair = KeyPair::new(&secp, &mut secp256k1::rand::thread_rng());
    let sender_input_secret_key = SecretKey::from_keypair(&sender_input_key_pair);
    let sender_input_public_key = PublicKey::from_keypair(&sender_input_key_pair);

    let sender_tweaked_output_public_key = sender::tweak_payment_address(&recipient_output_public_key, &sender_input_secret_key);
    let recipient_tweaked_output_public_key = recipient::detect_payment(&recipient_output_secret_key, &sender_input_public_key);
    assert_eq!(sender_tweaked_output_public_key.cmp(&recipient_tweaked_output_public_key), Ordering::Equal);

    println!("The keys below must be the same.");
    println!("Sender tweaked public key: {}", sender_tweaked_output_public_key);
    println!("Recipient tweaked public key: {}", recipient_tweaked_output_public_key);
}
```