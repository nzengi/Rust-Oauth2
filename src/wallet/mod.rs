// src/wallet/mod.rs

pub mod multisig;
pub mod hardware;

use secp256k1::{Secp256k1, Message, SecretKey};
use sha2::{Sha256, Digest};

pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

impl Wallet {
    pub fn new(address: &str, private_key: &str) -> Result<Self, String> {
        Ok(Wallet {
            address: address.to_string(),
            private_key: private_key.to_string(),
        })
    }

    pub fn sign_message(&self, message: &str) -> Result<String, String> {
        let secp = Secp256k1::new();

        // Private key'yi hex string'den SecretKey formatına çeviriyoruz
        let private_key = SecretKey::from_slice(&hex::decode(&self.private_key)
            .map_err(|_| "Invalid private key")?)
            .map_err(|_| "Invalid private key format")?;

        // Mesajı SHA256 ile hash'liyoruz
        let message_hash = Sha256::digest(message.as_bytes());

        // Hash edilmiş mesajı secp256k1 formatına çeviriyoruz
        let message = Message::from_slice(message_hash.as_slice()).map_err(|_| "Invalid message hash")?;

        // İmzayı oluşturuyoruz
        let signature = secp.sign_ecdsa(&message, &private_key);

        // İmzayı hex string olarak döndürüyoruz
        Ok(hex::encode(signature.serialize_compact()))
    }
}
