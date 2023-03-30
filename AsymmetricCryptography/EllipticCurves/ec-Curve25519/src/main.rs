use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn main() {
    //let mut alice_random = [0u8; 32];
    //OsRng.fill_bytes(&mut alice_random);
    let alice_secret = EphemeralSecret::new(OsRng);
    let alice_public = PublicKey::from(&alice_secret);
    let bob_secret = EphemeralSecret::new(OsRng);
    let bob_public = PublicKey::from(&bob_secret);
    let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
    let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
    assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
}
