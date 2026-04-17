use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (private_key, public_key) = secp.generate_keypair(&mut rng);

        Wallet { private_key, public_key }
    }

    pub fn get_address(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.public_key.serialize());
        hex::encode(hasher.finalize())
    }

    pub fn sign_hash(&self, hash: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(hash).unwrap();
        secp.sign_ecdsa(&message, &self.private_key)
    }
}

pub fn verify_signature(public_key: &PublicKey, hash: &[u8], signature: &Signature) -> bool {
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(hash).unwrap();
    secp.verify_ecdsa(&message, signature, public_key).is_ok()
}