# Plan

The weekend as a sequence an agent can execute. Times are relative; adjust to the real event hours.

## Day 0 (tonight): unblock everything

The critical path is **getting the data**, not code. Start it first.

1. Someone creates a NASA Earthdata login. The archive is requested at the FIRMS **Archive Download** page: pick
   the sensor, the dates and the area, choose CSV and **standard** processing, and a link arrives by email. A
   `MAP_KEY` is only for the FIRMS API (recent days), which we do not need for the archive; if one is ever used,
   it goes in 1Password and `~/.zshrc.local` as `FIRMS_MAP_KEY`, never in the repo.
2. Request MODIS (Terra and Aqua, C6.1) and VIIRS (S-NPP, NOAA-20, NOAA-21) into `data/`, which is in `.gitignore`.
   The requests take time to be prepared, so **place them tonight**. Start with **one region** (say, South
   America) so the full pipeline can run early; the world comes after. Check disk space first.
3. **Done:** the Cargo workspace, the toolchain pin, the lints, `firecal-testkit`, `xtask` (`check`, `guard`,
   `worktree`, `pr-lint`, `build-wasm`), `OWNERS`, CI on macOS, Windows and Linux, the issues and the crew
   files. **Open:** the tile and aggregate formats in `firecal-types`, `xtask parity`.
   Nobody codes against a contract that does not exist yet.
4. Everyone reads `AGENTS.md` and installs their agent (Claude Code or Codex) pointing at the repo.
5. Everyone (macOS and Windows): install Rust with `rustup`, then `rustup target add wasm32-unknown-unknown`.
   Node is needed for the web app and for the parity test.

## Modules and first tasks

| Owner | Module | First task, done when |
|---|---|---|
| lead | `firecal-types`, `-wasm`, `xtask` | types compile everywhere; a one-function wasm call works in Node on macOS and Windows; `check` and `parity` run |
| `ingest` | `firecal-ingest` | one real FIRMS CSV gives the hand-counted fixture's aggregate; throughput measured |
| `harmonize` | `firecal-harmonize` | quantile-map VIIRS to MODIS per tile with sigma; the overlap test passes; a before/after figure exists |
| lead | `firecal-query` | `Series` with anomaly and the `unusual` rule from hand-made tiles; then analogs |
| `calendar` | `web/calendar` | calendar from the stub `Series`; count/energy modes; sigma mark; CSV; analogs view |
| `map` | `web/map` | draw a rectangle or type lat/lon; emits `Area`; service worker and pinned region |
| pitch | `pitch/`, tokens | tokens and ramp chosen, story, demo script, backup video |

`calendar` and `map` start **against stubs** (a hard-coded `Series` from the fixture) so they never wait for the crates. When `query` and `wasm`
land, the stub is replaced by the wasm call and nothing else changes. This is why contracts come first.

## Rhythm

- **Fri night:** data downloading, workspace and headers pushed, stubs running, ramp and look chosen.
- **Sat morning:** ingest + aggregate for the first region; calendar showing real numbers by lunch.
- **Sat afternoon:** harmonization; wasm in the browser; first full end-to-end demo by dinner.
- **Sat night:** the world, or the regions that tell the best story; polish; decide the CSV question.
- **Sun morning:** freeze features at a fixed hour (agree it now). Only bug fixes, the pitch, the video.
- **Sun:** record a backup demo video **before** the final polish, in case the live demo fails.

## Pitch material to keep as we go

Each of these falls out of the work; save them as they appear, in `pitch/`:

- before/after of harmonization: MODIS and VIIRS monthly totals diverging, then agreeing;
- the calendar of one place everyone knows (a recent big fire season) with its anomaly;
- the number that surprises: how fast the browser answers over 20 years, and how fast the pipeline ate the
  archive (measure it, do not guess it);
- the one-line story: the sensor stops mattering.

## Risks, and what we do

- **Data download is slow or rate-limited.** Start now; ship with the region that finished.
- **Harmonization is subtle.** Show the method and its check honestly; a clear, validated simple method beats
  a clever one nobody can explain.
- **wasm toolchain friction.** Spike it tonight with a one-function crate on both macOS and Windows; if it
  fights us for more than an hour, ask for help instead of working around it.
- **Nobody reads Rust yet.** The tests are the safety net; the compiler and clippy catch the memory bugs C
  would let through; the core stays small.
- **Live demo fails.** Static site, local copy of `tiles/`, backup video.

## Open questions for the team

1. The look and the ramp (owner: design). Constraint from the manifesto: accessible to color-blind viewers.
2. The area of the pitch story: which place and which season.
3. The CSV: aggregate only (default) or add recent raw hotspots.
4. The real name.
