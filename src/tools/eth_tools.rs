use crate::tools::MultiTool;
use crate::tools::traits::EvmTools;
use anyhow::{Result, anyhow};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use rmcp::schemars;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BalanceInput {
    #[schemars(description = "The address or ENS name to check the balance for")]
    pub addr: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendInput {
    #[schemars(description = "The address or ENS name used to send ETH from")]
    pub sender: String,
    #[schemars(description = "The address or ENS name to send ETH to")]
    pub receiver: String,
    #[schemars(description = "The amount of ETH to send")]
    pub amount: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetContractInput {
    #[schemars(description = "The address of the contract to look for")]
    pub addr: String,
}

impl EvmTools for MultiTool {
    async fn get_balance(&self, address: String) -> Result<String> {
        let addr = Address::from_str(&address)?;
        let balance = self.provider.get_balance(addr, None).await.map_err(|e| {
            // Add tracing
            anyhow!("failed to get balance from {}: {}", address, e.to_string())
        })?;
        Ok(balance.to_string())
    }

    async fn send(&self, from: String, to: String, amount: String) -> Result<String> {
        let sender = Address::from_str(&from)?;
        let receiver = Address::from_str(&to)?;
        let amount = parse_ether(&amount)?;

        let wallet = match self.accounts.wallets.get(&sender) {
            None => Err(anyhow!("could not get wallet {from}")),
            Some(acc) => Ok(&acc.wallet),
        }?;

        // Initialize client
        let client = SignerMiddleware::new(&self.provider, wallet.clone());
        let tx = TransactionRequest::new()
            .to(NameOrAddress::Address(receiver))
            .value(amount);

        // Send transaction
        let pending_tx = client
            .send_transaction(tx, None)
            .await
            .map_err(|e| anyhow!("send transaction failed {e}"))?;
        let receipt = pending_tx
            .await
            .map_err(|e| anyhow!("send transaction failed {e}"))?;

        // Check receipt and return tx hash if it's ok
        let tx_hash = match receipt {
            None => Err(anyhow!("receipt was empty")),
            Some(r) => Ok(r.transaction_hash),
        }?;

        Ok(format!("{:x}, {}", tx_hash, tx_hash.to_string()))
    }

    async fn get_contract(&self, contract: String) -> Result<String> {
        let contract_addr = Address::from_str(&contract)?;
        let code = self.provider.get_code(contract_addr, None).await?;
        if !code.is_empty() {
            Ok(format!(
                "Contract {contract} is deployed (code size: {})",
                code.len()
            ))
        } else {
            Ok(format!("Contract {contract} is not deployed"))
        }
    }
}
