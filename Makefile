# Paths
RAINIX_DIR = ./vendor/rainix
FLOAT_LIB = ./vendor/rain.math.float

# Targets
.PHONY: all abi wasm test clean build

# Default
all: abi

# Generate ABI and bytecode
abi:
	nix develop $(RAINIX_DIR) --command bash -c '\
		forge build --root $(FLOAT_LIB) \
	'

# Build the WASM module
wasm:
	nix develop $(RAINIX_DIR) --command bash -c '\
	  cd ./wasm; \
		wasm-pack build --target bundler --out-dir ../js \
	'

# Run tests
test:
	nix develop $(RAINIX_DIR) --command bash -c '\
		cd rain-float && cargo test -- --nocapture; \
		cd ../wasm && wasm-pack test --node \
	'

# Clean up generated files
clean:
	nix develop $(RAINIX_DIR) --command bash -c '\
		cd rain-float && cargo clean; \
		cd ../wasm && cargo clean; \
		rm -rf ./js \
	'

# Builds everything from scratch
build:
	$(MAKE) clean
	$(MAKE) abi
	$(MAKE) test
	$(MAKE) wasm
