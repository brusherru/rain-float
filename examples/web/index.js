import { WasmRainFloat } from 'rain-float-wasm';

const rf = new WasmRainFloat();
const a = rf.parse('1.23');
const b = rf.parse('3.456');
const c = rf.add(a, b);
const res = rf.format(c);

console.log(res); // "4.686"

document.body.innerHTML = `
  <h1>WasmRainFloat Example</h1>
  <p>
    1.23 + 3.456 = <strong>${res}</strong>
  </p>
`;