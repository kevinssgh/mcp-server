//! # MultiTool Module
//!
//! This module provides a unified interface for interacting with multiple blockchain and web services
//! through the `MultiTool` struct. It aggregates various tools and contexts needed for DeFi operations,
//! web searches, and Ethereum blockchain interactions.
//!
//! ## Components
//!
//! - **Ethereum Provider**: Connection to Ethereum RPC endpoint for blockchain operations
//! - **Account Management**: Handles account-related functionality
//! - **Brave Search Integration**: Provides web search capabilities via Brave API
//! - **0x Protocol Integration**: Enables decentralized exchange functionality
//!
//! ## Submodules
//!
//! - `agent_mcp`: MCP (Model Context Protocol) agent functionality
//! - `brave_tools`: Brave search API integration tools
//! - `eth_tools`: Ethereum blockchain interaction utilities
//! - `zero_x_tools`: 0x protocol integration for DEX operations
//! - `uniswap_tools`: Uniswap protocol integration tools
//! - `traits`: Common traits and interfaces
//!
//! ## Constants
//!
//! - `DEFAULT_ETH_TOKEN_ADDRESS`: Default Ethereum token address (ETH placeholder)
//! - `WETH_TOKEN_ADDRESS`: Wrapped Ethereum (WETH) contract address on mainnet
//!
//! ## Usage
//!
//! ```rust
//! let config = Config::load();
//! let multitool = MultiTool::new(&config);
//! // Use multitool for various blockchain and web operations
//! ```
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
mod uniswap_tools;

const DEFAULT_ETH_TOKEN_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
const WETH_TOKEN_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

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
