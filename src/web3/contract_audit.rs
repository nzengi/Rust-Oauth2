// src/web3/contract_audit.rs

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

        // receipt'i unwrap yaparak TransactionReceipt alın
        let receipt = receipt.unwrap();

        let status = receipt.status;

        if status.is_none() || status.unwrap().is_zero() {
            return Err(Box::from("Contract audit failed"));
        }

        Ok(receipt)
    }
}
