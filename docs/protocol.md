# The protocol: write, verify, push

How every agent writes code here, and what must be true before anything leaves the machine. It is the
checklist behind `docs/engineering.md`, in the order you do it. Humans follow it too.

**Nothing is pushed on trust.** Either you ran it and saw it pass, or you say you did not.

## 1. Before you write

1. You have said who you are, read your crew file, and picked an issue. One issue at a time.
2. You are in your own worktree (`cargo xtask worktree <alias>/<task>`), on a branch cut from the latest
   `origin/main`. Run `cargo xtask hooks` once on the machine, so a push cannot skip step 5.
3. Read the contract you implement or consume (`docs/data-format.md`, the crate's public API, `wasm/firecal.d.ts`).
   If it is wrong or missing, **stop**: write a request or tell the lead. Never edit it to suit you.
4. Say in a sentence what "done" looks like for this issue, and which test will prove it. If you cannot,
   you do not understand the task yet.

## 2. While you write

1. **Test first.** Write the test, run it, watch it fail for the right reason. Then make it pass. A bug gets a failing
   test before the fix.
2. **Expected values come from outside the code.** Work them out by hand or with an independent tool, and write
   down where they came from. Never paste the program's own output into the expectation.
3. **Small steps.** After each small step the tests are green. A step is a function, not a feature.
4. **The rules are not negotiable:** no `unwrap`/`panic`/indexing in libraries, no casts that lose bits, no `usize`
   in a format, no `exp`/`ln`/`powf` in the core, no hash-order output, no dependencies, no new files outside
   what you own. A lint in the way means the code is wrong; do not silence it.
5. **Names say what a thing does, files stay near 200 lines, public items have docs.** Write the code you would want
   to read at 3 a.m. in front of a judge.
6. **No leftovers.** No dead code, no commented-out code, no `TODO` without an issue (`TODO(#12)`), no debug
   prints, no scratch files.
7. Stop and ask when: a contract looks wrong, you need a file you do not own, a dependency, a lint seems in the
   way, a test cannot be deterministic, or you have been stuck twenty minutes on one error.

## 3. Verify the code did what you meant

1. **Break it on purpose.** For anything that computes, change the code so it is wrong (flip a comparison, drop
   a term) and watch a test fail. Put it back. A test that cannot fail proves nothing.
2. **Run the real thing.** The binary on the fixture, the page in the browser, the file it writes. Look at the
   output with your own eyes (open the page, read the bytes, compare to what you expected by hand). "It compiles"
   and "tests pass" are not "it works".
3. **Edge cases from `docs/engineering.md`**: empty, one, maximum, saturation, antimeridian, poles, missing tile,
   CRLF and BOM, bad numbers. Each one that applies has a test.
4. **A change that must not change the output** is proven by running old and new on the same input and
   comparing the bytes.
5. **Read your own diff as a stranger would.** `git diff origin/main...HEAD`: is every line needed, named well,
   inside your directories, free of surprises?

## 4. The gate: `cargo xtask ready`

Run it before every push. It stops at the first failure, and the pre-push hook runs it for you. It checks:

1. the working tree is clean, so what is verified is what is pushed;
2. the branch contains the latest `origin/main` (rebase if not);
3. the diff has no data, tiles, keys or `.env` files, no secrets, no `TODO` without an issue, and no new function
   without a test file touched;
4. `cargo xtask check --full`: format, clippy with every lint denied, all tests, the wasm build, and the same in the
   release profile;
5. `cargo xtask guard`: every file belongs to your owner.

If `ready` fails, fix the cause. **Never** push with `--no-verify`, never weaken a lint or a test to pass,
never delete a test that fails, never `force` anything shared. CI runs the same checks on macOS, Windows and Linux,
so skipping them only moves the failure to where it is slower to fix.

When the core changed (`types`, `query`, `wasm`), also run `cargo xtask parity` once it exists.

## 5. The pull request

1. Push the branch and open the pull request from the template. The **title** is the commit: imperative,
   capitalized, at most 72 characters, no period. The **Why** says the reason, not the change.
2. **Ran** lists the exact commands and what they printed. **Not run** says what you could not run and why, and
   "none" only when it is true. **Evidence** is a number, a screenshot or a diff of outputs.
3. Report outcomes as they are. If something failed or was skipped, say so; a green summary over a red result is
   the worst thing you can push.
4. CI must go green on all three systems and `pr-lint`. A failure is yours to fix, and it is fixed in a new
   commit on the same branch.
5. When it merges, it is squashed into one commit. Delete the worktree, and take the next issue.

## 6. Work that is not code (`pitch/`)

The same discipline, different checks: every number has a source in `pitch/numbers.md` and was measured; every
claim matches `docs/idea.md` ("What we do not claim"); every external image, text or idea is in
`docs/references.md`; links work; the demo was run from the script by someone who did not write it.

## What you hand back

When an issue is done you can say, truthfully: the test failed first, I broke the code and a test caught it, I ran
the real thing and looked at the output, `cargo xtask ready` passed, the pull request says what I did and did not
run. If you cannot say all of that, the issue is not done.
