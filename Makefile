# Paths
RAINIX_DIR = ./vendor/rainix
FLOAT_LIB = ./vendor/rain.math.float
BUILD_JSON = $(FLOAT_LIB)/out/DecimalFloat.sol/DecimalFloat.json
ABI_OUTPUT_DIR = ./abi

# Targets
.PHONY: all abi wasm test clean

# Default
all: abi

# Generate ABI and bytecode
abi:
	nix develop $(RAINIX_DIR) --command bash -c '\
	  forge build --root $(FLOAT_LIB); \
		mkdir -p $(ABI_OUTPUT_DIR); \
	  jq ".abi" $(BUILD_JSON) > $(ABI_OUTPUT_DIR)/abi.json; \
	  jq -r ".bytecode.object" $(BUILD_JSON) > $(ABI_OUTPUT_DIR)/bytecode.hex \
	'

# Generate WASM
wasm:
	cd bindings && wasm-pack build --target bundler

# Run tests
test:
	cd bindings && wasm-pack test --node

# Clean up generated files and node_modules
clean:
	rm -rf $(ABI_DIR)/abi.json $(ABI_DIR)/bytecode.hex
	cd bindings && cargo clean