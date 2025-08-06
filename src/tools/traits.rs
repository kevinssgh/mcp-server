use anyhow::Result;

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
    async fn get_quote(
        &self,
        from_token: String,
        to_token: String,
        amount: String,
    ) -> Result<String>;
}

/// Interface to Uniswap contract abi.
pub(crate) trait UniSwapTools {
    async fn swap_eth_to_token(&self, to_token: String, amount: String) -> Result<String>;
    async fn swap_token_to_token(
        &self,
        from_token: String,
        to_token: String,
        amount: String,
    ) -> Result<String>;
}
