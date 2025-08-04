use crate::tools::agent_mcp::AgentMcpServer;
use ethers::prelude::Middleware;
use ethers::types::NameOrAddress;
use rmcp::handler::server::tool::Parameters;
use rmcp::{ErrorData as McpError, model::*, schemars, tool};
use std::vec;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BalanceInput {
    #[schemars(description = "The address or ENS name to check the balance for")]
    pub addr: String,
}


