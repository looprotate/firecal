# Engineering rules for agents

For every agent, Claude Code or Codex, and every human working beside one. `AGENTS.md` is the law,
`docs/style.md` is the style; this file is how we make code that is **portable**, **correct** and **never in
someone else's way**. Where it is stricter than the others, this one wins.

The three goals are one discipline. Code that runs the same on macOS, Windows and wasm is code with no hidden
inputs; code with no hidden inputs is easy to test; code that is easy to test can be changed by someone else's
agent without fear. Sloppiness in any one shows up as a collision in another.

---

## Part 1. Portable code

The target set is **macOS (aarch64), Windows (x86_64), Linux (CI) and `wasm32-unknown-unknown`**. A change
that works on one and not the others is broken.

### The compiler proves it

- `firecal-types` and `firecal-query` are **`#![no_std]`**. They cannot touch files, clocks, threads or the OS,
  because the compiler refuses. This is the strongest portability guarantee we have; do not add `std` back.
  If you need to allocate, ask the lead; the default is caller-owned slices.
- `firecal-ingest` and `firecal-harmonize` use `std`, but split into a **pure library** (no I/O) and a thin
  binary (all I/O). Only the binary opens a file.
- CI builds `types` and `query` for `wasm32-unknown-unknown` on every PR. A core change that does not build for
  wasm does not merge.

### Numbers

- **Fixed-width integers in anything stored or shared**: `u8 u16 u32 u64`. Never `usize` or `isize` in a
  format, a public API or a struct that crosses the wasm boundary. `usize` is 32 bits in wasm and 64 on the
  desktop; the same code must not change meaning between them.
- **Arithmetic cannot silently wrap.** Use `checked_`, `saturating_` or `wrapping_` and say which on purpose.
  Release builds set `overflow-checks = true` in the workspace profile, so a wrong assumption crashes loudly
  in a test instead of corrupting a count.
- **Casts that can lose information are forbidden** (`as` between sizes or signs). Use `u16::try_from(x)` or a
  named helper that saturates. `clippy::cast_possible_truncation` and its relatives are denied.
- **Bytes are explicit.** Read and write with `from_le_bytes` and `to_le_bytes`. Never `transmute`, never cast a
  pointer to read a struct, never rely on struct layout. A format is what `docs/data-format.md` says, byte by
  byte.
- **Floating point must be reproducible.** `f64`, and only `+ - * /` and `sqrt`; no `exp`, `ln`, `powf`, trig
  (native and wasm math libraries differ in the last bit). No `f32` accumulation. Sums run in a fixed order;
  never `.sum()` over something unordered, never a parallel reduction in the core. Compare floats in tests with
  an explicit tolerance written next to the reason for it, except where bit-equality is the point (parity).
- **Ordering is deterministic.** No `HashMap` or `HashSet` in anything whose output matters: use `BTreeMap` or a
  sorted `Vec`. No dependence on thread count, locale or timezone.

### The machine

- **Paths:** `std::path::Path` and `join`, never `"a/b"` concatenation or hard-coded separators. Never assume a
  drive letter, a home directory or that a path is valid UTF-8 when it comes from the user. Windows is
  case-insensitive and macOS usually is: two files must never differ only by case.
- **File names** use `a-z 0-9 _ -` and `.`, lowercase, short. No spaces, no `:` or `*`, nothing Windows
  reserves (`con`, `nul`, `aux`). Keep paths short: Windows has length limits.
- **Line endings:** LF everywhere (`.gitattributes`). Code that reads text, FIRMS CSV in particular, must
  accept `\n`, `\r\n` and a leading UTF-8 byte-order mark, and a test proves it.
- **Temp files** in tests use a per-test directory under `std::env::temp_dir()` that the test removes. Never
  `/tmp`, never the working directory.
- **No `build.rs`**, no native dependencies, no shelling out. If a task needs a program, it is a
  `cargo xtask` command written in Rust.
