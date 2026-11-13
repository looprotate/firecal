//! Fails a branch that edits files outside its owner's directories.

use crate::owners::Owners;
use crate::shell::output;

/// Checks the branch in the current checkout against `base` (for example `origin/main`).
pub(crate) fn run(base: &str) -> Result<(), String> {
    let branch = match std::env::var("GITHUB_HEAD_REF") {
        Ok(name) if !name.is_empty() => name,
        _ => output("git", &["rev-parse", "--abbrev-ref", "HEAD"])?,
    };
    if branch == "main" {
        println!("guard: on main, nothing to check");
        return Ok(());
    }
    let owners = Owners::read_from_repository()?;
    let range = format!("{base}...HEAD");
    let changed = output("git", &["diff", "--name-only", &range])?;
    let files: Vec<&str> = changed.lines().filter(|line| !line.is_empty()).collect();
    let foreign = foreign_files(&owners, &branch, &files)?;
    if foreign.is_empty() {
        println!("guard: {} file(s), all editable by `{branch}`", files.len());
        Ok(())
    } else {
        Err(format!(
            "branch `{branch}` edits files it does not own:\n  {}\nundo them, or write a request in requests/",
            foreign.join("\n  ")
        ))
    }
}

/// The files in `files` that the owner named by the branch prefix may not edit.
pub(crate) fn foreign_files<'a>(
    owners: &Owners,
    branch: &str,
    files: &[&'a str],
) -> Result<Vec<&'a str>, String> {
    let owner = branch.split_once('/').map_or("", |(prefix, _)| prefix);
    if !owners.has_owner(owner) {
        return Err(format!(
            "branch `{branch}` must be named <owner>/<task> with an owner from OWNERS"
        ));
    }
    Ok(files
        .iter()
        .copied()
        .filter(|file| !owners.may_edit(owner, file))
        .collect())
}

#[cfg(test)]
mod tests;
