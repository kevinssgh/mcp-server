//! # Traits Module
//!
//! This module defines the core trait interfaces used by the Agent to interact with various
//! blockchain and external services. These traits provide a standardized API for different
//! tool implementations, enabling modular and testable code architecture.
//!
//! ## Overview
//!
//! The traits in this module abstract away the implementation details of different services,
//! allowing the Agent to work with any implementation that satisfies these interfaces. This
//! design promotes loose coupling and makes it easy to swap implementations or add new
//! service providers.
//!
//! ## Traits
//!
//! ### `EvmTools`
//! EVM blockchain interaction interface providing:
//! - Balance queries for native tokens and ERC20 tokens
//! - Transaction sending capabilities
//! - Smart contract information retrieval
//!
//! ### `BraveTools`
//! Web search interface providing:
//! - Search query execution via Brave Search API
//! - Returns formatted search results as strings
//!
//! ### `ZeroXTools`
//! DEX aggregation interface providing:
//! - Token swap quote generation via 0x Protocol API
//! - Price discovery and routing optimization
//!
//! ### `UniSwapTools`
//! Uniswap protocol interface providing:
//! - Direct ETH ↔ Token swaps via Uniswap contracts
//! - Balance validation for swap operations
//! - On-chain transaction execution
//!
//! ## Usage Pattern
//!
//! These traits are typically implemented by the corresponding tool modules and used
//! by the Agent through dependency injection or trait objects, enabling clean separation
//! of concerns and testability.
//!
//! ## Note:
//! The separation of these Tools allows tool modules to selectively mock a particular
//! toolset in order to test the agent's response.
//! ```
use crate::tools::uniswap_tools::{SwapEthInput, SwapTokenInput};
use crate::tools::zero_x_tools::QuoteInput;
use anyhow::Result;
use ethers::prelude::U256;

/// Interface to evm related tools used by Agent.
pub(crate) trait EvmTools {
    async fn get_balance(&self, address: String) -> Result<String>;
    async fn send(&self, from: String, to: String, amount: String) -> Result<String>;
    async fn get_contract(&self, contract: String) -> Result<String>;
    async fn get_erc20_balance(&self, contract: String, account: String) -> Result<String>;
}

/// Interface to brave related tools used by Agent.
pub(crate) trait BraveTools {
    async fn search(&self, query: String) -> Result<String>;
}

/// Interface to 0x api for swap quotes.
pub(crate) trait ZeroXTools {
    async fn get_quote(&self, input: QuoteInput) -> Result<String>;
}

/// Interface to Uniswap contract abi.
pub(crate) trait UniSwapTools {
    async fn swap_eth_to_token(&self, swap_input: SwapEthInput) -> Result<String>;
    async fn swap_token_to_eth(&self, swap_input: SwapTokenInput) -> Result<String>;
    async fn check_balance(&self, amount_in: U256, balance: String) -> Result<()>;
}
