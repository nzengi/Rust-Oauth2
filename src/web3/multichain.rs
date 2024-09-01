// src/web3/multichain.rs

use web3::transports::Http;
use web3::Web3;
use std::collections::HashMap;
use std::error::Error;

pub struct MultiChainClient {
    clients: HashMap<String, Web3<Http>>,
}

impl MultiChainClient {
    pub async fn new(urls: Vec<(&str, &str)>) -> Result<Self, Box<dyn Error>> {
        let mut clients = HashMap::new();
        for (name, url) in urls {
            let transport = Http::new(url)?;
            clients.insert(name.to_string(), Web3::new(transport));
        }
        Ok(MultiChainClient { clients })
    }

    pub async fn get_balance(
        &self,
        chain_name: &str,
        address: &str,
    ) -> Result<web3::types::U256, Box<dyn Error>> {
        if let Some(client) = self.clients.get(chain_name) {
            let balance = client.eth().balance(address.parse()?, None).await?;
            Ok(balance)
        } else {
            Err(Box::from("Chain not found"))
        }
    }
}
