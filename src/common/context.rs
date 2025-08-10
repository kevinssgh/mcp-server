//! # Context Module
//!
//! This module provides the core context and configuration management for the application.
//! It defines the generic `Context` wrapper and configuration structures needed to initialize
//! and manage the various tools and services used throughout the system.
//!
//! ## Core Components
//!
//! ### `Context<T>`
//! A generic wrapper that holds a multi-tool instance implementing all required trait interfaces.
//! The type parameter `T` must implement:
//! - `EvmTools`: For Ethereum blockchain interactions
//! - `BraveTools`: For web search capabilities
//! - `UniSwapTools`: For Uniswap protocol operations
//! - `ZeroXTools`: For 0x protocol integration
//! - `Send`: For thread safety across async boundaries
//!
//! This design allows for dependency injection and makes the system highly testable by
//! accepting any implementation that satisfies the required trait bounds.
//!
//! ### `Config`
//! Configuration structure that manages all external service credentials and endpoints:
//! - **ETH RPC URL**: Ethereum node connection endpoint
//! - **Brave API Key**: Authentication for Brave Search API
//! - **0x API Key**: Authentication for 0x Protocol API
//!
//! ## Configuration Management
//!
//! The `Config` struct automatically loads all required configuration from environment variables:
//! - `ETH_RPC`: Ethereum RPC endpoint URL
//! - `BRAVE_API_KEY`: Brave Search API authentication key
//! - `ZERO_X_API_KEY`: 0x Protocol API authentication key
//!
//! All environment variables are required and the application will panic on startup if any
//! are missing, ensuring fail-fast behavior for configuration issues.
//!
//! ## Usage
//!
//! ```rust
//! // Load configuration from environment
//! let config = Config::new();
//!
//! // Create multi-tool with config
//! let multitool = MultiTool::new(&config);
//!
//! // Wrap in context for dependency injection
//! let context = Context::new(multitool);
//! ```
//!
//! ## Future Extensibility
//!
//! The `Config` struct is designed with serialization support (`Serialize`/`Deserialize`)
//! to enable future persistence to disk-based configuration files as the configuration
//! grows in complexity.
use crate::common::{ENV_BRAVE_API_KEY, ENV_ETH_RPC, ENV_ZERO_X_API_KEY, get_env_var};
use crate::tools::traits::{BraveTools, EvmTools, UniSwapTools, ZeroXTools};
use serde::{Deserialize, Serialize};
use std::marker::Send;

pub struct Context<T>
where
    T: EvmTools + BraveTools + UniSwapTools + ZeroXTools + Send,
{
    pub m_tool: T,
}

impl<T: EvmTools + BraveTools + UniSwapTools + ZeroXTools + Send> Context<T> {
    pub fn new(m_tool: T) -> Self {
        Self { m_tool }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) eth_rpc: String,
    pub(crate) brave_api_key: String,
    pub(crate) zero_x_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Get all configuration fields from environment variables for now. If this gets bigger
/// it can be persisted to disk as file using Serialize.
impl Default for Config {
    fn default() -> Self {
        Self {
            eth_rpc: get_env_var(ENV_ETH_RPC).expect("ETH_RPC not set"),
            brave_api_key: get_env_var(ENV_BRAVE_API_KEY).expect("BRAVE_API_KEY not set"),
            zero_x_api_key: get_env_var(ENV_ZERO_X_API_KEY).expect("ZERO_X_API_KEY not set"),
        }
    }
}