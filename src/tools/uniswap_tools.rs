//! Uniswap V2 Router integration module.
//!
//! This module provides tools for executing token swaps on Uniswap V2 through the router contract.
//! It implements the core swap functionality including ETH-to-token and token-to-ETH exchanges
//! with automatic slippage protection and balance validation.
//!
//! # Features
//!
//! - **ETH ⟷ ERC20 swaps**: Seamless conversion between ETH and any ERC20 token
//! - **Slippage protection**: Automatic minimum output calculation with 10% safety margin
//! - **Balance validation**: Pre-transaction checks to prevent insufficient fund failures
//! - **Gas estimation**: Accounts for transaction costs in balance calculations
//!
//! # Key Functions
//!
//! - [`swap_eth_to_token`]: Convert ETH to ERC20 tokens using exact input amounts
//! - [`swap_token_to_eth`]: Convert ERC20 tokens to ETH (requires prior token approval)
//! - [`check_balance`]: Validate account has sufficient funds including gas costs
//!
//! # Usage
//!
//! ```rust
//! // ETH to Token swap
//! let eth_input = SwapEthInput {
//!     uniswap_address: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
//!     amount_in: "1.0".to_string(), // 1 ETH
//!     min_amount_out: "1000000000000000000".to_string(), // Expected tokens in base units
//!     to_token_addr: "0xA0b86a33E6441...".to_string(),
//!     account_addr: "0x742d35Cc6aF4...".to_string(),
//! };
//!
//! multi_tool.swap_eth_to_token(eth_input).await?;
//! ```
//!
//! # Important Notes
//!
//! - Token-to-ETH swaps require prior approval of the Uniswap Router to spend tokens
//! - All amounts are automatically adjusted for 10% slippage tolerance
//! - Transactions include 5-minute deadline for execution
//! - WETH conversion is handled automatically by the router contract
use crate::tools::MultiTool;
use crate::tools::traits::{EvmTools, UniSwapTools};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use rmcp::schemars;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

// Uniswap V2 Router contract interface generated from the ABI.
//
// This includes the main swap functions from the Uniswap V2 Router contract.
// Only implements essential swap functions to reduce the number of tools added to the server.
//
// # Functions
// - `swapExactETHForTokens`: Swap exact amount of ETH for as many tokens as possible
// - `swapETHForExactTokens`: Swap ETH for exact amount of tokens
// - `swapExactTokensForETH`: Swap exact amount of tokens for as much ETH as possible
// - `swapTokensForExactETH`: Swap tokens for exact amount of ETH
// - `swapExactTokensForTokens`: Swap exact amount of tokens for as many other tokens as possible
// - `swapTokensForExactTokens`: Swap tokens for exact amount of other tokens
abigen!(
    UniswapV2Router,
    r#"[
        function swapExactETHForTokens(uint amountOutMin, address[] path, address to, uint deadline) payable returns (uint[] memory amounts)
        function swapETHForExactTokens(uint amountOut, address[] path, address to, uint deadline) payable returns (uint[] memory amounts)
        function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] path, address to, uint deadline) returns (uint[] memory amounts)
        function swapTokensForExactETH(uint amountOut, uint amountInMax, address[] path, address to, uint deadline) returns (uint[] memory amounts)
        function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] path, address to, uint deadline) returns (uint[] memory amounts)
        function swapTokensForExactTokens(uint amountOut, uint amountInMax, address[] path, address to, uint deadline) returns (uint[] memory amounts)
    ]"#
);

/// Input parameters for swapping ETH to ERC20 tokens on Uniswap V2.
///
/// This struct contains all the necessary parameters to execute an ETH-to-token swap
/// transaction through the Uniswap V2 Router contract.
///
/// # Fields
/// - `uniswap_address`: The contract address of the Uniswap V2 Router (0x7a250d5630b4cf539739df2c5dacb4c659f2488d on mainnet)
/// - `min_amount_out`: Minimum tokens expected to receive (slippage protection, in token's base units)
/// - `amount_in`: Amount of ETH to swap (in ETH units, not wei - will be converted internally)
/// - `to_token_addr`: Contract address of the ERC20 token to receive
/// - `account_addr`: Ethereum address that will receive the tokens and pay for the transaction
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SwapEthInput {
    #[schemars(description = "Uniswap V2 router contract address")]
    pub uniswap_address: String,
    #[schemars(
        description = "The minimum amount of expected tokens to be swapped for based on rate"
    )]
    pub min_amount_out: String,
    #[schemars(description = "The amount of tokens to be swapped from the sender in ETH not wei")]
    pub amount_in: String,
    #[schemars(description = "The output token address or contract")]
    pub to_token_addr: String,
    #[schemars(description = "The address where funds will be swapped from")]
    pub account_addr: String,
}

