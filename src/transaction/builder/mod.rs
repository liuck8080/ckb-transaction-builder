pub(crate) use super::{
    handler::{HandlerContext, TransactionBuilderConfiguration},
    TransactionWithScriptGroups,
};

pub trait TransactionBuilder {
    fn build(&self, contexts: &[Box<dyn HandlerContext>]) -> TransactionWithScriptGroups;
}

pub struct SimpleTransactionBuilder {
    configurataion: TransactionBuilderConfiguration,
}

impl SimpleTransactionBuilder {
    pub fn new(configurataion: TransactionBuilderConfiguration) -> Self {
        Self { configurataion }
    }
}

impl TransactionBuilder for SimpleTransactionBuilder {
    fn build(&self, contexts: &[Box<dyn HandlerContext>]) -> TransactionWithScriptGroups {
        let script_group = super::ScriptGroup {};
        for handler in self.configurataion.get_script_handlers() {
            for context in contexts {
                if let Ok(true) = handler.build_transaction(self, &script_group, context.as_ref()) {
                    break;
                }
            }
        }

        TransactionWithScriptGroups {}
    }
}
