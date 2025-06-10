mod sol;

use anyhow::{anyhow, Result};
use revm::{
    context::{result::{ExecutionResult, Output}, BlockEnv, CfgEnv, Evm, TxEnv},
    database::InMemoryDB,
    handler::{instructions::EthInstructions, EthPrecompiles},
    interpreter::interpreter::EthInterpreter,
    primitives::{Address, Bytes, FixedBytes, TxKind, U256}, 
    state::Bytecode,
    Context,
    ExecuteCommitEvm,
    MainBuilder,
    MainContext, SystemCallEvm
};

use crate::sol::DecimalFloat;

type EvmContext = Context<BlockEnv, TxEnv, CfgEnv, InMemoryDB>;
type EvmInstance = Evm<EvmContext, (), EthInstructions<EthInterpreter, EvmContext>, EthPrecompiles>;

pub struct RainFloat {
    evm: EvmInstance,
    address: Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Float(FixedBytes<32>);

impl RainFloat {
    pub fn new() -> Result<Self> {
        // Decode hex => Vec<u8>
        let bytecode = Bytecode::new_legacy(DecimalFloat::BYTECODE.clone());

        // Prepare DB
        let db = InMemoryDB::default();

        // Prepare deploy Tx
        let tx = TxEnv {
            caller: Address::ZERO,
            kind: TxKind::Create,
            value: U256::ZERO,
            nonce: 0,
            data: bytecode.bytes(),
            gas_limit: 1_000_000_000_000_u64, // Arbitrary large limit
            ..Default::default()
        };

        let mut evm = Context::mainnet().with_db(db.clone()).build_mainnet();

        // Deploy the contract
        let result = evm.transact_commit(tx).unwrap();
        let address = result.created_address().expect("No contract address created");

        Ok(
            RainFloat {
                evm,
                address,
            }
        )
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> Address {
        self.address
    }
    #[allow(dead_code)]
    fn execute(&mut self, data: Bytes) -> Result<Bytes> {
        let result = self.evm.transact_system_call(self.address, data)
            .map_err(|e| anyhow!("EVM execution error: {}", e))?
            .result;

        let out_bytes = match result {
            ExecutionResult::Success {
                output: Output::Call(value),
                ..
            } => value,
            x => return Err(anyhow!("Unexpected execution result: {:?}", x)),
        };

        Ok(out_bytes)
    }
}

mod parse;
mod format;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let rainfloat = RainFloat::new().unwrap();
        println!("Contract address: {}", rainfloat.get_address());
        assert!(!rainfloat.get_address().is_zero());
    }

    #[test]
    fn roundtrip_parse_format() {
        let mut rain = RainFloat::new().unwrap();

        let inputs = [
            "0",
            "1",
            "123.456",
            "1000000000000000000000000",
            "10000000000000000000000000000000000000000.000000000001",
            "0.000000000000000001",
            // TODO: There is a problem with more precise numbers in Solidity contract
            // "0.0000000000000000001",
            "-1",
            "-999999999999999999999999",
            // TODO: There is a problem with negative decimals in Solidity contract
            // "-0.001",
        ];

        for original in inputs {
            let float = rain.parse(original).expect("parse failed");
            let result = rain.format(&float).expect("format failed");
            assert_eq!(
                original,
                result,
                "Roundtrip failed: input = '{}', output = '{}', float.bytes = {:?}",
                original, result, float.0
            );
        }
    }
}