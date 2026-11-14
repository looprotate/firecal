# Eminem (Americo, @americomenendez)

Say **"I'm Eminem"** (or "soy eminem") to the agent and this is the file it follows.

**Role:** Does not code. Owns the story, the demo, the video and the submission.
**Branch prefix:** `eminem/`
**Labels:** `owner:eminem`

## You own

- `pitch/`

You own what the judges see and hear. No Rust or TypeScript for you: you write, you record, and you test the app as a user on Windows.

## Start

1. Read `AGENTS.md`, `docs/idea.md`, `docs/plan.md`, section 6 of `docs/protocol.md` and this file.
2. `git pull`.
3. Find your work: `gh issue list --label owner:eminem --state open --json number,title,milestone`. Take the
   lowest milestone first, and inside it the lowest issue number.
4. Your files are in `pitch/`. Write there; for anything outside it, write a request in `requests/`.
5. Open a pull request from the template, with `closes #<issue>`, then take the next issue.
6. You do not need `cargo`. When an issue asks you to run the app, the agent helps you run it and you look at it.

## Order

1. Issue 6: onboarding. Check the app on your Windows machine as soon as there is one to run.
2. Issue 42: pick the story, the place and the season. Everyone's demo depends on it.
3. Issue 43: the judging criteria and the one-line pitch.
4. Issues 44 and 45: the three-minute demo script, and the measured numbers as the team produces them.
5. Issue 46, before the polish on Sunday: record the backup video.
6. Issue 47: the submission text and project page.

The issue numbers are the work; this is only the order, and it is for when the lowest unblocked issue is not obvious.

## Never

- Edit a file outside what you own. `cargo xtask guard` fails the pull request, so do not try.
- Need something from someone else? Write `requests/eminem-to-<owner>-<topic>.md` and work against a stub.
- Change a contract on your own. If one looks wrong, stop and tell Biggie.
