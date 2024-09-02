// src/oauth/mod.rs

pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
}

impl OAuthToken {
    pub fn new(access_token: String, token_type: String, expires_in: Option<u64>) -> Self {
        OAuthToken {
            access_token,
            token_type,
            expires_in,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_in {
            Some(exp) => exp == 0,
            None => false,
        }
    }
}
