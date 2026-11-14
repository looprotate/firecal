# The Notorious B.I.G. (Axel, @axel_ibarrondo)

Say **"I'm Biggie"** (or "soy biggie") to the agent and this is the file it follows.

**Role:** Lead. Codes the contracts and the core, and designs with Easy-E.
**Branch prefix:** `biggie/`
**Labels:** `owner:biggie`

## You own

- `crates/firecal-types/`
- `crates/firecal-query/`
- `crates/firecal-wasm/`
- `crates/firecal-testkit/`
- `xtask/`
- `wasm/`
- `web/stub/`
- `web/tokens/` (with Easy-E)
- and the lead-only files: root config, .github/, OWNERS, AGENTS.md, docs/, crew/

You are the integrator: you review contract changes, keep `main` green, and unblock the others first. Design is yours with Easy-E: tokens and the accessible ramp.

## Start

1. Read `AGENTS.md`, `docs/engineering.md`, `docs/idea.md` and this file.
2. `git pull`, then `cargo xtask check` to be sure your machine is green, and `cargo xtask hooks` once.
3. Find your work: `gh issue list --label owner:biggie --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number that is not blocked.
4. Make your worktree: `cargo xtask worktree biggie/<short-task-name>`, and work only there.
5. Follow `docs/protocol.md`: test first, break it on purpose, run the real thing, then `cargo xtask ready`.
   Never push without it, and never use `--no-verify`.
6. Open a pull request from the template, with `closes #<issue>`, then take the next issue.

## Order

1. Issue 63: export the comparison, analogs, polygon and series CSV through wasm. 2Pac's views wait on it.
2. Issue 10: close it once the team is in the repo (done).
3. Issue 16, on Sunday: end to end on the whole world, after Dre and Cube have run the pipeline.

Between those, review contract changes first and keep `main` green.

The issue numbers are the work; this is only the order, and it is for when the lowest unblocked issue is not obvious.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/biggie-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell the team.
