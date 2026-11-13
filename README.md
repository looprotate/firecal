# firecal

NASA Space Apps 2026 (Uruguay). A burning-activity calendar that harmonizes MODIS and VIIRS hotspots, tells you
how sure it is, works with no signal, and remembers which past year this one looks like.
`docs/idea.md` says what it is and why it is different.

## Start (everyone)

1. Install [Rust via rustup](https://rustup.rs), [Node](https://nodejs.org) and the
   [GitHub CLI](https://cli.github.com); run `gh auth login`.
2. `git clone` this repo and open it with Claude Code or Codex.
3. Check the machine: `cargo xtask check` (the pinned toolchain installs itself the first time).
4. Tell the agent who you are: **"I'm Biggie"**, **"I'm Easy-E"**, **"I'm Dre"**, **"I'm Cube"**,
   **"I'm 2Pac"** or **"I'm Eminem"**. It reads your file in `crew/`, takes your first open issue and starts.

Licensed under Apache-2.0 (`LICENSE`). The repo is private until we submit; the challenge requires the code to
be public and open at submission.

`AGENTS.md` is the law for every agent. The weekend plan is `docs/plan.md`.

## Commands

```
cargo xtask check [--full]     format, lints, tests, wasm build; --full adds the release profile
cargo xtask guard [base]       fails when a branch edits files its owner does not own
cargo xtask worktree <a>/<t>   a fresh worktree and branch for one agent
cargo xtask pr-lint            checks a PR title and description (CI sets PR_TITLE and PR_BODY)
cargo xtask build-wasm         builds web/public/firecal.wasm
```
