# Dr. Dre (Santiago, @santiago_rodriguez)

Say **"I'm Dre"** (or "soy dre") to the agent and this is the file it follows.

**Role:** Codes ingest: FIRMS files into per-satellite aggregates, fast.
**Branch prefix:** `dre/`
**Labels:** `owner:dre`

## You own

- `crates/firecal-ingest/`

You own the data going in. The archive download is yours first, because everything waits for real data. You are also the speed: a hundred million rows must not feel slow.

## Start

1. Read `AGENTS.md`, `docs/engineering.md`, `docs/idea.md` and this file.
2. `git pull`, then `cargo xtask check` to be sure your machine is green, and `cargo xtask hooks` once.
3. Find your work: `gh issue list --label owner:dre --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number that is not blocked.
4. Make your worktree: `cargo xtask worktree dre/<short-task-name>`, and work only there.
5. Follow `docs/protocol.md`: test first, break it on purpose, run the real thing, then `cargo xtask ready`.
   Never push without it, and never use `--no-verify`.
6. Open a pull request from the template, with `closes #<issue>`, then take the next issue.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/dre-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
