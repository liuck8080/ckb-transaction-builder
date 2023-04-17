use crate::transaction::{builder::TransactionBuilder, ScriptGroup};

use super::{HandlerContext, ScriptHandler};

pub struct Secp256k1Blake160SighashAllScriptHandler {}

pub struct Secp256k1Blake160SighashAllScriptContext {}

impl HandlerContext for Secp256k1Blake160SighashAllScriptContext {}

impl Secp256k1Blake160SighashAllScriptHandler {
    pub fn is_match(&self, script_group: &ScriptGroup) -> bool {
        true
    }
}

impl super::ScriptHandler for Secp256k1Blake160SighashAllScriptHandler {
    fn build_transaction(
        &self,
        tx_builder: &dyn TransactionBuilder,
        script_group: &crate::transaction::ScriptGroup,
        context: &dyn HandlerContext,
    ) -> Result<bool, String> {
        if self.is_match(script_group) {
            return Ok(false);
        }
        if let Some(args) = context
            .as_any()
            .downcast_ref::<Secp256k1Blake160SighashAllScriptHandler>()
        {
        } else {
            return Ok(false);
        }
        todo!()
    }

    fn init(&mut self, network: crate::transaction::Network) {
        // init code hash and cell deps
    }
}

