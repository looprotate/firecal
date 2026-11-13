# 2Pac (Brandon, @brandon_bignoli)

Say **"I'm 2Pac"** (or "soy 2pac") to the agent and this is the file it follows.

**Role:** Codes the calendar, the charts and the analog years view.
**Branch prefix:** `2pac/`
**Labels:** `owner:2pac`

## You own

- `web/calendar/`

You own the hero of the demo: the calendar. It reads a `Series` and draws it. It never computes statistics.

## Start

1. Read `AGENTS.md`, `docs/engineering.md`, `docs/idea.md` and this file.
2. `git pull`, then `cargo xtask check` to be sure your machine is green, and `cargo xtask hooks` once.
3. Find your work: `gh issue list --label owner:2pac --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number that is not blocked.
4. Make your worktree: `cargo xtask worktree 2pac/<short-task-name>`, and work only there.
5. Follow `docs/protocol.md`: test first, break it on purpose, run the real thing, then `cargo xtask ready`.
   Never push without it, and never use `--no-verify`.
6. Open a pull request from the template, with `closes #<issue>`, then take the next issue.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/2pac-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
