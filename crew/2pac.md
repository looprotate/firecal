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

## Order

1. Issue 5: onboarding.
2. Issue 35: the calendar heatmap from `web/stub/series.json`, with the ramp and marks in `web/tokens/tokens.css`. It needs Easy-E's scaffold (issue 17); until it is merged, write the calendar as plain modules and tests in `web/calendar/`.
3. Issues 36 and 37: the anomaly strip and the CSV buttons. They need issue 63 for real data and use the stub until then.
4. Issues 38 and 39: count and energy modes, and the analog years view.
5. Issue 40: the before and after figure, which needs Cube's overlap report (issue 34).
6. Issue 41, on Sunday: the accessibility pass.

The issue numbers are the work; this is only the order, and it is for when the lowest unblocked issue is not obvious.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/2pac-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
