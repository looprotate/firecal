//! What a diff must not contain, as pure functions over the text of `git diff --unified=0`.

use std::path::Path;

const MIN_SECRET_VALUE_CHARS: usize = 8;

/// Everything wrong with `diff`; empty when it is fine to push.
pub(crate) fn problems(diff: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut path = "";
    let mut paths_with_new_functions = Vec::new();
    let mut tests_touched = false;
    for line in diff.lines() {
        if let Some(name) = line.strip_prefix("+++ b/") {
            path = name;
            if is_forbidden_path(path) {
                found.push(format!(
                    "{path}: data, tiles, keys and .env files never go to git"
                ));
            }
            tests_touched |= is_test_file(path);
        } else if let Some(added) = line.strip_prefix('+') {
            if is_rules_source(path) {
                continue;
            }
            if holds_secret(added) {
                found.push(format!(
                    "{path}: looks like a key or secret; keys come from the environment"
                ));
            }
            if !has_extension(path, "md") && is_unissued_todo(added) {
                found.push(format!(
                    "{path}: TODO or FIXME without an issue number (`TODO(#12)`)"
                ));
            }
            if declares_function(added)
                && is_source_file(path)
                && !paths_with_new_functions.contains(&path)
            {
                paths_with_new_functions.push(path);
            }
        }
    }
    if !paths_with_new_functions.is_empty() && !tests_touched {
        found.push(format!(
            "new functions in {} but no test file changed: every function that computes has a test",
            paths_with_new_functions.join(", ")
        ));
    }
    found
}

fn is_forbidden_path(path: &str) -> bool {
    path.starts_with("data/")
        || path.starts_with("tiles/")
        || path == ".env"
        || path.ends_with("/.env")
        || has_extension(path, "key")
        || has_extension(path, "pem")
}

/// The rules and their tests name the very markers they look for.
fn is_rules_source(path: &str) -> bool {
    path.starts_with("xtask/src/ready/")
}

fn has_extension(path: &str, extension: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|found| found.eq_ignore_ascii_case(extension))
}

fn is_test_file(path: &str) -> bool {
    path.ends_with("tests.rs") || path.contains("/tests/") || path.ends_with(".test.ts")
}

fn is_source_file(path: &str) -> bool {
    has_extension(path, "rs")
        && !is_test_file(path)
        && (path.starts_with("crates/") || path.starts_with("xtask/"))
}

fn declares_function(added: &str) -> bool {
    let trimmed = added.trim_start();
    [
        "fn ",
        "pub fn ",
        "pub(crate) fn ",
        "const fn ",
        "pub const fn ",
    ]
    .iter()
    .any(|start| trimmed.starts_with(start))
}

fn holds_secret(added: &str) -> bool {
    added.contains("BEGIN PRIVATE KEY")
        || added.split_once("MAP_KEY=").is_some_and(|(_, value)| {
            value
                .chars()
                .take_while(char::is_ascii_alphanumeric)
                .count()
                >= MIN_SECRET_VALUE_CHARS
        })
}

fn is_unissued_todo(added: &str) -> bool {
    (added.contains("TODO") || added.contains("FIXME")) && !added.contains("(#")
}

#[cfg(test)]
mod tests;
