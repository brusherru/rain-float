use alloy_sol_types::{ SolCall };
use anyhow::{anyhow, Result};

use crate::sol::DecimalFloat::formatCall;
use crate::{Float, RainFloat};

impl RainFloat {
    pub fn format(&mut self, input: &Float) -> Result<String> {
        let data = formatCall { a: input.0.into()  }.abi_encode();
        let result = self.execute(data.into()).unwrap();
        let decoded = formatCall::abi_decode_returns(&result)
            .map_err(|e| anyhow!("Failed to decode output: {}", e))?;

        Ok(decoded)
    }
}


#[cfg(test)]
mod tests {
    use revm::primitives::hex::FromHex;

    use super::*;
    
    fn check(input: &str, expected: &str) {
        let mut rain = RainFloat::new().unwrap();
        let val: [u8; 32] = FromHex::from_hex(input).unwrap();
        let float = Float(val.into());
        let actual = rain.format(&float).unwrap();
        assert_eq!(actual.as_str(), expected, "Input: {}, Expected: {}, Actual: {:?}", input, expected, actual);
    }

    #[test]
    fn simple() {
        check( "0000000000000000000000000000000000000000000000000000000000000001", "1");
        check( "0000000000000000000000000000000000000000000000000000000000000002", "2");
        check("000000000000000000000000000000000000000000000000000000000000000a", "10");
        check( "0000000000000000000000000000000000000000000000000000000000000064", "100");
    }

    #[test]
    fn decimals() {
        check( "0000000000000000000000000000000000000000000000000000000000000001", "1");
        check( "ffffffff00000000000000000000000000000000000000000000000000000001", "0.1");
        check( "fffffffe00000000000000000000000000000000000000000000000000000001", "0.01");
        check( "fffffffd00000000000000000000000000000000000000000000000000000001", "0.001");
        check( "fffffffc00000000000000000000000000000000000000000000000000000002", "0.0002");
        check( "fffffffc00000000000000000000000000000000000000000000000000003e3b", "1.5931");
        check( "fffffff800000000000000000000000000000000000000000000000000000001", "0.00000001");
    }

    #[test]
    fn negatives() {
        check( "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "-1");
        check( "00000000fffffffffffffffffffffffffffffffffffffffffffffffffffffffe", "-2");
        check( "00000000fffffffffffffffffffffffffffffffffffffffffffffffffffffff6", "-10");
        check( "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffff9c", "-100");

        check( "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "-0.1");
        check( "fffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffff", "-0.01");
        check( "fffffffefffffffffffffffffffffffffffffffffffffffffffffffffffffffe", "-0.02");
    }

    #[test]
    fn large_numbers() {
        // TODO:
    }
}