# Paths
RAINIX_DIR = ./vendor/rainix
FLOAT_LIB = ./vendor/rain.math.float

# Targets
.PHONY: all abi test clean

# Default
all: abi

# Generate ABI and bytecode
abi:
	nix develop $(RAINIX_DIR) --command bash -c '\
	  forge build --root $(FLOAT_LIB) \
	'

# Run tests
test:
	cd rain-float && cargo test -- --nocapture

# Clean up generated files and node_modules
clean:
	cd rain-float && cargo clean