# firecal

NASA Space Apps 2026 (Uruguay). A burning-activity calendar that harmonizes MODIS and VIIRS hotspots, tells you
how sure it is, works with no signal, and remembers which past year this one looks like.
`docs/idea.md` says what it is and why it is different.

## For the judges

![The calendar of south-east Australia, November 2000 to December 2019, with November and December 2019 flagged](docs/calendar-south-east-australia.png)

Two generations of NASA fire satellites, MODIS since 2000 and VIIRS since 2012, count burning in ways that cannot be
compared. firecal maps VIIRS onto MODIS per 10° tile, keeps the error of that mapping as an uncertainty on every
number, and draws a calendar for any area you pick: months across, years down. A month is flagged only when it
clears that uncertainty and the range of past years. It shows hotspot counts or fire radiative power, the past years
most like this one, and a CSV, and it keeps answering with no network in a region pinned before.

- **Data:** NASA FIRMS active-fire hotspots, MODIS Collection 6.1 (Terra, Aqua) and VIIRS 375 m (Suomi NPP,
  NOAA-20, NOAA-21). Nothing else enters the science. Measured results are in [`pitch/numbers.md`](pitch/numbers.md).
- **Run it:** `cargo xtask build-wasm`, then `npm --prefix web ci` and `npm --prefix web run dev`, and open
  http://localhost:5173. Without tiles the calendar shows a hand-made sample and says so; tiles written by
  `firecal-harmonize` go in `web/public/tiles/`.
- **How it is built:** one Rust core, compiled for the pipeline and to WebAssembly for the browser; a test proves both
  give identical bytes. The page draws; every number comes from the core. No server.
- **What we do not claim:** a hotspot is a thermal anomaly seen from orbit, not a fire or a burned area; harmonized
  counts are MODIS-equivalent and fitted on the years both sensors saw; analog years are history, not a forecast;
  the newest months are provisional, from near-real-time data; the uncertainty is an estimate.
- **How AI was used:** the team chose the idea, the design and the rules (`AGENTS.md`); AI coding agents (Claude Code
  and Codex) wrote and tested the code and docs under those rules, and every change passed the same checks on macOS,
  Windows and Linux before it was merged.

## Start (everyone)

1. Install [Rust via rustup](https://rustup.rs), [Node](https://nodejs.org) (the version in `.nvmrc`) and the
   [GitHub CLI](https://cli.github.com); run `gh auth login`.
   **On Windows** also install [Git for Windows](https://gitforwindows.org) (its `sh` runs the pre-push hook) and
   the Visual Studio Build Tools with the "Desktop development with C++" workload, which Rust needs to link;
   then `git config --global core.longpaths true`. Use PowerShell or Git Bash; every command below is the same.
2. `git clone` this repo and open it with Claude Code or Codex.
3. Check the machine: `cargo xtask check` (the pinned toolchain installs itself the first time), then
   `cargo xtask hooks` so a push always runs the verification first.
4. Tell the agent who you are: **"I'm Biggie"**, **"I'm Easy-E"**, **"I'm Dre"**, **"I'm Cube"**,
   **"I'm 2Pac"** or **"I'm Eminem"**. It reads your file in `crew/`, takes your first open issue and starts.

Licensed under Apache-2.0 (`LICENSE`). The repository is public, as the challenge requires at submission: every
commit, issue and pull request is world-readable, so no keys, no third-party data and no personal addresses.

`AGENTS.md` is the law for every agent. The weekend plan is `docs/plan.md`.

## Commands

```
cargo xtask check [--full]     format, lints, tests, wasm build and its Node test; --full adds release
cargo xtask guard [base]       fails when a branch edits files its owner does not own
cargo xtask worktree <a>/<t>   a fresh worktree and branch for one agent
cargo xtask ready              the gate before a push: clean, rebased, diff rules, check --full, guard
cargo xtask hooks              installs the pre-push hook that runs ready
cargo xtask pr-lint            checks a PR title and description (CI sets PR_TITLE and PR_BODY)
cargo xtask build-wasm         builds web/public/firecal.wasm
cargo xtask parity             native and wasm (under Node) give identical bytes for the same calls
npm --prefix web run smoke     the built app in headless Chrome, online and with the host stopped
```
