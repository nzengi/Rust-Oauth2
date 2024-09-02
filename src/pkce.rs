// src/pkce.rs

use base64::{encode_config, URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use rand::Rng;

pub struct PkceCode {
    pub code_verifier: String,
    pub code_challenge: String,
}

impl PkceCode {
    pub fn new() -> Self {
        let code_verifier = Self::generate_code_verifier();
        let code_challenge = Self::generate_code_challenge(&code_verifier);

        PkceCode {
            code_verifier,
            code_challenge,
        }
    }

    fn generate_code_verifier() -> String {
        let verifier_bytes: [u8; 32] = rand::thread_rng().gen();
        encode_config(&verifier_bytes, URL_SAFE_NO_PAD)
    }

    fn generate_code_challenge(code_verifier: &str) -> String {
        let hash = Sha256::digest(code_verifier.as_bytes());
        encode_config(&hash, URL_SAFE_NO_PAD)
    }
}