/// Input parameters for swapping ERC20 tokens to ETH on Uniswap V2.
///
/// This struct contains all the necessary parameters to execute a token-to-ETH swap
/// transaction through the Uniswap V2 Router contract.
///
/// **Important**: Before executing this swap, the token contract must have approved
/// the Uniswap Router to spend the specified amount of tokens.
///
/// # Fields
/// - `uniswap_address`: The contract address of the Uniswap V2 Router
/// - `amount_in`: Amount of tokens to swap (in token's base units - e.g., for USDC with 6 decimals, use "1000000" for 1 USDC)
/// - `min_amount_out`: Minimum ETH expected to receive (in ETH units, will be converted to wei internally)
/// - `from_token_addr`: Contract address of the ERC20 token being swapped
/// - `account_addr`: Ethereum address that owns the tokens and will receive the ETH
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SwapTokenInput {
    #[schemars(description = "Uniswap V2 router contract address")]
    pub uniswap_address: String,
    #[schemars(description = "The amount of tokens expected to sell for eth")]
    pub amount_in: String,
    #[schemars(description = "The minimum amount of eth expected to receive in ETH")]
    pub min_amount_out: String,
    #[schemars(description = "The input token address or contract being swapped for ETH")]
    pub from_token_addr: String,
    #[schemars(description = "The address where funds will be swapped from")]
    pub account_addr: String,
}

impl UniSwapTools for MultiTool {
    /// Swaps ETH for ERC20 tokens using Uniswap V2.
    ///
    /// This function executes a swap from ETH to any ERC20 token using the `swapExactETHForTokens`
    /// function from the Uniswap V2 Router. The function automatically handles ETH to WETH conversion
    /// internally within the router contract.
    ///
    /// # Arguments
    /// * `input` - SwapEthInput struct containing swap parameters
    ///
    /// # Returns
    /// * `Result<String>` - Success message with transaction hash and gas used, or error
    async fn swap_eth_to_token(&self, input: SwapEthInput) -> anyhow::Result<String> {
        tracing::info!("Swapping Eth for Token");
        let token_addr = Address::from_str(&input.to_token_addr)?;
        let account_addr = Address::from_str(&input.account_addr)?;
        let weth_addr = Address::from_str(super::WETH_TOKEN_ADDRESS)?;
        let contract_addr = Address::from_str(&input.uniswap_address)?;

        let eth_amount_in = parse_ether(&input.amount_in)?;

        // Calculate minimum tokens out with slippage (e.g., 5% slippage = accept 95% of expected)
        // This value might be 0 if the 0x protocol api isn't available.
        let expected_tokens_out = U256::from_dec_str(&input.min_amount_out)?;

        // Calculate 90% directly (remove 10%) (SLIPPAGE COST) - Trying to allow swaps to go through since this is a test account
        let min_tokens_out = expected_tokens_out * U256::from(90) / U256::from(100);
        tracing::info!("Min TOKEN EXPECTED: {min_tokens_out}");

        let path = vec![weth_addr, token_addr];
        let deadline = U256::from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300);

        // Check account balance first
        let balance = self.get_balance(input.account_addr).await?;
        self.check_balance(eth_amount_in, balance).await?;

        let contract = UniswapV2Router::new(contract_addr, self.eth_provider.clone());

        // Build transaction with input values
        let tx = contract
            .swap_exact_eth_for_tokens(
                min_tokens_out, // Minimum tokens to accept (slippage protection)
                path,
                account_addr,
                deadline,
            )
            .value(eth_amount_in); // ETH amount to swap