- **No environment reads in libraries.** A binary may read `FIRMS_MAP_KEY`; a library receives it as an
  argument.
- **One toolchain**: `rust-toolchain.toml`. `Cargo.lock` is committed. Node is pinned in `.nvmrc`; the web
  lockfile is committed.

### The browser

- No `std::time`, `std::fs`, `std::thread` or `println!` in the wasm crate. Time and files are the page's job.
- The wasm boundary is **plain `extern "C"` over a linear-memory buffer**: `u32` offsets and lengths in,
  a `u32` status out. Every exported function is described in `wasm/firecal.d.ts` and has a test through Node.
- Never trust the caller's pointer or length: validate before reading, and return an error code instead of
  panicking. A panic in wasm is an unreachable trap that kills the page.
- TypeScript targets evergreen browsers and no Node-only API in `web/`. `localStorage` is not the store for
  tiles; the service worker cache is.

---

## Part 2. Code of extremely high quality

"High quality" here means **checkable**, so a person who cannot read Rust can still verify it.

### Lints are the first reviewer

The workspace `Cargo.toml` declares one `[workspace.lints]` table and every crate inherits it. It sets:

```
clippy::pedantic              warn, and CI treats warnings as errors
clippy::unwrap_used           deny
clippy::expect_used           deny
clippy::panic                 deny
clippy::indexing_slicing      deny      use .get() or iterators; a bad index is a bug, not a crash
clippy::cast_possible_truncation, cast_sign_loss, cast_possible_wrap   deny
clippy::float_cmp             deny
clippy::todo, unimplemented, dbg_macro, print_stdout (libraries)      deny
unsafe_code                   forbid    except firecal-wasm, where each unsafe block states why it is sound
missing_docs                  warn on public items
```

Tests may `unwrap`. Nobody weakens a lint to get a green build; to allow one case, put `#[expect(lint,
reason = "...")]` on that single item with the reason. An `expect` with no reason is rejected in review.

### Design

- **Make wrong states unrepresentable.** A month is `MonthIndex`, not a `u16`; a satellite is an enum, not a
  string; an area that is empty cannot be built. Parse at the edge into strong types; the inside trusts them.
- **Small, total functions.** A function takes what it needs, returns a `Result` or a value for every input, and
  does one thing. If its name needs "and", it is two functions.
- **No premature cleverness.** Plain loops over iterator tricks when they read better; no macros to save lines;
  no generics with one use. The next agent has to understand it in one read.
- **Performance is measured.** The hot path is `ingest` parsing hundreds of millions of rows. Optimize it only
  with a benchmark that shows the number before and after, and keep the simple version in a test as the oracle
  the fast one must match.
- **Errors are values.** Each `enum Error` says what went wrong in terms the caller can act on. No stringly
  errors in libraries. No `Result<_, ()>`.
- **No dead code, no commented-out code, no TODO without an issue number.** A speculative feature is not added
  "for later".

### Docs that earn their keep

- `//!` on every module and `///` on every public item: what it is, its units, what it does at the edges
  (empty, saturated, out of range). Examples in docs are doctests, so they run.
- The **why** of a non-obvious choice goes in the commit message or `docs/`, never as a comment that restates
  the line.

### Review a person can do without reading the code

Every PR description has these headings, and a PR without them is not reviewed:

```
What:      one sentence
Contract:  untouched | changed (link the doc change)
Ran:       the exact commands and their results (cargo xtask check, parity, the app)
Not run:   anything that could not be run, and why
Evidence:  a number, a screenshot, or a diff of outputs, whichever proves it works
```

---

## Part 3. Tests

The tests are the review. A feature is not written until you know how it will be tested.

### What exists, from cheap to expensive

1. **Unit tests** in `<module>/tests.rs`: one behavior each, named `<thing>_<situation>_<expected>`
   (`month_index_from_november_2000_is_ten`). Expected values are written by hand and checkable by a person.
