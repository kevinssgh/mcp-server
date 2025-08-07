pub mod accounts;
pub mod context;

const ENV_SERVER_ADDRESS: &str = "MCP_SERVER_ADDRESS";
const ENV_SERVER_PORT: &str = "MCP_SERVER_PORT";
const ENV_ETH_RPC: &str = "ETH_RPC";
const ENV_BRAVE_API_KEY: &str = "BRAVE_API_KEY";
const ENV_ZERO_X_API_KEY: &str = "ZERO_X_API_KEY";

pub fn get_env_var(name: &str) -> anyhow::Result<String> {
    let var = std::env::var(name)?;
    Ok(var)
}

pub fn get_bind_address() -> anyhow::Result<String> {
    let addr = get_env_var(ENV_SERVER_ADDRESS)?;
    let port = get_env_var(ENV_SERVER_PORT)?;
    Ok(format!("{addr}:{port}"))
}
