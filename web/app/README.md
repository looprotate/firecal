# Firecal web interface

The site follows the team's supplied visual reference while retaining Firecal's real analysis. English is the default; Spanish, German, French and simplified Chinese are available. The public screens are `/index.html`, `/explorer.html`, `/approach.html`, `/data.html`, `/about.html` and `/support.html`.

## Development

From the repository root, install the pinned Rust toolchain and Node version, then run:

```
cargo xtask build-wasm
npm --prefix web ci
npm --prefix web run typecheck
npm --prefix web test
npm --prefix web run build
npm --prefix web run dev
```

`web/dist/` contains a static deployment with all six direct entry pages. Source does not include Sites project identity, credentials, dependencies, generated builds, WebAssembly binaries or fire-data tiles.

## Published data

The six-capital demo uses 13 FCT1 tiles and a coverage catalog in `web/public/tiles/`. Restore these from the separately packaged runtime dataset before building a demo with data. The catalog must identify every published tile's row, column, first month and month count. Queries reject incomplete catalog coverage rather than treating unknown neighboring tiles as observations of zero.

The packaged data covers Ottawa, Brasília, Beijing, Moscow and Algiers for January 2012–December 2019, plus Australian regions for November 2000–December 2019. Calibration uses MODIS Terra and Suomi NPP and excludes 2019 from fitting. This historical demonstration is not live wildfire monitoring. Raw satellite archives and generated tiles belong outside source Git; see `docs/references.md` for scientific sources and limits.

## Tools and presentation

- Natural-color map, rectangle/polygon drawing, coordinate entry, monthly selection tint and expanded map dialog.
- Date range selection, hotspot counts or radiative power, native CSV exports, monthly statistics, anomaly markers and analog years.
- Side-by-side area comparison with shared interval, reference years and color scale; totals are not area-normalized.
- Calendar PNG/SVG export and validated query links preserving geometry, metric, interval and selected month.
- Region pinning and local WebAssembly queries offline. The basemap requires a network.
- Five-language interface; fixed navbar, mobile right-side modal drawer, reduced-motion support and the selected transparent tree/flame logo with a skippable entry animation.

The supplied forest photograph appears behind the home title and entry introduction. It comes from the team's archive; the footer is plain. Placeholder charts and statistics from that reference are not imported.

## Integration

This update retains the latest upstream Rust core, its rising-only anomaly behavior, and the newer region-loading API. A pinned-only reader delegates to that API with host reads disabled. The summary does not display the core's floored-denominator anomaly as a percentage of a median below one.

Visual browser, image-download and live offline checks remain unavailable in the current execution environment. Automated checks and any repository publication gate failures must be reported in the pull request.
