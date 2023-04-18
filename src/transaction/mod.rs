pub mod builder;
pub mod handler;

pub struct ScriptGroup {}
pub struct TransactionWithScriptGroups {}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Network {
    Mainnet,
    Testnet,
}
