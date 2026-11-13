//! The gate before a push: everything an agent must have verified, checked by the machine.

use crate::check;
use crate::guard;
use crate::shell::output;

mod rules;

/// Runs every verification a branch needs before it is pushed, and stops at the first that fails.
pub(crate) fn run() -> Result<(), String> {
    let branch = output("git", &["rev-parse", "--abbrev-ref", "HEAD"])?;
    if branch == "main" {
        println!("ready: on main, nothing to push from here");
        return Ok(());
    }
    require_clean_tree()?;
    output("git", &["fetch", "--quiet", "origin", "main"])?;
    require_rebased()?;
    let diff = output("git", &["diff", "--unified=0", "origin/main...HEAD"])?;
    let found = rules::problems(&diff);
    if !found.is_empty() {
        return Err(format!(
            "the diff has problems:\n  - {}",
            found.join("\n  - ")
        ));
    }
    check::run(true)?;
    guard::run("origin/main")?;
    println!(
        "ready: verified. Push, then fill the pull request template; `pr-lint` checks the title and text."
    );
    Ok(())
}

fn require_clean_tree() -> Result<(), String> {
    if output("git", &["status", "--porcelain"])?.is_empty() {
        Ok(())
    } else {
        Err(String::from(
            "uncommitted changes: commit or stash them, so what is verified is what is pushed",
        ))
    }
}

fn require_rebased() -> Result<(), String> {
    let status = std::process::Command::new("git")
        .args(["merge-base", "--is-ancestor", "origin/main", "HEAD"])
        .status()
        .map_err(|error| format!("cannot start git: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(String::from(
            "the branch is behind origin/main: run `git rebase origin/main`, then verify again",
        ))
    }
}
