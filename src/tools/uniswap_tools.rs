use crate::tools::MultiTool;
use crate::tools::traits::UniSwapTools;
use ethers::prelude::*;
use ethers::utils::parse_ether;
use rmcp::schemars;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

// Generated main swap functions from Uniswap V2 Rounter. Normally would use entire json file but
// Will only implement  two of these in order to reduce the number of tools added to the server.
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

/// SwapEth input struct
///
///     Fields:
///         uniswap_address (String): Uniswap V2 Router contract address
///         min_amount_out (String): Minimum amount of the output token expected to receive
///         amount_in (String): Eth amount used for the swap
///         to_token_addr (String): ERC20 Token address of the output token
///         account_addr (String): The Eth account where tokens will be swapped from
///
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

/// S
///
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SwapTokenInput {
    #[schemars(description = "Uniswap V2 router contract address")]
    pub uniswap_address: String,
    #[schemars(description = "The amount of tokens expected to sell for eth")]
    pub amount_in: String,
    #[schemars(description = "The minimum amount of eth expected to receive")]
    pub min_amount_out: String,
    #[schemars(description = "The input token address or contract being swapped for ETH")]
    pub from_token_addr: String,
    #[schemars(description = "The address where funds will be swapped from")]
    pub account_addr: String,
}

impl UniSwapTools for MultiTool {
    async fn swap_eth_to_token(&self, input: SwapEthInput) -> anyhow::Result<String> {
        let token_addr = Address::from_str(&input.to_token_addr)?;
        let account_addr = Address::from_str(&input.account_addr)?;
        let weth_addr = Address::from_str(super::WETH_TOKEN_ADDRESS)?;
        let contract_addr = Address::from_str(&input.uniswap_address)?;

        let eth_amount_in = parse_ether(&input.amount_in)?;

        // Calculate minimum tokens out with slippage (e.g., 5% slippage = accept 95% of expected)
        // This value might be 0 if the 0x protocol api isn't available.
        let expected_tokens_out = U256::from_str(&input.min_amount_out)?;

        let slippage_percent = 10; // 5% slippage tolerance
        let min_tokens_out = expected_tokens_out * (100 - slippage_percent) / 100;
        tracing::info!("Min TOKEN EXPECTED *******: {min_tokens_out}");

        let path = vec![weth_addr, token_addr];
        let deadline = U256::from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300);

        // Check account balance first
        self.check_balance(account_addr, eth_amount_in).await?;

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

    async fn swap_token_to_eth(&self, input: SwapTokenInput) -> anyhow::Result<String> {
        let from_token_addr = Address::from_str(&input.from_token_addr)?;
        let account_addr = Address::from_str(&input.account_addr)?;
        let weth_addr = Address::from_str(super::WETH_TOKEN_ADDRESS)?;
        let contract_addr = Address::from_str(&input.uniswap_address)?;

        let token_amount_in = parse_ether(&input.amount_in)?;

        // Calculate minimum tokens out with slippage (e.g., 5% slippage = accept 95% of expected)
        // This value might be 0 if the 0x protocol api isn't available.
        let expected_tokens_out = U256::from_str(&input.min_amount_out)?;

        let slippage_percent = 5; // 5% slippage tolerance
        let min_tokens_out = expected_tokens_out * (100 - slippage_percent) / 100;

        let path = vec![from_token_addr, weth_addr];
        let deadline = U256::from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300);

        // Check account balance first
        self.check_balance(account_addr, token_amount_in).await?;

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

    async fn check_balance(&self, account_addr: Address, amount_in: U256) -> anyhow::Result<()> {
        let balance = self.eth_provider.get_balance(account_addr, None).await?;
        let gas_estimate = U256::from(200_000); // Rough estimate
        let gas_price = self.eth_provider.get_gas_price().await?;
        let estimated_gas = gas_estimate * gas_price;

        if balance < amount_in + estimated_gas {
            return Err(anyhow::anyhow!(
                "Insufficient ETH balance. Need {} ETH for swap + {} ETH for gas. Balance: {} ETH",
                ethers::utils::format_ether(amount_in),
                ethers::utils::format_ether(estimated_gas),
                ethers::utils::format_ether(balance)
            ));
        }
        Ok(())
    }
}

// let token_addr = Address::from_str(&input.to_token_addr)?;
// let account_addr = Address::from_str(&input.account_addr)?;
// let weth_addr = Address::from_str(super::WETH_TOKEN_ADDRESS)?;
// let contract_addr = Address::from_str(&input.uniswap_address)?;
//
// let eth_amount_in = parse_ether(&input.amount_in)?;
//
// // Calculate minimum tokens out with slippage (e.g., 5% slippage = accept 95% of expected)
// // This value might be 0 if the 0x protocol api isn't available.
// let expected_tokens_out = U256::from_str(&input.min_amount_out)?;
//
// let slippage_percent = 5; // 5% slippage tolerance
// let min_tokens_out = expected_tokens_out * (100 - slippage_percent) / 100;
//
// let path = vec![weth_addr, token_addr];
// let deadline = U256::from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300);
//
// // Check account balance first
// let balance = self.eth_provider.get_balance(account_addr, None).await?;
// let gas_estimate = U256::from(200_000); // Rough estimate
// let gas_price = self.eth_provider.get_gas_price().await?;
// let estimated_gas_cost = gas_estimate * gas_price;
//
// if balance < eth_amount_in + estimated_gas_cost {
// return Err(anyhow::anyhow!(
// "Insufficient ETH balance. Need {} ETH for swap + {} ETH for gas. Balance: {} ETH",
// ethers::utils::format_ether(eth_amount_in),
// ethers::utils::format_ether(estimated_gas_cost),
// ethers::utils::format_ether(balance)
// ));
// }
//
// let contract = UniswapV2Router::new(contract_addr, self.eth_provider.clone());
//
// // Build transaction with input values
// let tx = contract
// .swap_exact_eth_for_tokens(
// min_tokens_out,  // Minimum tokens to accept (slippage protection)
// path,
// account_addr,
// deadline
// )
// .value(eth_amount_in);  // ETH amount to swap
//
// // Send transaction and wait for confirmation
// let pending_tx = tx.send().await?;
// let receipt = pending_tx.await?.ok_or_else(|| anyhow::anyhow!("Transaction failed"))?;
//
// Ok(format!("Transaction successful! Hash: {:?}, Gas used: {:?}",
//            receipt.transaction_hash, receipt.gas_used))
