// src/oauth/resource_owner_password.rs

use crate::oauth::token::OAuthToken;
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct ResourceOwnerPasswordFlow {
    client_id: String,
    client_secret: String,
    token_url: String,
}

impl ResourceOwnerPasswordFlow {
    pub fn new(client_id: &str, client_secret: &str, token_url: &str) -> Self {
        ResourceOwnerPasswordFlow {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token_url: token_url.to_string(),
        }
    }

    pub async fn request_token(
        &self,
        username: &str,
        password: &str,
    ) -> Result<OAuthToken, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
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
