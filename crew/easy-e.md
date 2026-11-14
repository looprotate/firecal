# Eazy-E (Kevin, @kevin_arregin)

Say **"I'm Easy-E"** (or "soy easy-e") to the agent and this is the file it follows.

**Role:** Team owner. Codes the web app shell, the map and offline, and designs with Biggie.
**Branch prefix:** `easy-e/`
**Labels:** `owner:easy-e`

## You own

- `web/app/`
- `web/map/`
- `web/tokens/` (with Biggie)
- web/package.json, web/package-lock.json, web/tsconfig.json, web/vite.config.ts, web/index.html

You own how the app is put together: the page, the map, the service worker, and the web dependencies. You decide the look with Biggie.

## Start

1. Read `AGENTS.md`, `docs/engineering.md`, `docs/idea.md` and this file.
2. `git pull`, then `cargo xtask check` to be sure your machine is green, and `cargo xtask hooks` once.
3. Find your work: `gh issue list --label owner:easy-e --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number that is not blocked.
4. Make your worktree: `cargo xtask worktree easy-e/<short-task-name>`, and work only there.
5. Follow `docs/protocol.md`: test first, break it on purpose, run the real thing, then `cargo xtask ready`.
   Never push without it, and never use `--no-verify`.
6. Open a pull request from the template, with `closes #<issue>`, then take the next issue.

## Order

1. Issue 2: onboarding.
2. Issue 17: scaffold the web app. It unblocks everyone in `web/`, so do it first. It must define `typecheck`, `test` and `build` in `web/package.json`; `test` runs `node --test`, which also picks up `web/tokens/ramp.test.mjs`.
3. Issue 18: the map with rectangle drawing and lat/lon entry.
4. Issue 19: wire the app against `web/stub/series.json`, then against wasm once issue 63 is merged.
5. Issues 20 and 21: offline and polygon drawing.
6. Issue 22, on Sunday: the smoke test with the network off.

Use the tokens in `web/tokens/tokens.css`; never a literal color.

The issue numbers are the work; this is only the order, and it is for when the lowest unblocked issue is not obvious.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/easy-e-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
