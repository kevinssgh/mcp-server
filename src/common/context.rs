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

// "0.0.0.0:3000"
// eth_rpc: String::from("http://127.0.0.1:8545"),
// brave_api_key: String::from("BSAurYK60YLTxZbBwMI2YhR-uIAVMAm"),
// zero_x_api_key: String::from("a1672326-a68f-4dec-bd19-1dd1cd4fa1d0"),
// }
