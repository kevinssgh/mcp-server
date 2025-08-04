use serde::{Deserialize, Serialize};
use std::marker::Send;

use crate::tools::traits::EvmTools;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Context<T>
where T: EvmTools + Send
{
    pub m_tool: T,
    pub cfg: Config,
}

impl <T: EvmTools + Send> Context<T> {
    pub fn new(m_tool: T, cfg: Config) -> Self {
        Self {
            m_tool,
            cfg
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    server_address: String,
    server_port: u32,
    pub(crate) eth_rpc: String,
}

impl Config {
    pub fn new() -> Self {
        // Check if config file exists
        tracing::info!("Creating Config struct");
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: String::from("0.0.0.0"),
            server_port: 3000u32,
            eth_rpc: String::from("http://127.0.0.1:8545"),
        }
    }
}
