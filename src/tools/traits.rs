use crate::tools::uniswap_tools::{SwapEthInput, SwapTokenInput};
use crate::tools::zero_x_tools::QuoteInput;
use anyhow::Result;
use ethers::addressbook::Address;
use ethers::prelude::U256;

/// Interface to evm related tools used by Agent.
pub(crate) trait EvmTools {
    async fn get_balance(&self, address: String) -> Result<String>;
    async fn send(&self, from: String, to: String, amount: String) -> Result<String>;
    async fn get_contract(&self, contract: String) -> Result<String>;
    async fn get_erc20_balance(&self, contract: String, account: String) -> Result<String>;
}

/// Interface to brave related tools used by Agent.
pub(crate) trait BraveTools {
    async fn search(&self, query: String) -> Result<String>;
}

/// Interface to 0x api for swap quotes.
pub(crate) trait ZeroXTools {
    async fn get_quote(&self, input: QuoteInput) -> Result<String>;
}

/// Interface to Uniswap contract abi.
pub(crate) trait UniSwapTools {
    async fn swap_eth_to_token(&self, swap_input: SwapEthInput) -> Result<String>;
    async fn swap_token_to_eth(&self, swap_input: SwapTokenInput) -> Result<String>;
    async fn check_balance(&self, account_addr: Address, amount_in: U256) -> Result<()>;
}
