use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::error::Error;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

pub enum JwtAlgorithm {
    RS256,
    ES256,
}

lazy_static! {
    static ref JWT_CACHE: Mutex<HashMap<String, Claims>> = Mutex::new(HashMap::new());
}

pub fn create_jwt(claims: &Claims, key: &[u8], alg: JwtAlgorithm) -> Result<String, Box<dyn Error>> {
    let (encoding_key, algorithm) = match alg {
        JwtAlgorithm::RS256 => (EncodingKey::from_rsa_pem(key)?, Algorithm::RS256),
        JwtAlgorithm::ES256 => (EncodingKey::from_ec_pem(key)?, Algorithm::ES256),
    };

    let token = encode(&Header::new(algorithm), claims, &encoding_key)?;
    Ok(token)
}

pub fn validate_jwt(token: &str, key: &[u8], alg: JwtAlgorithm) -> Result<Claims, Box<dyn Error>> {
    // Önce önbelleği kontrol et
    if let Some(cached_claims) = JWT_CACHE.lock()?.get(token) {
        return Ok(cached_claims.clone());
    }

    let (decoding_key, algorithm) = match alg {
        JwtAlgorithm::RS256 => (DecodingKey::from_rsa_pem(key)?, Algorithm::RS256),
        JwtAlgorithm::ES256 => (DecodingKey::from_ec_pem(key)?, Algorithm::ES256),
    };

    let validation = Validation::new(algorithm);
    let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
    let claims = decoded_token.claims.clone();

    // Önbelleğe ekle
    JWT_CACHE.lock()?.insert(token.to_string(), claims.clone());

    Ok(claims)
}
