# rain-float

`rain-float` is a Rust library and WebAssembly (WASM) wrapper for interacting with a smart contract [Rain.Math.Float](https://github.com/rainlanguage/rain.math.float) that implements floating-point arithmetic or logic. This package enables seamless integration of smart contract logic into Rust and JavaScript/TypeScript environments via WASM.

## Features

- **Rust Library**: Core logic for interacting with the smart contract, written in idiomatic Rust.
- **WASM Wrapper**: Exposes Rust functionality to JavaScript/TypeScript via WASM bindings.
- **Easy Integration**: Designed for use in both backend (Rust) and frontend (WASM) projects.

## Project Structure

```
rain-float/             # Rust source code for the core library
wasm/                   # WASM bindings and interface code
vendor/                 # Directory contains sub-modules to dependencies
├── rainix/             # Submodule that contains flake.nix, to use the same environment and tooling versions as Rain devs
└── rain.math.float/    # Submodule with actual smart-contract code, used to build ABI and bytecode from it
```

## Makefile Commands

- `make abi`: Generates ABI and bytecode from submodule
- `make wasm`: Build the WASM package using `wasm-pack`.
- `make test`: Run all tests: in Rust library and Wasm package
- `make clean`: Clean build artifacts
- `make build`: Cleans everything and builds from scratch

## Usage

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
rain-float-wasm = "0.1.0"
```

Import and use in your Rust code:

```rust
use rain_float_wasm::RainFloat;

let rf = RainFloat::new().unwrap();

let a = rf.parse("1.23").unwrap();
let b = rf.parse("2.34").unwrap();
let c = rf.add(a, b).unwrap();
let r = rf.format(c).unwrap();

assertEq!(r, "3.57");
```

### WASM (JavaScript/TypeScript)

Import in your JS/TS project:

```js
import { WasmRainFloat } from 'rain-float-wasm';

const rf = new WasmRainFloat();
const a = rf.parse('1.23');
const b = rf.parse('3.456');
const c = rf.add(a, b);
const res = rf.format(c);
console.log(c); // "4.686"
```

Check out `examples` directory for details.
To run examples from there you need to run `make build` before.

## Development

- Ensure you have installed `nix`, it is used in all scripts.
- Use the provided `Makefile` for common tasks.
- Contributions and issues are welcome!
