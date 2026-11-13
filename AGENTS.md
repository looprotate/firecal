# firecal — manifesto

NASA Space Apps 2026 challenge: **a harmonized burning activity calendar from MODIS + VIIRS active-fire
hotspots (NASA FIRMS)**, in a web app. Everyone on the team, human or agent, builds the same software by
following this file. When this file and your instinct disagree, this file wins; to change it, open a PR
that touches only this file and tell the team.

`firecal` is a working name.

## Starting a session: "I'm Biggie"

This team is six rappers. When the person says who they are, you know what to do; do not ask for a task.

1. Match what they said to a crew file: stage name, alias, first name or Space Apps handle, in English or
   Spanish ("soy Biggie", "I'm 2Pac", "sou el Dre"). The table is [`crew/README.md`](crew/README.md); the file is
   `crew/<alias>.md`. If it is ambiguous, ask once.
2. Read that file and follow its **Start** steps: pull, check the machine is green, list the open issues with
   the label `owner:<alias>`, take the lowest milestone and the lowest unblocked issue, make a worktree with
   `cargo xtask worktree <alias>/<task>`, and begin.
3. Say in two sentences who they are and what you are about to do, then do it. You never wait to be told the task:
   the issues are the task list.
4. Work by the protocol in `docs/engineering.md`. When the issue is done, open the pull request and take the next
   one.

Eminem does not code: his work is in `pitch/`, and his crew file says how he works.

## Read next

- [`docs/idea.md`](docs/idea.md): what the app is and what makes it different. Read before building anything.
- [`crew/README.md`](crew/README.md): the six people and their files. [`docs/team.md`](docs/team.md): who owns
  which crate or package, and what each depends on. The machine-readable version is [`OWNERS`](OWNERS).
- [`docs/engineering.md`](docs/engineering.md): **portable, high-quality, tested code and not stepping on each
  other.** Every agent reads it before its first change; it is stricter than the rest.
- [`docs/style.md`](docs/style.md): how code is written, tested and committed; how to brief an agent.
- [`docs/data-format.md`](docs/data-format.md): the files and types the pieces share.
- [`docs/plan.md`](docs/plan.md): the weekend, in order.

## What we win with

Judges reward the **idea, the visuals and the pitch**, not the amount of code.

1. One thing works end to end before anything is polished. A demo that never breaks beats ten features.
2. The calendar is the hero. Everything else supports it.
3. Four things set us apart, in this order of importance: **uncertainty on every number**, **energy and
   effort, not just counts**, **analog years**, and an **offline field tool** (`docs/idea.md`). A layer that is
   half-done is removed from the demo, never shown broken.
4. The pitch line is real: **the same Rust function that harmonized 20 years of satellites runs in your
   browser, with no signal, on the area you just drew.**

## Product (what the user can do)

1. Pick an area: draw on a map, or type lat/lon bounds.
2. Pick a date range.
3. See the **burning calendar**: months across, years down, each cell a harmonized value on an accessible
   sequential color ramp; mode is hotspot **count** or fire radiative **energy**.
4. See **how now compares with history**: the current month against the historical median, flagged only when
   it clears the uncertainty band.
5. See **analog years**: the past seasons that look most like this one.
6. Download a **CSV** of what the search covers.
7. Use all of it **offline** in a region pinned earlier.

Out of scope for the weekend: accounts, alerts, forecasting, mobile app, raw per-hotspot history.

## Architecture

```
FIRMS archive (MODIS, VIIRS)          never in git
        │  firecal-ingest             native Rust, parallel, runs on one laptop
        ▼
data/aggregates/*.fca                 cell-month counts and energy, per satellite
        │  firecal-harmonize          VIIRS -> MODIS-equivalent counts, with sigma
        ▼
tiles/*.bin (static files)            the contract, docs/data-format.md
        │  served as static files
        ▼
browser: firecal-wasm + TypeScript    the same Rust code answers the area query
```

- **No backend.** Static site, so the demo cannot fall over and works offline.
- **One core in Rust**, built two ways: native (the pipeline) and `wasm32-unknown-unknown` (the browser). Same
  source, same results; a test proves it byte for byte. Cargo works the same on macOS and Windows.
