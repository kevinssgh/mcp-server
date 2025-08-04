use ethers::prelude::{Http, Provider};

pub mod agent_mcp;
pub mod eth_tools;
pub mod traits;

#[derive(Debug)]
pub struct MultiTool {
    pub provider: Provider<Http>,
}

impl MultiTool {
    pub fn new(eth_rpc: &str) -> Self {
        tracing::info!("Creating ETH provider");
        let provider =
            Provider::<Http>::try_from(eth_rpc).expect("should build provider to local eth node");
        Self { provider }
    }
}
