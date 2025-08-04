
/// Interface to evm related tools used by Agent.
pub trait EvmTools {
    fn get_balance() -> String;
    fn send() -> String;
    fn get_contract() -> String;
}