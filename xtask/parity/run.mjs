// Runs the calls in a cases file through firecal.wasm and writes one u32 result per call.
// Usage: node run.mjs <firecal.wasm> <cases file> <results file>
// A case is three little-endian u32: function id, first argument, second argument.
import { readFileSync, writeFileSync } from "node:fs";

const FUNCTIONS = ["fc_month_index", "fc_month_span"];
const CASE_BYTES = 12;

const [wasmPath, casesPath, resultsPath] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {});
const cases = readFileSync(casesPath);
const view = new DataView(cases.buffer, cases.byteOffset, cases.byteLength);
const count = cases.byteLength / CASE_BYTES;
const results = new DataView(new ArrayBuffer(count * 4));

for (let at = 0; at < count; at += 1) {
  const name = FUNCTIONS[view.getUint32(at * CASE_BYTES, true)];
  const first = view.getUint32(at * CASE_BYTES + 4, true);
  const second = view.getUint32(at * CASE_BYTES + 8, true);
  results.setUint32(at * 4, instance.exports[name](first, second) >>> 0, true);
}
writeFileSync(resultsPath, new Uint8Array(results.buffer));