2. **Golden fixtures** in `crates/*/tests/fixtures/`: tiny, hand-counted inputs (ten hotspots, one tile, three
   years) with the expected output **written by a human, never generated by the code under test**. Real FIRMS
   data is never a fixture.
3. **Round trips** for every format: `read(write(x)) == x` for the aggregate and the tile, including empty,
   one-record and maximum-size cases.
4. **Property tests** without a dependency: a tiny deterministic generator (a fixed-seed xorshift in the
   `firecal-testkit` dev crate, owned by the lead) drives thousands of cases. State the invariant in a
   sentence: the series sum equals the sum of its cells; an area that contains another has at least its total;
   anomaly is zero when value equals the median; rank distance of a year to itself is zero. The seed is printed
   on failure so it can be replayed.
5. **Parity**: `cargo xtask parity` runs the same inputs through the native and the wasm builds and compares the
   output bytes. It is a blocker for any change to `types`, `query` or `wasm`.
6. **Harmonization check**: over the overlap years, harmonized VIIRS agrees with MODIS monthly totals per tile
   within a tolerance written in the test with its reason; sigma is checked to cover the real MODIS value at the
   stated rate.
7. **Web smoke**: load the app against fixture tiles, apply a fixed area, assert the calendar cell values;
   repeat with the network off. `npm --prefix web run smoke`, in headless Chrome (`CHROME_PATH` if it is not in
   the usual place); it reads the debug wasm build, so run `cargo xtask check` first.

### Edge cases that must have a test

Hand each of these to the agent as a checklist; most real bugs live here.

- Empty input, a single record, the largest input.
- Counts at 65535 and above: saturation, not wraparound, and the sigma saturating with it.
- An area crossing the **antimeridian** (±180°), touching a **pole**, thinner than one cell, covering one cell
  exactly, covering the world.
- Polygon vertices on a cell edge; clockwise and counter-clockwise winding; a self-intersecting polygon rejected.
- Month ranges: start equals end, start after end, before the first or after the last available month, a gap
  with no tile.
- A tile that does not exist (ocean, no hotspots) is zeros, not an error.
- CSV: CRLF, BOM, missing column, extra column, empty line, a bad number, an out-of-range latitude.
- Differences between MODIS and VIIRS records in the same loader: different column names and confidence
  encodings.
- Windows paths and a file name with a space or non-ASCII character in the CLI.

### Rules of a good test

- **Deterministic and isolated.** No clock, network, global state, or dependence on test order; it passes in
  parallel and alone. Randomness is seeded.
- **It fails when the code is wrong.** Before trusting a new test, break the code on purpose and watch it fail.
  A test that cannot fail is noise. For anything in the hot path of correctness (calibration, area sum,
  anomaly), the PR says which deliberate break was tried.
- **One reason to fail.** If a test fails you should know what broke from its name.
- **Tests do not mirror the implementation.** They assert behavior and invariants, so a refactor does not break
  them.
- **A bug is a failing test first**, then the fix, in the same PR.

### Coverage is a question, not a score

No percentage target. Instead, every public function has at least one test and every `match` arm and error
variant is reached by one. `cargo xtask check` reports public items with no test.

---

## Part 4. Not stepping on each other

Six people and several agents will edit one repository for two days. Collisions are the biggest schedule risk
after the data download, so the rules are mechanical.

### Ownership is a file, and the machine enforces it

- `OWNERS` at the root maps each owner (the branch prefix) to the directories and files they may edit.
- **No human review is required to merge.** CI is the reviewer: it is stricter, faster and never tired, and
  waiting for a person is the slowest thing in a two-day build. That is why the tests are the review, and why
  `guard` and `pr-lint` are mechanical. A person may still comment, and anyone may ask for a second look on a
  contract change, but nothing blocks on it.
- `cargo xtask guard` fails if a branch's diff touches a file outside its owner's directories. CI runs it on
  every PR. An agent that "just needs to fix one thing" in another crate cannot.

