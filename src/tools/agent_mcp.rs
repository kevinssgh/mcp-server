//! Model Context Protocol (MCP) server for blockchain and DeFi operations.
//!
//! This module implements an MCP server that exposes blockchain interaction tools through
//! a standardized protocol interface. It provides a comprehensive toolkit for Ethereum
//! operations, DeFi protocols, and web search capabilities accessible via natural language
//! through compatible AI agents.
//!
//! # Architecture
//!
//! - **AgentMcpServer**: Main server struct implementing the MCP ServerHandler protocol
//! - **Thread-safe context**: Shared state management using Arc<Mutex<Context<MultiTool>>>
//! - **Tool routing**: Automatic tool discovery and routing using procedural macros
//! - **Error handling**: Standardized MCP error responses with detailed context
//!
//! # Available Tools
//!
//! ## Ethereum Operations
//! - **`balance`**: Query ETH balance for any address
//! - **`send`**: Transfer ETH between addresses with transaction confirmation
//! - **`get_contract`**: Verify contract deployment and inspect bytecode
//! - **`get_erc20_balance`**: Check ERC20 token balances
//!
//! ## DeFi Protocol Integration
//! - **`get_quote`**: Get swap quotes from 0x Protocol aggregator
//! - **`swap_eth_for_tokens`**: Execute ETH-to-token swaps via Uniswap V2
//! - **`swap_tokens_for_eth`**: Execute token-to-ETH swaps via Uniswap V2
//!
//! ## Web Search
//! - **`web_search`**: Search for contract addresses and blockchain information
//!
//! # Usage
//!
//! The server is designed to be used with MCP-compatible AI agents that can call tools
//! through natural language requests:
//!
//! ```text
//! User: "What's the ETH balance of vitalik.eth?"
//! Agent: calls balance tool with ENS resolution
//!
//! User: "Swap 1 ETH for USDC on Uniswap"
//! Agent: calls get_quote then swap_eth_for_tokens
//!
//! User: "Send 0.5 ETH to Alice"
//! Agent: calls send tool with specified parameters
//! ```
//!
//! # Server Capabilities
//!
//! - **Tools**: All blockchain and DeFi operations
//! - **Prompts**: Standardized prompt templates (enabled for future use)
//! - **Resources**: Access to blockchain data and contract information
//! - **Protocol**: Full MCP protocol compliance with latest version support
//!
//! # Key Features
//!
//! - **Async operations**: Non-blocking blockchain interactions
//! - **Comprehensive error handling**: Detailed error messages with operation context
//! - **Thread safety**: Concurrent access support for multiple agent requests
//! - **Extensible architecture**: Easy addition of new tools and capabilities
//! - **Protocol compliance**: Standard MCP interface for broad agent compatibility
//!
//! # Dependencies
//!
//! Built on the `rmcp` (Rust MCP) framework with integration to `ethers` for blockchain
//! operations, providing a robust foundation for DeFi automation and blockchain tooling.
use anyhow::Result;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{Implementation, ProtocolVersion};
use rmcp::{ServerHandler, model::*, tool, tool_handler, tool_router};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::common::context::{Config, Context};
use crate::tools::MultiTool;
use crate::tools::traits::{BraveTools, EvmTools, UniSwapTools, ZeroXTools};

// Main server struct that implements ServerHandler
#[derive(Clone)]
pub struct AgentMcpServer {
    // Internal state - Contains server context, behind Atomic Reference and Mutex for thread safety
    pub(crate) ctx: Arc<Mutex<Context<MultiTool>>>,
    // Tool Router
    tool_router: ToolRouter<AgentMcpServer>,
}

#[tool_router]
impl AgentMcpServer {
    pub fn new() -> Self {
        let cfg = Config::new();
        let m_tool = MultiTool::new(&cfg);

        AgentMcpServer {
            ctx: Arc::new(Mutex::new(Context::new(m_tool))),
            tool_router: Self::tool_router(),
        }
    }

    // Balance command
    #[tool(description = "Get the balance of an account in wei")]
    async fn balance(
        &self,
        Parameters(address): Parameters<super::eth_tools::BalanceInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let balance = self
            .ctx
            .lock()
            .await
            .m_tool
            .get_balance(address.addr)
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("server failed to get balance: {e}"), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(balance)]))
    }

    // Balance command
    #[tool(description = "Sends an amount in ETH from one address to another")]
    async fn send(
        &self,
        Parameters(input): Parameters<super::eth_tools::SendInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let receipt = self
            .ctx
            .lock()
            .await
            .m_tool
            .send(input.sender, input.receiver, input.amount)
            .await
            .map_err(|e| ErrorData::internal_error(format!("server failed to send: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(receipt)]))
    }

    // Verify whether a contract is deployed
    #[tool(description = "Checks whether a contract is deployed given the address")]
    async fn get_contract(
        &self,
        Parameters(input): Parameters<super::eth_tools::GetContractInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .get_contract(input.addr)
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("server failed to get contract: {e}"), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
    }

    // ERC20 Balance command
    #[tool(
        description = "Gets the balance of an address for a specific erc20 token using its defined denominations"
    )]
    async fn get_erc20_balance(
        &self,
        Parameters(input): Parameters<super::eth_tools::ERC20BalanceInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .get_erc20_balance(input.erc20_addr, input.account)
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("server failed to get erc20 balance: {e}"), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
    }

    // Perform web search for contract addresses
    #[tool(description = "Searches the web for different types of contract addresses")]
    async fn web_search(
        &self,
        Parameters(input): Parameters<super::brave_tools::WebSearchInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .search(input.query)
            .await
            .map_err(|e| ErrorData::internal_error(format!("web search failed: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
    }

    // Get Swap quote from 0x Protocol
    #[tool(description = "Gets a quote for a swap from one token type to another")]
    async fn get_quote(
        &self,
        Parameters(input): Parameters<super::zero_x_tools::QuoteInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .get_quote(input)
            .await
            .map_err(|e| ErrorData::internal_error(format!("quote request failed: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
    }

    // Use uniswap tools to swap eth for another token type
    #[tool(description = "Swaps ETH tokens for a specified output token")]
    async fn swap_eth_for_tokens(
        &self,
        Parameters(input): Parameters<super::uniswap_tools::SwapEthInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .swap_eth_to_token(input)
            .await
            .map_err(|e| ErrorData::internal_error(format!("token swap failed: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
    }

    // Use uniswap tools to swap tokens for eth
    #[tool(description = "Swaps specific tokens for eth")]
    async fn swap_tokens_for_eth(
        &self,
        Parameters(input): Parameters<super::uniswap_tools::SwapTokenInput>,
    ) -> std::result::Result<CallToolResult, ErrorData> {
        let reply = self
            .ctx
            .lock()
            .await
            .m_tool
            .swap_token_to_eth(input)
            .await
            .map_err(|e| ErrorData::internal_error(format!("token swap failed: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(reply)]))
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
