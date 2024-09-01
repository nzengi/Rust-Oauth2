// src/oauth/implicit.rs

use super::OAuthToken;

// Defines the necessary parameters for the implicit flow
#[derive(Debug)]
pub struct ImplicitFlow {
    client_id: String,
    redirect_uri: String,
    response_type: String,
}

impl ImplicitFlow {
    pub fn new(client_id: &str, redirect_uri: &str) -> Self {
        ImplicitFlow {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
            response_type: "token".to_string(),
        }
    }

    pub fn generate_authorization_url(&self, auth_url: &str, state: Option<&str>) -> String {
        let state_param = state.map_or("".to_string(), |s| format!("&state={}", s));
        format!(
            "{}?response_type={}&client_id={}&redirect_uri={}{}",
            auth_url, self.response_type, self.client_id, self.redirect_uri, state_param
        )
    }
}
