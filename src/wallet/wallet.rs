use secp256k1::{Secp256k1, Message, SecretKey};
use sha2::{Sha256, Digest};
use std::fmt;

pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

// Newtype tanımlama
pub struct SignatureWrapper(secp256k1::ecdsa::Signature);

impl fmt::Display for SignatureWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.serialize_compact()))
    }
}

impl SignatureWrapper {
    pub fn is_empty(&self) -> bool {
        self.0.serialize_compact().iter().all(|&byte| byte == 0)
    }
}

impl Wallet {
    pub fn new(address: &str, private_key: &str) -> Result<Self, String> {
        Ok(Wallet {
            address: address.to_string(),
            private_key: private_key.to_string(),
        })
    }

    pub fn sign_message(&self, message: &str) -> Result<SignatureWrapper, String> {
        let secp = Secp256k1::new();
        let private_key = SecretKey::from_slice(&hex::decode(&self.private_key).map_err(|_| "Invalid private key")?)
            .map_err(|_| "Invalid private key format")?;
        let message_hash = Sha256::digest(message.as_bytes());
        let message = Message::from_slice(&message_hash).map_err(|_| "Invalid message hash")?;
        let signature = secp.sign_ecdsa(&message, &private_key);
        Ok(SignatureWrapper(signature))
    }
}
