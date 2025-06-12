# Paths
FLOAT_LIB = ./vendor/rain.math.float

# Targets
.PHONY: all abi wasm test clean build

# Default
all: abi

# Generate ABI and bytecode
abi:
	nix develop --command bash -c '\
		forge build --root $(FLOAT_LIB) \
	'

# Build the WASM module
wasm:
	nix develop --command bash -c '\
	  cd ./wasm; \
		wasm-pack build --target bundler --release --no-opt --no-pack --out-dir ../js/lib; \
		wasm-pack build --target nodejs --release --no-opt --no-pack --out-dir ../js/node \
	'

# Run tests
test:
	nix develop --command bash -c '\
		cd rain-float && cargo test -- --nocapture; \
		cd ../wasm && wasm-pack test --node \
	'

# Clean up generated files
clean:
	nix develop --command bash -c '\
		cd rain-float && cargo clean; \
		cd ../wasm && cargo clean; \
		rm -rf ./js/lib && rm -rf ./js/node \
	'

# Builds everything from scratch
build:
	$(MAKE) clean
	$(MAKE) abi
	$(MAKE) test
	$(MAKE) wasm
