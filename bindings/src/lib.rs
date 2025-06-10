use wasm_bindgen::prelude::*;
use hex;
use revm::{
    context::{Evm, TransactTo, TransactionType, TxEnv}, database::{CacheDB, EmptyDB, InMemoryDB}, handler::{instructions::EthInstructions, EthPrecompiles}, interpreter::InterpreterTypes, primitives::{Bytes, TxKind, B256}, Context
};

/// WebAssembly wrapper
#[wasm_bindgen]
pub struct FloatVM {
    evm: Context<InMemoryDB>,
    contract: B256,
}

#[wasm_bindgen]
impl FloatVM {
    pub fn create_evm_instance() -> Evm<InMemoryDB, (), EthInstructions<>, EthPrecompiles> {
        let db = CacheDB::new(EmptyDB::default());
        let mut evm = Evm::new(db, EthInstructions::new_mainnet(), EthPrecompiles::default());
        evm
    }

    #[wasm_bindgen(constructor)]
    pub fn new(bytecode_hex: &str) -> Result<FloatVM, JsValue> {
        // Decode hex => Vec<u8>
        let bytecode = hex::decode(bytecode_hex).map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Выполняем deploy
        let mut ctx = Context::<InMemoryDB>::default()
            .with_db(revm::database::InMemoryDB::default());
        let mut evm = ctx.build();

        let tx = TxEnv {
            tx_type: TransactionType::Legacy,
            kind: TxKind::Create,
            caller: Default::default(),
            gas_limit: u64::MAX,
            gas_price: Default::default(),
            gas_priority_fee: None,
            transact_to: TransactTo::Create(Bytes::from(bytecode)),
            value: Default::default(),
            data: Bytes::new(),
            nonce: 0,
            chain_id: None,
            access_list: vec![],
            blob_hashes: vec![],
            max_fee_per_blob_gas: 0,
            authorization_list: vec![],
        };
        evm.env.tx = tx;

        let out = evm.transact().map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
        let address = out.created_address.ok_or_else(|| JsValue::from_str("No created address"))?;

        Ok(FloatVM { evm, contract: address })
    }

    #[wasm_bindgen]
    pub fn call_add(&mut self, a: i128, b: i128) -> Result<String, JsValue> {
        let mut calldata = vec![0x77, 0x16, 0x02, 0xf7];
        calldata.extend_from_slice(&a.to_be_bytes());
        calldata.extend_from_slice(&b.to_be_bytes());

        self.evm.env.tx = TxEnv {
            transact_to: TransactTo::Call(self.contract),
            data: Bytes::from(calldata),
            ..self.evm.env.tx.clone()
        };

        let out = self.evm.transact_ref().map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
        let data = out.result.into_data().unwrap_or_default();
        Ok(hex::encode(data))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let bytecode = std::fs::read_to_string("../abi/bytecode.hex").unwrap();
        let mut vm = FloatVM::new(&bytecode).unwrap();
        let result = vm.call_add(1, 2);
        println!("add(1,2) = {}", result);
        assert!(!result.is_empty());
    }
}