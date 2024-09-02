// src/wallet/multisig.rs

use secp256k1::{Message, Secp256k1, SecretKey, Signature};
use secp256k1::ecdsa::Signature;
use std::error::Error;
use sha2::{Sha256, Digest};

pub struct MultiSigWallet {
    pub addresses: Vec<String>,
    pub required_signatures: usize,
}

impl MultiSigWallet {
    pub fn new(addresses: Vec<String>, required_signatures: usize) -> Self {
        MultiSigWallet {
            addresses,
            required_signatures,
        }
    }

    pub fn sign_message(
        &self,
        message: &str,
        private_keys: Vec<SecretKey>,
    ) -> Result<Vec<Signature>, Box<dyn Error>> {
        if private_keys.len() < self.required_signatures {
            return Err(Box::from("Not enough signatures provided"));
        }

        let secp = Secp256k1::new();
        let message_hash = Message::from_slice(&Sha256::digest(message.as_bytes()))?;
        let mut signatures = Vec::new();

        for key in private_keys.iter().take(self.required_signatures) {
            let signature = secp.sign_ecdsa(&message_hash, key);
            signatures.push(signature);
        }

        Ok(signatures)
    }
}
