use crate::tools::traits::EvmTools;
use serde::{Deserialize, Serialize};
use std::marker::Send;

#[allow(dead_code)]
pub struct Context<T>
where
    T: EvmTools + Send,
{
    pub m_tool: T,
    pub cfg: Config,
}

impl<T: EvmTools + Send> Context<T> {
    pub fn new(m_tool: T, cfg: Config) -> Self {
        Self { m_tool, cfg }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    server_address: String,
    server_port: u32,
    pub(crate) eth_rpc: String,
    pub(crate) brave_api_key: String,
    pub(crate) zero_x_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        // Check if config file exists
        tracing::info!("Creating Config struct");
        Self::default()
    }
}

/// Get all configuration fields from environment variables for now. If this gets bigger
/// it can be persisted to disk as file using Serialize.
impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: String::from("0.0.0.0"),
            server_port: 3000u32,
            eth_rpc: String::from("http://127.0.0.1:8545"),
            brave_api_key: String::from("BSAurYK60YLTxZbBwMI2YhR-uIAVMAm"),
            zero_x_api_key: String::from("a1672326-a68f-4dec-bd19-1dd1cd4fa1d0"),
        }
    }
}
