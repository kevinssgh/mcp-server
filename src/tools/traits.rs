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
