use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::sha256;



fn main() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

    let sig = secp.sign_ecdsa(&message, &secret_key);
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
}
