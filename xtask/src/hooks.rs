//! Installs the pre-push hook, so a push runs the same verification the agent is told to run.

use crate::shell::run;

/// Points git at `.githooks/`, where `pre-push` runs `cargo xtask ready`.
pub(crate) fn install() -> Result<(), String> {
    run("git", &["config", "core.hooksPath", ".githooks"])?;
    println!("hooks: installed. A push now runs `cargo xtask ready` first.");
    Ok(())
}
