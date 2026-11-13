# Who owns what

Six rappers. Five write code; Eminem owns the story. **Ownership is by directory, and each owner talks to the
rest only through a public API or a file format.** The machine-readable version is [`OWNERS`](../OWNERS); the
people and how a session starts are in [`crew/`](../crew/README.md).

| Alias | Stage name | Owns | Language |
|---|---|---|---|
| `biggie` | The Notorious B.I.G. (Axel) | `firecal-types`, `firecal-query`, `firecal-wasm`, `firecal-testkit`, `xtask`, `wasm/`, `web/stub/`, `web/tokens/` (with Easy-E), and every lead-only file | Rust |
| `easy-e` | Eazy-E (Kevin, team owner) | `web/app/`, `web/map/`, `web/tokens/` (with Biggie), the web build files, offline | TypeScript |
| `dre` | Dr. Dre (Santiago) | `firecal-ingest` | Rust |
| `cube` | Ice Cube (Bayron) | `firecal-harmonize` | Rust |
| `2pac` | 2Pac (Brandon) | `web/calendar/` | TypeScript |
| `eminem` | Eminem (Americo) | `pitch/`: story, demo, video, submission; tests the app on Windows | none |

Biggie is the lead and integrator: reviews every contract change, keeps `main` green, unblocks the rest first.
One person owning three crates is deliberate: they are small and they are the contracts. If that becomes a
bottleneck, `firecal-query` moves to someone else, because it only depends on `firecal-types`. Biggie and Easy-E
decide the look together; the accessible ramp is theirs.

## Dependencies

```
firecal-types ◄── firecal-ingest ──► data/aggregates/*.fca
      ▲                                     │
      ├──────── firecal-harmonize ◄─────────┘ ──► tiles/*.bin
      ▲                                              │
      ├──────── firecal-query ◄──────────────────────┘
      ▲               ▲
      └── firecal-wasm┘ ──► web/*  (through wasm/firecal.d.ts)
```

- Nothing depends on `ingest` or `harmonize` except the files they write. They are pipeline, not library.
- `web/*` never sees Rust. It sees `firecal.d.ts` and the tile files.
- Data flows through **files with a documented format**, so Dre and Cube never wait for each other: each
  works against a hand-made fixture of the other's output.

## Who unblocks whom

1. Biggie's first issues unblock everyone: the foundation is already in (types, wasm spike, xtask, CI), then the
   tile and aggregate readers and writers.
2. Easy-E and 2Pac work against the **stub `Series`** (`web/stub/series.json`) from the first hour.
3. Dre and Cube work against fixtures in their own `tests/fixtures/`.
4. Integration points, in order: stub → real tiles for one region → the wasm call replaces the stub → the
   whole world. Each is a merge that Biggie demos to the team.

## The public API of each crate (proposal)

Signatures are intentions; the real ones live in the code, and the code wins.

- `firecal-types`: `Satellite`, `MonthIndex`, `CellIndex`, `Error`, the `.fca` aggregate (`AggregateRecord`,
  `write_aggregate`, `AggregateView`) and the tile (`Plane`, `TileId`, `write_tile`, `TileView`) exist, as byte-slice
  readers and writers; `Area` and `Series` come next.
- `firecal-ingest`: `parse_record(&[u8]) -> Result<Hotspot, ParseError>`, `Aggregator` (add hotspot, write
  `.fca` bytes). A binary walks `data/` and runs it in parallel.
- `firecal-harmonize`: `fit(&Aggregates, &Aggregates) -> Calibration`, `apply(&Calibration, &Aggregates) -> Tile`.
  A binary writes `tiles/`.
- `firecal-query`: `month_span` exists; `series(&[TileView], &Area, MonthRange, &mut Series)`,
  `analogs(&Series, ...) -> [Analog; N]` and `write_csv(&Series, &mut Vec<u8>)` come next.
- `firecal-wasm`: the above as `extern "C"` over a linear-memory buffer, described in `wasm/firecal.d.ts`.

## Done when

| Owner | Done when |
|---|---|
| `firecal-types` | all crates compile against it; tile and aggregate round-trip tests pass |
| `firecal-ingest` | a real FIRMS CSV gives the same aggregate as the hand-counted fixture; throughput measured |
| `firecal-harmonize` | over the overlap, harmonized VIIRS agrees with MODIS within the tolerance in the tests; sigma is non-trivial and checked |
| `firecal-query` | series, median, anomaly (with the sigma rule) and analogs match hand-computed fixtures; `parity` is green |
| `web/calendar` | the fixture area renders the expected calendar, the three modes work, the CSV downloads |
| `web/map`, `web/app` | draw or type an area, it reaches the calendar; the app works with the network off |
| `pitch/` | a demo that runs without Biggie present, a backup video recorded before Sunday's polish |
