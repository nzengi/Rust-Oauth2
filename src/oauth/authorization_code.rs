// src/oauth/authorization_code.rs

use super::OAuthToken;
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct AuthorizationCodeFlow {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    token_url: String,
}

impl AuthorizationCodeFlow {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str, token_url: &str) -> Self {
        AuthorizationCodeFlow {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            token_url: token_url.to_string(),
        }
    }

    pub async fn exchange_code(&self, code: &str) -> Result<OAuthToken, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.redirect_uri),
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
            Err(Box::from("Failed to exchange authorization code"))
        }
    }
}
