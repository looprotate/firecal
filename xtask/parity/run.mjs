// Runs the calls of a cases file through firecal.wasm, by the loader the page uses, and writes the answers.
// Usage: node run.mjs <firecal.wasm> <cases file> <results file>
// Cases file: records of u32 function id, u32 payload length, payload (all little-endian).
//   0 fc_month_index, 1 fc_month_span: payload is two u32. Answer: one u32.
//   3 cells.csv: payload as series. Answer: a u32 status, then, when it is 0, the text.
//   2 series: payload is south, north, west, east (f64), first month, months, plane, tile count (u32), then per
//     tile row, column, length (u32) and the tile bytes. Answer: a u32 status, then, when it is 0, the values
//     and the sigmas as f64.
// Results file: per record a u32 answer length and the answer.
import { readFileSync, writeFileSync } from "node:fs";
import { loadFirecal } from "../../wasm/firecal.ts";

const [wasmPath, casesPath, resultsPath] = process.argv.slice(2);
const firecal = await loadFirecal(readFileSync(wasmPath));
const cases = readFileSync(casesPath);
const view = new DataView(cases.buffer, cases.byteOffset, cases.byteLength);
const answers = [];

function word(value) {
  const bytes = new Uint8Array(4);
  new DataView(bytes.buffer).setUint32(0, value, true);
  return bytes;
}

function queryOf(payload) {
  const at = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  const [south, north, west, east] = [0, 8, 16, 24].map((offset) => at.getFloat64(offset, true));
  const [firstMonth, months, plane, tileCount] = [32, 36, 40, 44].map((offset) => at.getUint32(offset, true));
  const tiles = [];
  let cursor = 48;
  for (let tile = 0; tile < tileCount; tile += 1) {
    const length = at.getUint32(cursor + 8, true);
    tiles.push({
      row: at.getUint32(cursor, true),
      col: at.getUint32(cursor + 4, true),
      bytes: payload.subarray(cursor + 12, cursor + 12 + length),
    });
    cursor += 12 + length;
  }
  return { south, north, west, east, firstMonth, months, plane, tiles };
}

function seriesAnswer(payload) {
  const answer = firecal.series(queryOf(payload));
  const numbers = new Float64Array([...answer.value, ...answer.sigma]);
  const out = new Uint8Array(4 + numbers.length * 8);
  out.set(word(answer.status));
  const outView = new DataView(out.buffer);
  numbers.forEach((number, index) => outView.setFloat64(4 + index * 8, number, true));
  return out;
}

function cellsCsvAnswer(payload) {
  const answer = firecal.cellsCsv(queryOf(payload));
  const out = new Uint8Array(4 + answer.text.length);
  out.set(word(answer.status));
  out.set(answer.text, 4);
  return out;
}

for (let cursor = 0; cursor < cases.byteLength; ) {
  const id = view.getUint32(cursor, true);
  const length = view.getUint32(cursor + 4, true);
  const payload = cases.subarray(cursor + 8, cursor + 8 + length);
  if (id === 2) {
    answers.push(seriesAnswer(payload));
  } else if (id === 3) {
    answers.push(cellsCsvAnswer(payload));
  } else {
    const first = view.getUint32(cursor + 8, true);
    const second = view.getUint32(cursor + 12, true);
    const call = id === 0 ? firecal.exports.fc_month_index : firecal.exports.fc_month_span;
    answers.push(word(call(first, second)));
  }
  cursor += 8 + length;
}

const parts = answers.flatMap((answer) => [word(answer.length), answer]);
writeFileSync(resultsPath, Buffer.concat(parts));
