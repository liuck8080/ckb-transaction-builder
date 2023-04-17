use super::{
    handler::{HandlerContext, ScriptHandler, HandlerManager},
    Network, TransactionWithScriptGroups,
};

pub trait TransactionBuilder {
    fn build(&self, contexts: &[Box<dyn HandlerContext>]) -> TransactionWithScriptGroups;
}

pub struct SimpleTransactionBuilder {
    network: Network,
}

impl SimpleTransactionBuilder {
    pub fn new(network: Network) -> Self {
        Self { network }
    }
}

impl TransactionBuilder for SimpleTransactionBuilder {
    fn build(&self, contexts: &[Box<dyn HandlerContext>]) -> TransactionWithScriptGroups {
        let script_group = super::ScriptGroup {};
        if let Some(handlers) = HandlerManager::get(self.network) {
            for handler in handlers.iter() {
                for context in contexts {
                    if let Ok(true) = handler.build_transaction(self, &script_group, context.as_ref()) {
                        break;
                    }
                }
            }
        }

        TransactionWithScriptGroups {}
    }
}
