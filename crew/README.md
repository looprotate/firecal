# The crew

Each person has a file here. A session starts when someone says who they are.

| Say | Stage name | Name | Space Apps handle | Does |
|---|---|---|---|---|
| `biggie` | The Notorious B.I.G. | Axel | @axel_ibarrondo | codes |
| `easy-e` | Eazy-E | Kevin | @kevin_arregin | codes |
| `dre` | Dr. Dre | Santiago | @santiago_rodriguez | codes |
| `cube` | Ice Cube | Bayron | @bayron_rodriguez | codes |
| `2pac` | 2Pac | Brandon | @brandon_bignoli | codes |
| `eminem` | Eminem | Americo | @americomenendez | pitch |

The agent accepts the stage name, the alias in the first column, the first name or the handle, in English or
Spanish ("soy Biggie", "I'm 2Pac", "sou el Dre"), and opens `crew/<alias>.md`.

## Set up a machine

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
cargo xtask tokens [--report]  writes the identity files (colors, logo, title card), or how each color was derived
npm --prefix web run smoke     the built app in headless Chrome, online and with the host stopped
```
