# Rain-float-wasm

A WASM wrapper for the Rain.Math.Float contract, enabling high-performance float math in JavaScript/TypeScript via WebAssembly.

## Installation

```bash
npm install rain-float-wasm
```


or
```bash
yarn add rain-float-wasm
```

## Usage

```js
import { WasmRainFloat } from 'rain-float-wasm';

const rf = new WasmRainFloat();
const a = rf.parse('1.23');
const b = rf.parse('3.456');
const c = rf.add(a, b);
const res = rf.format(c);
console.log(c); // "4.686"
```
