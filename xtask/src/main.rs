//! Project tasks, the same on macOS and Windows: `cargo xtask <task>`.
//!
//! - `check [--full]`: format, lints, tests, the wasm build and its Node test; `--full` adds the release profile.
//! - `guard [base]`: fails when the branch edits files its owner does not own.
//! - `worktree <owner>/<task>`: a fresh worktree and branch for one agent.
//! - `ready`: the gate before a push: clean tree, rebased, diff rules, `check --full`, `guard`.
//! - `hooks`: installs the pre-push hook that runs `ready`.
//! - `pr-lint`: checks the pull request title and description in `PR_TITLE` and `PR_BODY`.
//! - `parity`: the same calls through the native build and `firecal.wasm` under Node must give identical bytes.
//! - `build-wasm`: builds the browser module and copies it to `web/public/firecal.wasm`.

mod build_wasm;
mod check;
mod guard;
mod hooks;
mod owners;
mod parity;
mod pr_lint;
mod ready;
mod shell;
mod worktree;

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let result = match words.as_slice() {
        ["check"] => check::run(false),
        ["check", "--full"] => check::run(true),
        ["guard"] => guard::run("origin/main"),
        ["guard", base] => guard::run(base),
        ["worktree", branch] => worktree::run(branch),
        ["ready"] => ready::run(),
        ["hooks"] => hooks::install(),
        ["pr-lint"] => pr_lint::run(),
        ["build-wasm"] => build_wasm::run(),
        ["parity"] => parity::run(),
        _ => Err(String::from(
            "usage: cargo xtask check [--full] | guard [base] | worktree <owner>/<task> | ready | hooks | pr-lint | build-wasm | parity",
        )),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("xtask: {message}");
            ExitCode::FAILURE
        }
    }
}
