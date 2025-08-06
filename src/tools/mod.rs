use crate::common::accounts::Accounts;
use crate::common::context::Config;
use crate::tools::brave_tools::BraveContext;
use ethers::prelude::{Http, Provider};
use std::sync::Arc;

pub mod agent_mcp;
mod brave_tools;
pub mod eth_tools;
pub mod traits;

pub struct MultiTool {
    pub eth_provider: Arc<Provider<Http>>,
    pub accounts: Accounts,
    pub brave_ctx: BraveContext,
}

impl MultiTool {
    pub fn new(cfg: &Config) -> Self {
        tracing::info!("Creating ETH provider");
        let provider = Provider::<Http>::try_from(cfg.eth_rpc.clone())
            .expect("should build provider to local eth node");
        Self {
            eth_provider: Arc::new(provider),
            accounts: Accounts::default(),
            brave_ctx: BraveContext::new(cfg.brave_api_key.clone()),
        }
    }
}
