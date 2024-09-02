// src/contract_audit.rs

use web3::types::TransactionReceipt;
use web3::Web3;
use std::error::Error;

pub struct ContractAudit;

impl ContractAudit {
    pub async fn audit_contract(
        web3: &Web3<web3::transports::Http>,
        contract_address: &str,
    ) -> Result<TransactionReceipt, Box<dyn Error>> {
        let receipt = web3.eth().transaction_receipt(contract_address.parse()?).await?;
        // Temel kontroller burada yapılır
        if receipt.status.is_none() || receipt.status.unwrap().is_zero() {
            return Err(Box::from("Contract audit failed"));
        }
        Ok(receipt)
    }
}
