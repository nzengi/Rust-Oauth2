// src/wallet/mod.rs

pub mod multisig;
pub mod hardware;

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
        // Mesajı imzalama kodu burada yer alacak
        Ok("signed_message".to_string())
    }
}
