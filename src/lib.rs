pub mod sender {
    #![allow(non_snake_case)]
    use secp256k1::{Secp256k1, ecdh, SecretKey, PublicKey};

    /// The sender tweaks the silent payment address with the public key of their input: `X' = hash(i*X)*G + X`
    pub fn tweak_payment_address(recipient_output_public_key: &PublicKey, sender_input_secret_key: &SecretKey) ->  PublicKey {
        let secp = Secp256k1::new();

        let i = sender_input_secret_key.clone();
        let mut X = recipient_output_public_key.clone();

        // hash(i*X)
        let shared_secret = ecdh::SharedSecret::new(&X, &i);

        // hash(i*X)*G
        let tweaked_x = SecretKey::from_slice(&shared_secret.secret_bytes()).unwrap();
        // let mut tweaked_X = PublicKey::from_secret_key(&secp, &tweaked_x); not needed

        // hash(i*X)*G + X
        X.add_exp_assign(&secp, &tweaked_x.secret_bytes()).unwrap();

        X
    }
}

pub mod recipient {
    #![allow(non_snake_case)]
    use secp256k1::{Secp256k1, PublicKey, ecdh, SecretKey};

    pub fn detect_payment(recipient_output_secret_key: &SecretKey, sender_input_public_key: &PublicKey) ->  PublicKey {

        let secp = Secp256k1::new();

        // transaction input key (tweaked address ?)
        let I = sender_input_public_key.clone();

        // private key of the output in the corresponding transaction
        let x = recipient_output_secret_key.clone();

        // public key of the output in the corresponding transaction
        let mut X = PublicKey::from_secret_key(&secp, &x);

        // hash(x*I)
        let shared_secret = ecdh::SharedSecret::new(&I, &x);

        // hash(x*I)*G
        let tweaked_x = SecretKey::from_slice(&shared_secret.secret_bytes()).unwrap();
        // let mut tweaked_X = PublicKey::from_secret_key(&secp, &tweaked_x); not needed

        // hash(x*I)*G + X
        X.add_exp_assign(&secp, &tweaked_x.secret_bytes()).unwrap();

        X

    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use secp256k1::{Secp256k1, KeyPair, PublicKey, SecretKey};

    use crate::{sender, recipient};

    #[test]
    fn it_works() {
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
    }
}
