use wasm_bindgen::prelude::*;
use rain_float::{RainFloat, Float};

#[wasm_bindgen]
pub struct WasmRainFloat(RainFloat);

#[wasm_bindgen]
impl WasmRainFloat {
    
    // Creates a new instance of `WasmRainFloat`
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmRainFloat, JsValue> {
        Ok(WasmRainFloat(RainFloat::new().map_err(|e| JsValue::from_str(&e.to_string()))?))
    }

    // Parses string into a `Float``
    #[wasm_bindgen]
    pub fn parse(&mut self, s: &str) -> Result<WasmFloat, JsValue> {
        let float = self.0.parse(s).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmFloat(float))
    }

    // Formats a `Float` into a string
    #[wasm_bindgen]
    pub fn format(&mut self, float: &WasmFloat) -> Result<String, JsValue> {
        self.0.format(&float.0).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    // Sums two `Float` instances and returns a new one
    #[wasm_bindgen]
    pub fn add(&mut self, a: &WasmFloat, b: &WasmFloat) -> Result<WasmFloat, JsValue> {
        let result = self.0.add(&a.0, &b.0).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmFloat(result))
    }

    // Checks equality of two `Float` instances and returns a boolean
    #[wasm_bindgen]
    pub fn eq(&mut self, a: &WasmFloat, b: &WasmFloat) -> Result<bool, JsValue> {
        self.0.eq(&a.0, &b.0).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    //
    // Utils
    //

    // Returns contract address, might be useful to verify the contract version
    #[wasm_bindgen(getter = address)]
    pub fn address(&self) -> String {
        self.0.get_address().to_string()
    }

    // Returns the version of the library
    // This is useful for debugging and ensuring compatibility.
    // The version is set at compile time using the `CARGO_PKG_VERSION` environment variable.
    #[wasm_bindgen(getter = version)]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[wasm_bindgen]
pub struct WasmFloat(Float);

// Single test to check that it is working well
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_wasm_rain_float() {

        // Create a new instance of WasmRainFloat
        let mut rain_float = WasmRainFloat::new().unwrap();

        let parsed_float = rain_float.parse("3.14").unwrap();
        let formatted = rain_float.format(&parsed_float).unwrap();
        assert_eq!(formatted, "3.14");

        // Test adding two WasmFloat instances
        let another_float = rain_float.parse("2.86").unwrap();
        let sum = rain_float.add(&parsed_float, &another_float).unwrap();
        let formatted_sum = rain_float.format(&sum).unwrap();
        assert_eq!(formatted_sum, "6");

        // Test equality of two WasmFloat instances
        let six_float = rain_float.parse("006.00").unwrap();
        assert!(
            !rain_float.eq(&parsed_float, &another_float).unwrap()
        );
        assert!(
            rain_float.eq(&sum, &six_float).unwrap()
        );

        // Check the address and version
        assert!(!rain_float.address().is_empty());
        assert!(!rain_float.version().is_empty());
    }
}