use ckb_transaction_builder::transaction::{handler::{
    sighash::Secp256k1Blake160SighashAllScriptContext, HandlerContext,
    TransactionBuilderConfiguration,
}, builder::{SimpleTransactionBuilder, TransactionBuilder}};

fn build_contexts() -> Vec<Box<dyn HandlerContext>> {
    let mut ret = Vec::new();
    let sighash_context = Box::new(Secp256k1Blake160SighashAllScriptContext {}) as Box<_>;
    ret.push(sighash_context);
    ret
}

pub fn main() {
    let configuration = TransactionBuilderConfiguration::new_testnet();
    let builder = SimpleTransactionBuilder::new(configuration);
    let contexts = build_contexts();
    let tx_with_groups = builder.build(&contexts);
}
