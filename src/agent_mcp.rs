use std::str::FromStr;
use crate::context::Context;
use anyhow::Result;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{Implementation, ProtocolVersion};
use rmcp::{ServerHandler, model::*, tool_handler, tool_router, tool};
use std::sync::Arc;
use ethers::middleware::Middleware;
use ethers::prelude::{Address, NameOrAddress};
use tokio::sync::Mutex;

// Main server struct that implements ServerHandler
#[allow(dead_code)] // ignore some warnings that aren't helpful
#[derive(Debug, Clone)]
pub struct AgentMcpServer {
    // Internal state - Contains server context, behind Atomic Reference and Mutex for thread safety
    pub(crate) ctx: Arc<Mutex<Context>>,
    // Tool Router
    tool_router: ToolRouter<AgentMcpServer>,
}

#[tool_router]
impl AgentMcpServer {
    pub fn new() -> Self {
        AgentMcpServer {
            ctx: Arc::new(Mutex::new(Context::new())),
            tool_router: Self::tool_router(),
        }
    }

    // Balance command
    #[tool(description = "Get the balance of an account in wei")]
    async fn balance(
        &self,
        Parameters(address): Parameters<crate::tools::eth_tools::BalanceInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        //let addr = NameOrAddress::from(&address.addr);
        let ad = Address::from_str(&address.addr).unwrap();
        let balance = self
            .ctx
            .lock()
            .await
            .provider
            .get_balance(ad, None)
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("failed to get balance: {}", e.to_string()), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(
            balance.to_string(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for AgentMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides a counter tool that can increment and decrement values. The counter starts at 0 and can be modified using the 'increment' and 'decrement' tools. Use 'get_value' to check the current count.".to_string()),
        }
    }
}