        // Send transaction and wait for confirmation
        let pending_tx = tx.send().await?;
        let receipt = pending_tx
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!(
            "Transaction successful! Hash: {:?}, Gas used: {:?}",
            receipt.transaction_hash, receipt.gas_used
        ))
    }
    /// Swaps ERC20 tokens for ETH using Uniswap V2.
    ///
    /// This function executes a swap from any ERC20 token to ETH using the `swapExactTokensForETH`
    /// function from the Uniswap V2 Router. The function automatically handles WETH to ETH conversion
    /// internally within the router contract.
    ///
    /// **IMPORTANT**: Before calling this function, the token contract must have approved the
    /// Uniswap Router to spend the specified amount of tokens. Without approval, the transaction
    /// will fail with `TRANSFER_FROM_FAILED` error.
    ///
    /// # Arguments
    /// * `input` - SwapTokenInput struct containing swap parameters
    ///
    /// # Returns
    /// * `Result<String>` - Success message with transaction hash and gas used, or error
    async fn swap_token_to_eth(&self, input: SwapTokenInput) -> anyhow::Result<String> {
        tracing::info!("Swapping Token for ETH");
        let from_token_addr = Address::from_str(&input.from_token_addr)?;
        let account_addr = Address::from_str(&input.account_addr)?;
        let weth_addr = Address::from_str(super::WETH_TOKEN_ADDRESS)?;
        let contract_addr = Address::from_str(&input.uniswap_address)?;

        let token_amount_in = U256::from_dec_str(&input.amount_in)?;

        // Calculate minimum tokens out with slippage (e.g., 5% slippage = accept 95% of expected)
        // This value might be 0 if the 0x protocol api isn't available.
        let expected_tokens_out = parse_ether(&input.min_amount_out)?;

        // Calculate 90% directly (remove 10%) (SLIPPAGE COST)
        let min_tokens_out = expected_tokens_out * U256::from(90) / U256::from(100);
        tracing::info!("Min TOKEN EXPECTED: {min_tokens_out}");

        let path = vec![from_token_addr, weth_addr];
        let deadline = U256::from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300);

        // Check account balance first
        // let balance = self.get_erc20_balance(input.from_token_addr, input.account_addr).await?;
        // self.check_balance(token_amount_in, balance).await?;

        let contract = UniswapV2Router::new(contract_addr, self.eth_provider.clone());

        // Build transaction with input values
        let tx = contract.swap_exact_tokens_for_eth(
            token_amount_in,
            min_tokens_out, // Minimum tokens to accept (slippage protection)
            path,
            account_addr,
            deadline,
        );

        // Send transaction and wait for confirmation
        let pending_tx = tx.send().await?;
        let receipt = pending_tx
            .await?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;

        Ok(format!(
            "Transaction successful! Hash: {:?}, Gas used: {:?}",
            receipt.transaction_hash, receipt.gas_used
        ))
    }

    /// Validates that an account has sufficient balance to cover a transaction amount plus gas fees.
    ///
    /// This function checks if the account has enough ETH to cover both the swap amount and
    /// estimated gas costs. It helps prevent transaction failures due to insufficient funds.
    ///
    /// # Arguments
    /// * `amount_in` - The amount of ETH required for the swap (in wei)
    /// * `balance` - The current account balance as a string (in wei)
    ///
    /// # Returns
    /// * `Result<()>` - Ok if balance is sufficient, Err with details if insufficient
    async fn check_balance(&self, amount_in: U256, balance: String) -> anyhow::Result<()> {
        let bal = U256::from_dec_str(&balance)?;
        let gas_estimate = U256::from(200_000); // Rough estimate
        let gas_price = self.eth_provider.get_gas_price().await?;
        let estimated_gas = gas_estimate * gas_price;

        if bal < amount_in + estimated_gas {
            return Err(anyhow::anyhow!(
                "Insufficient balance. Need {} for swap + {} for gas. Balance: {} ",
                ethers::utils::format_ether(amount_in),
                ethers::utils::format_ether(estimated_gas),
                ethers::utils::format_ether(bal)
            ));
        }
        Ok(())
    }
}
