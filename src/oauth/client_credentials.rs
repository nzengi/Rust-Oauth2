// src/oauth/client_credentials.rs

use super::OAuthToken;
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct ClientCredentialsFlow {
    client_id: String,
    client_secret: String,
    token_url: String,
}

impl ClientCredentialsFlow {
    pub fn new(client_id: &str, client_secret: &str, token_url: &str) -> Self {
        ClientCredentialsFlow {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token_url: token_url.to_string(),
        }
    }

    pub async fn request_token(&self) -> Result<OAuthToken, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let response = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token: OAuthToken = response.json().await?;
            Ok(token)
        } else {
            Err(Box::from("Failed to request token"))
        }
    }
}
