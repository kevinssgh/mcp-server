use ethers::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Context {
    pub provider: Provider<Http>,
    pub cfg: Config,
}

impl Context {
    pub fn new() -> Self {
        let cfg = Config::new();
        tracing::info!("Creating ETH provider");
        let provider = Provider::<Http>::try_from(&cfg.eth_rpc)
            .expect("should build provider to local eth node");

        Self { provider, cfg }
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