- **The UI is thin TypeScript.** It draws, it never computes statistics: every number on screen comes from the
  core.

## Layout and owners

One owner per directory; the full table is in `docs/team.md`. Touch another's directory only through its
public API or file format.

```
crates/firecal-types      shared vocabulary and file formats
crates/firecal-ingest     FIRMS -> aggregates
crates/firecal-harmonize  aggregates -> tiles, with sigma
crates/firecal-query      series, anomaly, analogs, CSV
crates/firecal-wasm       the browser build
crates/firecal-testkit    deterministic helpers for tests
xtask/                    check, guard, worktree, build-wasm
wasm/firecal.d.ts         the browser contract
web/app web/map web/calendar web/tokens web/stub
pitch/  docs/  crew/  requests/
```

`OWNERS` says who may edit what, and `cargo xtask guard` enforces it on every pull request.

Contracts are the only shared surface. **A contract changes in its own PR, with the code, the docs and every
consumer updated together.** Never in passing.

## Rules (the short version; the long one is `docs/style.md`)

- **English** for code, comments, docs, commits.
- **Rust**, one version in `rust-toolchain.toml`, a Cargo workspace. **No shell scripts**: `cargo xtask`.
- **Names say what a thing does.** No comment restates code; the why goes in `docs/` or the commit.
- **One responsibility per file**, about 200 lines. `//!` opens each module; `///` on what is public. Tests in
  `<module>/tests.rs`.
- **No dependencies without asking.** The core crates have none. Web: `typescript`, `vite`, `maplibre-gl` and
  one drawing plugin; charts by hand.
- **The core touches nothing in the world.** Slices in, slices out; no files, clock, network or threads.
  Binaries at the edge do the I/O.
- **Deterministic, so wasm equals native:** no hash-order output, no `exp`/`ln`/`powf`/trig in the core.
- **Errors someone decides on are an `enum`**; errors that are only shown are a `String`, at the edge.
- **Dead code is deleted** in the same change that kills it, from code, config and docs.
- **Third-party data never goes to git**, never into a prompt, never deleted without a copy. `data/` is in
  `.gitignore`. Keys come from the environment (`FIRMS_MAP_KEY`), never from the repo, a URL or a log.

## Tests: they are the review

The team cannot read Rust line by line, so **nothing is merged on trust**.

1. Every function that computes has a test; expected values are hand-checkable.
2. `cargo xtask check` (fmt, clippy pedantic, tests) is green before a PR. If it was not run, the PR says so.
3. Fixtures are tiny and hand-counted. Real FIRMS data is never a fixture.
4. **Native equals wasm:** `cargo xtask parity` runs the same queries both ways; outputs identical byte for
   byte. A blocker.
5. Harmonization has a property test over the overlap years, with its tolerance written and justified.
6. A bug gets a failing test first, then the fix. A change that must not change output is proven by comparing
   old and new on the same input.
7. Web: one smoke test against fixture tiles, and the app checked with the network off.

## Working with agents (Claude Code and Codex)

- Both read this file. `CLAUDE.md` only points here. Do not fork instructions per tool.
- **One agent, one directory, one branch** named `<owner>/<task>`. Two agents never edit the same files.
- Brief every task with the five parts in `docs/style.md`: crate, contract, task, done-when, do-not-touch.
- Agents verify their own work: build it, run the tests, run the app, look at the result. "It compiles" is
  not done. If something could not be run, the PR says so.
- Small PRs, merged often, **squashed**: one pull request is one commit on `main`. Rebase when older than two hours.
- Agents never change a contract they were not asked to. If a contract looks wrong, stop and tell a human.
- **The pull request title and description are the commit.** Title: imperative, capitalized, at most 72
  characters, no period. Description: the template, with **Why** saying the reason, not the change. No tool
  attribution lines. `cargo xtask pr-lint` enforces it in CI. One idea per pull request.
- Every external resource (data, code, text, images) is cited in `docs/references.md` when it is first used.

## Definition of done

The code is in; its tests pass (`cargo xtask check`, and `parity` if the core changed); the contract docs
match; and a human saw it work (screenshot or run), or the PR says why not.

## Not decided here

Names, look and feel, the ramp and the pitch narrative belong to the people who own them (`docs/team.md`). The
manifesto fixes rules, not taste.
