use crate::agent_mcp::AgentMcpServer;
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

impl AgentMcpServer {
    #[tool(description = "Add two numbers together")]
    async fn add(&self, a: u32, b: u32) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "{}",
            a + b
        ))]))
    }

    // Send eth command

    // // Balance command
    // #[tool(description = "Get the balance of an account in wei")]
    // async fn balance(
    //     &self,
    //     Parameters(address): Parameters<BalanceInput>,
    // ) -> Result<CallToolResult, McpError> {
    //     let address = NameOrAddress::from(address.addr);
    //     let balance = self
    //         .ctx
    //         .lock()
    //         .await
    //         .provider
    //         .get_balance(address, None)
    //         .await
    //         .map_err(|e| {
    //             ErrorData::internal_error(format!("failed to get balance: {}", e.to_string()), None)
    //         })?;
    //     Ok(CallToolResult::success(vec![Content::text(
    //         balance.to_string(),
    //     )]))
    // }

    // Get contract command
}
