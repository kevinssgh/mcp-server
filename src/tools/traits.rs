use anyhow::Result;


/// Interface to evm related tools used by Agent.
pub(crate) trait EvmTools {
    async fn get_balance(&self, address: String) -> Result<String>;
    async fn send(&self ) -> String;
    async fn get_contract(&self ) -> String;
}