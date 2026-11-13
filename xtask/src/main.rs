//! Project tasks, the same on macOS and Windows: `cargo xtask <task>`.
//!
//! - `check [--full]`: format, lints, tests, and the wasm build; `--full` adds the release profile.
//! - `guard [base]`: fails when the branch edits files its owner does not own.
//! - `worktree <owner>/<task>`: a fresh worktree and branch for one agent.
//! - `pr-lint`: checks the pull request title and description in `PR_TITLE` and `PR_BODY`.
//! - `build-wasm`: builds the browser module and copies it to `web/public/firecal.wasm`.

mod build_wasm;
mod check;
mod guard;
mod owners;
mod pr_lint;
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
        ["pr-lint"] => pr_lint::run(),
        ["build-wasm"] => build_wasm::run(),
        _ => Err(String::from(
            "usage: cargo xtask check [--full] | guard [base] | worktree <owner>/<task> | pr-lint | build-wasm",
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
