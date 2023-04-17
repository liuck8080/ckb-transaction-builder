use std::{any::Any, collections::HashMap};

use super::{builder::TransactionBuilder, Network, ScriptGroup};

pub mod sighash;
pub mod multihash;

pub trait ScriptHandler: Sync {
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

pub struct HandlerManager {
    envs: HashMap<Network, Vec<Box<dyn ScriptHandler>>>,
}
impl HandlerManager {
    pub fn new() -> Self {
        Self {
            envs: HashMap::new(),
        }
    }
    pub fn init(&mut self) {}
    pub fn get(network: Network) -> Option<&'static Vec<Box<dyn ScriptHandler>>> {
        todo!()
    }
}

