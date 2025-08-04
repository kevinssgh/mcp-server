use crate::common::accounts::Accounts;
use ethers::prelude::{Http, Provider};

pub mod agent_mcp;
pub mod eth_tools;
pub mod traits;

pub struct MultiTool {
    pub provider: Provider<Http>,
    pub accounts: Accounts,
}

impl MultiTool {
    pub fn new(eth_rpc: &str) -> Self {
        tracing::info!("Creating ETH provider");
        let provider =
            Provider::<Http>::try_from(eth_rpc).expect("should build provider to local eth node");
        Self {
            provider,
            accounts: Accounts::new(),
        }
    }
}
