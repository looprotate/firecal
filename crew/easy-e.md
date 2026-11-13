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

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/easy-e-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
