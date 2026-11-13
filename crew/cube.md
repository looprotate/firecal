# Ice Cube (Bayron, @bayron_rodriguez)

Say **"I'm Cube"** (or "soy cube") to the agent and this is the file it follows.

**Role:** Codes harmonization: VIIRS to MODIS-equivalent, with uncertainty.
**Branch prefix:** `cube/`
**Labels:** `owner:cube`

## You own

- `crates/firecal-harmonize/`

You own the science. The sensor stops mattering because of your work, and every number carries a sigma because of it.

## Start

1. Read `AGENTS.md`, `docs/engineering.md`, `docs/idea.md` and this file.
2. `git pull`, then `cargo xtask check` to be sure your machine is green, and `cargo xtask hooks` once.
3. Find your work: `gh issue list --label owner:cube --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number that is not blocked.
4. Make your worktree: `cargo xtask worktree cube/<short-task-name>`, and work only there.
5. Follow `docs/protocol.md`: test first, break it on purpose, run the real thing, then `cargo xtask ready`.
   Never push without it, and never use `--no-verify`.
6. Open a pull request from the template, with `closes #<issue>`, then take the next issue.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/cube-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
