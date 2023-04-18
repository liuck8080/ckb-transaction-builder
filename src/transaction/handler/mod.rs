use std::any::Any;

use super::{builder::TransactionBuilder, Network, ScriptGroup};

pub mod multihash;
pub mod sighash;

pub trait ScriptHandler {
    fn build_transaction(
        &self,
        tx_builder: &dyn TransactionBuilder,
        script_group: &ScriptGroup,
        context: &dyn HandlerContext,
    ) -> Result<bool, String>;

    fn init(&mut self, network: Network);
}

pub trait Type2Any: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> Type2Any for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait HandlerContext: Type2Any {}

pub struct TransactionBuilderConfiguration {
    network: Network,
    script_handlers: Vec<Box<dyn ScriptHandler>>,
    fee_rate: u64,
}

impl TransactionBuilderConfiguration {
    pub fn new() -> Self {
        Self::new_with_network(Network::Mainnet)
    }
    pub fn new_testnet() -> Self {
        Self::new_with_network(Network::Testnet)
    }

    fn new_with_network(network: Network) -> Self {
        Self {
            network,
            script_handlers: Self::generate_system_handlers(network),
            fee_rate: 1000,
        }
    }
    pub fn generate_system_handlers(network: Network) -> Vec<Box<dyn ScriptHandler>> {
        vec![
            Box::new(sighash::Secp256k1Blake160SighashAllScriptHandler::new_with_network(network))
                as Box<_>,
        ]
    }
    pub fn register_script_handler(&mut self, script_handler: Box<dyn ScriptHandler>) {
        self.script_handlers.push(script_handler);
    }
    pub fn get_script_handlers(&self) -> &Vec<Box<dyn ScriptHandler>> {
        &self.script_handlers
    }
}
