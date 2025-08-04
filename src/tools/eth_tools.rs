use std::str::FromStr;
use ethers::prelude::*;
use rmcp::schemars;
use anyhow::{anyhow, Result};

use crate::tools::MultiTool;
use crate::tools::traits::EvmTools;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BalanceInput {
    #[schemars(description = "The address or ENS name to check the balance for")]
    pub addr: String,
}

impl EvmTools for MultiTool {
    async fn get_balance(&self, address: String) -> Result<String> {
        let addr = Address::from_str(&address)?;
        let balance = self.provider.get_balance(addr, None).await.map_err(
            |e| {
                // Add tracing
                anyhow!("failed to get balance from {}: {}", address, e.to_string())
            }
        )?;
        Ok(balance.to_string())
    }

    async fn send(&self ) -> String {
        todo!()
    }

    async fn get_contract(&self ) -> String {
        todo!()
    }
}