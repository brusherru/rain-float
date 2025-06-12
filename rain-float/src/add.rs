use alloy_sol_types::SolCall;
use anyhow::{Result};

use crate::{Float, RainFloat};
use crate::sol::DecimalFloat::addCall;

impl RainFloat {
    pub fn add(&mut self, a: &Float, b: &Float) -> Result<Float> {
        let data = addCall { a: a.0, b: b.0 }.abi_encode();
        let result = self.execute(data.into()).unwrap();
        Float::from(&result[0..32])
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: &str) {
        let mut rain = RainFloat::new().unwrap();
        let a_float = rain.parse(a).unwrap();
        let b_float = rain.parse(b).unwrap();
        let result = rain.add(&a_float, &b_float).unwrap();
        let formatted = rain.format(&result).unwrap();
        assert_eq!(
          formatted, expected,
          "Input: {} + {}, Expected: {}, Actual: {:?}",
          a, b, expected, formatted
        );
    }

    #[test]
    fn simple() {
        check("0", "0", "0");
        check("1", "2", "3");
        check("3", "5", "8");
        check("10", "20", "30");
        check("100", "200", "300");
        check("10000000000000", "10000000000000", "20000000000000");

    }
    
    #[test]
    fn negatives() {
        check("-1", "-2", "-3");
        check("-3", "-5", "-8");
        check("-10", "-20", "-30");
        check("-100", "-200", "-300");
        check("-10000000000000", "-10000000000000", "-20000000000000");

        check("-200", "350", "150");
    }

    #[test]
    fn decimals() {
        check("0.1", "0.2", "0.3");
        check("0.3", "0.5", "0.8");
        check("1.0", "2.0", "3");
        check("10.0", "20.0", "30");
        check("100.000000000001", "200.000000000002", "300.000000000003");

        check("1.5931", "2.4069", "4");
        check("0.00000001", "0.00000002", "0.00000003");
    }

    #[test]
    fn leading_zeroes() {
        check("0000", "0000", "0");
        check("001", "002", "3");
        check("0001", "0002", "3");

        check("00.01", "00.02", "0.03");
        check("000.001", "000.002", "0.003");
    }
}