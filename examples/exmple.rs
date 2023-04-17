use ckb_transaction_builder::transaction::{
    builder::{SimpleTransactionBuilder, TransactionBuilder},
    handler::{sighash::Secp256k1Blake160SighashAllScriptContext, HandlerContext},
    Network,
};

fn build_contexts() -> Vec<Box<dyn HandlerContext>> {
    let mut ret = Vec::new();
    let sighash_context =  Box::new(Secp256k1Blake160SighashAllScriptContext {}) as Box<_>;
    ret.push(sighash_context);
    ret
}
pub fn main() {
    let builder = SimpleTransactionBuilder::new(Network::Test);
    let contexts = build_contexts();
    let tx_with_groups = builder.build(&contexts);
}
