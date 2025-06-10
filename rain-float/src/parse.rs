use alloy_sol_types::SolCall;
use anyhow::{anyhow, Result};

use crate::{Float, RainFloat};
use crate::sol::DecimalFloat::parseCall;

impl RainFloat {
    pub fn parse(&mut self, input: &str) -> Result<Float> {
        let data = parseCall { str: input.to_string() }.abi_encode();
        let result = self.execute(data.into()).unwrap();

        // Ensure that the output contains (byte4, bytes32), which are represented as 64 bytes in total
        if result.len() != 64 {
            return Err(anyhow!("Unexpected output length: {} ({})", result.len(), result));
        }
        // Ensure that there is no error code (first 32 bytes should be zero)
        if result[0..32] != [0u8; 32] {
            return Err(anyhow!("Error code present in output"));
        }
        // Extract the output bytes, which should be the second 32 bytes
        let mut value = [0u8; 32];
        value.copy_from_slice(&result[32..64]);
        // Return the new Float instance
        Ok(Float(value.into()))
    }
}

#[cfg(test)]
mod tests {
    use revm::primitives::hex::FromHex;

    use super::*;

    fn check(input: &str, expected: &str) {
        let mut rain = RainFloat::new().unwrap();
        let val = rain.parse(input).unwrap();
        let exp: [u8; 32] = FromHex::from_hex(expected).unwrap();
        assert_eq!(val.0, exp, "Input: {}, Expected: {}, Actual: {:?}", input, expected, val.0);
    }

    #[test]
    fn simple() {
        check("1", "0000000000000000000000000000000000000000000000000000000000000001");
        check("2", "0000000000000000000000000000000000000000000000000000000000000002");
        check("10", "000000000000000000000000000000000000000000000000000000000000000a");
        check("100", "0000000000000000000000000000000000000000000000000000000000000064");
    }
    
    #[test]
    fn negatives() {
        check("-1", "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        check("-2", "00000000fffffffffffffffffffffffffffffffffffffffffffffffffffffffe");
        check("-10", "00000000fffffffffffffffffffffffffffffffffffffffffffffffffffffff6");
        check("-100", "00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffff9c");

        // TODO:
        // check("-0.01", "fffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn decimals() {
        check("1.0", "0000000000000000000000000000000000000000000000000000000000000001");
        check("0.1", "ffffffff00000000000000000000000000000000000000000000000000000001");
        check("0.01", "fffffffe00000000000000000000000000000000000000000000000000000001");
        check("0.001", "fffffffd00000000000000000000000000000000000000000000000000000001");
        check("0.0002", "fffffffc00000000000000000000000000000000000000000000000000000002");
        check("1.5931", "fffffffc00000000000000000000000000000000000000000000000000003e3b");
    }

    #[test]
    fn leading_zeroes() {
        check("0000", "0000000000000000000000000000000000000000000000000000000000000000");
        check("001", "0000000000000000000000000000000000000000000000000000000000000001");
        check("0001", "0000000000000000000000000000000000000000000000000000000000000001");

        check("00.01", "fffffffe00000000000000000000000000000000000000000000000000000001");
        check("000.001", "fffffffd00000000000000000000000000000000000000000000000000000001");

    }

    #[test]
    fn exponents() {
        check("1E0", "0000000000000000000000000000000000000000000000000000000000000001");
        check("1e-8", "fffffff800000000000000000000000000000000000000000000000000000001");
        // TODO: Should large numbers work well or Reverting is expected?
        // check("1e57896044618658097711785492504343953926634992332820282019728792003956564819967", "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    }
}