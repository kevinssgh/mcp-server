use ethers::prelude::{Http, Provider};
use std::sync::Arc;

use crate::common::accounts::Accounts;
use crate::common::context::Config;
use crate::tools::brave_tools::BraveContext;
use crate::tools::zero_x_tools::ZeroXContext;

pub mod agent_mcp;
mod brave_tools;
mod eth_tools;
mod zero_x_tools;

pub mod traits;

pub struct MultiTool {
    pub eth_provider: Arc<Provider<Http>>,
    pub accounts: Accounts,
    pub brave_ctx: BraveContext,
    pub zero_x_context: ZeroXContext,
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
            zero_x_context: ZeroXContext::new(cfg.zero_x_api_key.clone()),
        }
    }
}