**Lead-only files** (no one else edits them, ever, except through the lead): the workspace `Cargo.toml` and
`Cargo.lock`, `rust-toolchain.toml`, `.github/`, `OWNERS`, `AGENTS.md`, everything in `docs/`, the
contract crate `firecal-types`, `wasm/firecal.d.ts`. These are the files two people would otherwise both
change.

### Need something from another crate?

Do not edit it. Write a short request in `requests/<from>-to-<owner>-<topic>.md` stating the function or type
you need, the input and output, and the test that would prove it. The owner answers by implementing it or by
saying why not. Until then, work against a stub on your side.

### One agent, one worktree

Two agents in the same checkout overwrite each other and fight over `target/`. So:

- **Each agent works in its own `git worktree`**, on its own branch, in its own directory, with its own
  `target/`. `cargo xtask worktree <owner>/<task>` creates it.
- **One agent, one crate**, per task. A person running two agents gives them different crates, never the same
  one.
- Do not run two builds of the same worktree at once.

### Branches and merging

- Branch name: `<owner>/<task>`, from the latest `main`. The task is small enough to merge the same day.
- **Rebase** on `main` before opening the PR and again if `main` moved; never merge `main` into the branch.
  Pull requests are **squash-merged**: one pull request is one commit on `main`, whose title is the PR title
  and whose body is the PR description. Linear history, and every commit is revertable on its own.
- `main` is protected: a PR needs green CI: `check --full` (which includes the wasm build and the web build),
  `parity` and the web smoke in Chrome, on macOS, Windows and Linux, plus `guard` and `pr-lint`. There is no required approval: when CI is green the author merges their
  own pull request, squashed.
- **Merge small and early.** A branch older than two hours is a risk; split it. A long-lived branch is the main
  way collisions happen.
- If a rebase conflicts in a file you do not own, **stop**: you touched something you should not have. Undo it
  and use a request.

### Contracts are frozen except on purpose

- A contract is `firecal-types`, the tile format, the aggregate format and `firecal.d.ts`.
- A change to one is **its own PR** by the lead, with the docs, the code and every consumer updated together
  and the round-trip tests extended. No agent changes a contract on its own initiative; if it looks wrong,
  stop and say so.
- Consumers pin to behavior through tests: a contract change that breaks a consumer's test is visible at once.

### The protocol every agent follows

This is the short form. The long one, in order and with the verification steps, is `docs/protocol.md`; the gate
before a push is `cargo xtask ready`.


```
1. Read AGENTS.md, docs/engineering.md, docs/style.md and the contract you implement.
2. git pull --rebase main; create your worktree and branch.
3. Confirm the one directory you may edit. If the task needs another, write a request and stop.
4. Write the test first, watch it fail, make it pass.
5. cargo xtask check, then parity if you touched types, query or wasm. Fix, do not silence.
6. Run the real thing and look at the result: the binary, the page, the output file.
7. Open the PR with the five headings. State what you did not run.
8. If anything surprised you (a contract that looks wrong, a lint you want to relax, a test you cannot make
   fail), stop and ask a human rather than working around it.
```

### Stop conditions

An agent stops and asks when: a contract looks wrong; the task needs a file it does not own; a dependency is
needed; a lint seems to be in the way; a test cannot be made deterministic; the task cannot be verified by
running it; it has been stuck for twenty minutes on the same error.

---

## What the lead creates first

These make the rules above true instead of hoped for. They are on the Day 0 list in `docs/plan.md`.

1. Workspace `Cargo.toml` with `[workspace.lints]` and the release profile with `overflow-checks`.
2. `rust-toolchain.toml`, `.nvmrc`, `.gitattributes` (exists), committed lockfiles.
3. `firecal-types` and `firecal-testkit`, with the round-trip tests.
4. `cargo xtask check | guard | worktree | build-wasm` (done) and `parity` (open issue).
5. `OWNERS`, `.github/workflows/ci.yml` on macOS, Windows and Linux, branch protection on `main`, the PR
   template with the five headings. Nobody needs `CODEOWNERS`: ownership is enforced by `guard`, not by reviewers.
