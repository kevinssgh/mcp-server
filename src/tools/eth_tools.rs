use crate::tools::MultiTool;
use crate::tools::traits::EvmTools;
use anyhow::{Result, anyhow};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use rmcp::schemars;
use std::str::FromStr;

// Generate ERC20 contract bindings - standard erc20 contract methods
abigen!(
    ERC20,
    r#"[
        function name() public view returns (string)
        function symbol() public view returns (string)
        function decimals() public view returns (uint8)
        function totalSupply() public view returns (uint256)
        function balanceOf(address _owner) public view returns (uint256)
        function transfer(address _to, uint256 _value) public returns (bool)
        function transferFrom(address _from, address _to, uint256 _value) public returns (bool)
        function approve(address _spender, uint256 _value) public returns (bool)
        function allowance(address _owner, address _spender) public view returns (uint256)
        event Transfer(address indexed _from, address indexed _to, uint256 _value)
        event Approval(address indexed _owner, address indexed _spender, uint256 _value)
    ]"#,
);

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BalanceInput {
    #[schemars(description = "The address or ENS name to check the balance for")]
    pub addr: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendInput {
    #[schemars(description = "The address or ENS name used to send ETH from")]
    pub sender: String,
    #[schemars(description = "The address or ENS name to send ETH to")]
    pub receiver: String,
    #[schemars(description = "The amount of ETH to send")]
    pub amount: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetContractInput {
    #[schemars(description = "The address of the contract to look for")]
    pub addr: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ERC20BalanceInput {
    #[schemars(description = "The address of the ERC20 contract to look for")]
    pub erc20_addr: String,
    #[schemars(description = "The address of the account to get the balance for")]
    pub account: String,
}

impl EvmTools for MultiTool {
    async fn get_balance(&self, address: String) -> Result<String> {
        let addr = Address::from_str(&address)?;
        let balance = self
            .eth_provider
            .get_balance(addr, None)
            .await
            .map_err(|e| {
                // Add tracing
                anyhow!("failed to get balance from {}: {}", address, e.to_string())
            })?;
        Ok(balance.to_string())
    }

    async fn send(&self, from: String, to: String, amount: String) -> Result<String> {
        let sender = Address::from_str(&from)?;
        let receiver = Address::from_str(&to)?;
        let amount = parse_ether(&amount)?;

        //Attempt to get specified sender wallet. If not provided or found, use default wallet.
        let wallet = match self.accounts.get_wallet(&sender) {
            None => {
                if let Some(acc) = self.accounts.default_wallet() {
                    Ok(acc)
                } else {
                    Err(anyhow!("sender not found, failed to get default wallet"))
                }
            }
            Some(acc) => Ok(acc),
        }?;

        // Initialize client
        let client = SignerMiddleware::new(&self.eth_provider, wallet.clone());
        let tx = TransactionRequest::new()
            .to(NameOrAddress::Address(receiver))
            .value(amount);

        // Send transaction
        let pending_tx = client
            .send_transaction(tx, None)
            .await
            .map_err(|e| anyhow!("send transaction failed {e}"))?;
        let receipt = pending_tx
            .await
            .map_err(|e| anyhow!("send transaction failed {e}"))?;

        // Check receipt and return tx hash if it's ok
        let tx_hash = match receipt {
            None => Err(anyhow!("receipt was empty")),
            Some(r) => Ok(r.transaction_hash),
        }?;

        Ok(format!("transaction hash: {tx_hash:x}"))
    }

    async fn get_contract(&self, contract: String) -> Result<String> {
        let contract_addr = Address::from_str(&contract)?;
        let code = self.eth_provider.get_code(contract_addr, None).await?;
        if !code.is_empty() {
            Ok(format!(
                "Contract {contract} is deployed (code size: {})",
                code.len()
            ))
        } else {
            Ok(format!("Contract {contract} is not deployed"))
        }
    }

    async fn get_erc20_balance(&self, contract: String, account: String) -> Result<String> {
        // Convert strings to addresses
        let token_addr = Address::from_str(&contract)?;
        let account_addr = Address::from_str(&account)?;

        // Get contract (cloning the atomic reference counter)
        let contract = ERC20::new(token_addr, self.eth_provider.clone());

        // get balance
        let balance = contract.balance_of(account_addr).call().await?;
        Ok(format!("balance is: {balance} in wei"))
    }
}
