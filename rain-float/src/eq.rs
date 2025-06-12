use alloy_sol_types::SolCall;
use anyhow::{anyhow, Result};

use crate::{Float, RainFloat};
use crate::sol::DecimalFloat::eqCall;

impl RainFloat {
    pub fn eq(&mut self, a: &Float, b: &Float) -> Result<bool> {
        let data = eqCall { a: a.0, b: b.0 }.abi_encode();
        let result = self.execute(data.into()).unwrap();
        eqCall::abi_decode_returns(&result[0..32])
            .map_err(|e| anyhow!("Failed to decode output: {}", e))
    }

}

#[cfg(test)]
mod tests {
    use revm::primitives::{hex::FromHex, FixedBytes};

    use super::*;

    fn check(a: &str, b: &str, expected: bool) {
        let mut rain = RainFloat::new().unwrap();
        let a_float = rain.parse(a).unwrap();
        let b_float = rain.parse(b).unwrap();
        let result = rain.eq(&a_float, &b_float).unwrap();
        assert_eq!(
            result, expected,
            "Input: {} == {}, Expected: {}, Actual: {:?}",
            a, b, expected, result
        );
    }

    #[test]
    fn is_equal() {
        check("0", "0", true);
        check("23", "23", true);
        check("0.001", "0.001", true);
        check("1000.001", "1000.001", true);
        check("-341.00201", "-341.00201", true);
    }

    #[test]
    fn is_not_equal() {
        check("0", "4", false);
        check("-1", "1", false);
        check("100.001", "0.001", false);
    }

    #[test]
    fn check_eq_of_different_representation_of_threes() {
        let mut rain = RainFloat::new().unwrap();
        let a_float = rain.parse("3").unwrap();
        let b_float = Float(
          FixedBytes::from_hex("ffffffdb0000000000000000000000001691ca32818ed48b02dca3e000000000").unwrap()
        );
        let result = rain.eq(&a_float, &b_float).unwrap();
        assert!(result, "Expected {} to be equal to {}", a_float.0, b_float.0);
    }
}