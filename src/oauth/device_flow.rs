use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

pub struct DeviceFlow {
    client_id: String,
    device_code_url: String,
    token_url: String,
}

impl DeviceFlow {
    pub fn new(client_id: &str, device_code_url: &str, token_url: &str) -> Self {
        DeviceFlow {
            client_id: client_id.to_string(),
            device_code_url: device_code_url.to_string(),
            token_url: token_url.to_string(),
        }
    }

    pub async fn request_device_code(&self) -> Result<DeviceCodeResponse, Box<dyn Error>> {
        let client = Client::new();
        let params = [("client_id", &self.client_id)];

        let response = client
            .post(&self.device_code_url)
            .form(&params)
            .send()
            .await?;

        let device_code_response: DeviceCodeResponse = response.json().await?;
        Ok(device_code_response)
    }

    pub async fn poll_for_token(
        &self,
        device_code: &str,
        interval: u64,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("device_code", device_code),
            ("client_id", &self.client_id),
        ];

        loop {
            let response = client
                .post(&self.token_url)
                .form(&params)
                .send()
                .await?;

            if response.status().is_success() {
                let token: String = response.json().await?;
                return Ok(Some(token));
            } else if response.status() == 400 {
                // Belirli hata kodları (örneğin, slow down, authorization_pending, vb.) burada işlenebilir
            }

            sleep(Duration::from_secs(interval)).await;
        }
    }
}
