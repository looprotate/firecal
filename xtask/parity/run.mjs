// Runs the calls of a cases file through firecal.wasm, by the loader the page uses, and writes the answers.
// Usage: node run.mjs <firecal.wasm> <cases file> <results file>
// Cases file: records of u32 function id, u32 payload length, payload (all little-endian).
//   0 fc_month_index, 1 fc_month_span: payload is two u32. Answer: one u32.
//   3 cells.csv: payload as series. Answer: a u32 status, then, when it is 0, the text.
//   4 area series, 5 comparison, 6 analogs, 7 series.csv: payload as series, then baseline first and last year,
//     season year and months, analog slots, corner count (u32) and the corners as f64 (latitude, longitude).
//     Answers: a u32 status, then when it is 0 the values and sigmas as f64 (4); the medians and anomalies as f64
//     and a byte per month for the flag (5); the found years as u32 and distances as f64, each year then its
//     distance, with no count (6); the text (7).
//   2 series: payload is south, north, west, east (f64), first month, months, plane, tile count (u32), then per
//     tile row, column, length (u32) and the tile bytes. Answer: a u32 status, then, when it is 0, the values
//     and the sigmas as f64.
// Results file: per record a u32 answer length and the answer.
import { readFileSync, writeFileSync } from "node:fs";
import { loadFirecal } from "../../wasm/load.ts";

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
  return { query: { south, north, west, east, firstMonth, months, plane, tiles }, cursor };
}

function analysisOf(payload) {
  const { query, cursor } = queryOf(payload);
  const at = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  const [baselineFirst, baselineLast, seasonYear, seasonMonths, slots, cornerCount] = [0, 4, 8, 12, 16, 20].map(
    (offset) => at.getUint32(cursor + offset, true),
  );
  const polygon = [];
  for (let corner = 0; corner < cornerCount; corner += 1) {
    const base = cursor + 24 + corner * 16;
    polygon.push([at.getFloat64(base, true), at.getFloat64(base + 8, true)]);
  }
  const analysis = { ...query, baselineFirst, baselineLast, seasonYear, seasonMonths };
  if (cornerCount > 0) analysis.polygon = polygon;
  return { analysis, slots };
}

function concat(parts) {
  return Buffer.concat(parts.map((part) => Buffer.from(part.buffer, part.byteOffset, part.byteLength)));
}

function areaSeriesAnswer(payload) {
  const answer = firecal.areaSeries(analysisOf(payload).analysis);
  return concat([word(answer.status), new Float64Array([...answer.value, ...answer.sigma])]);
}

function comparisonAnswer(payload) {
  const answer = firecal.comparison(analysisOf(payload).analysis);
  return concat([word(answer.status), new Float64Array([...answer.median, ...answer.anomaly]), answer.unusual]);
}

function analogsAnswer(payload) {
  const { analysis, slots } = analysisOf(payload);
  const answer = firecal.analogs(analysis, slots);
  const found = answer.analogs.map((analog) => {
    const slot = new Uint8Array(12);
    new DataView(slot.buffer).setUint32(0, analog.year, true);
    new DataView(slot.buffer).setFloat64(4, analog.distance, true);
    return slot;
  });
  return concat([word(answer.status), ...found]);
}

function seriesCsvAnswer(payload) {
  const answer = firecal.seriesCsv(analysisOf(payload).analysis);
  return concat([word(answer.status), answer.text]);
}

function seriesAnswer(payload) {
  const answer = firecal.series(queryOf(payload).query);
  const numbers = new Float64Array([...answer.value, ...answer.sigma]);
  const out = new Uint8Array(4 + numbers.length * 8);
  out.set(word(answer.status));
  const outView = new DataView(out.buffer);
  numbers.forEach((number, index) => outView.setFloat64(4 + index * 8, number, true));
  return out;
}

function cellsCsvAnswer(payload) {
  const answer = firecal.cellsCsv(queryOf(payload).query);
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
  } else if (id === 4) {
    answers.push(areaSeriesAnswer(payload));
  } else if (id === 5) {
    answers.push(comparisonAnswer(payload));
  } else if (id === 6) {
    answers.push(analogsAnswer(payload));
  } else if (id === 7) {
    answers.push(seriesCsvAnswer(payload));
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
