//! Checks a pull request's title and description. Pull requests are squash-merged, so the title becomes the
//! commit title on `main` and the description becomes its body: this is how the history stays clean.

const MAX_TITLE_CHARS: usize = 72;
const FILLED_SECTIONS: [&str; 4] = ["What", "Why", "Ran", "Evidence"];
const MIN_SECTION_CHARS: usize = 8;
const NOT_IMPERATIVE: [&str; 14] = [
    "added", "adds", "adding", "fixed", "fixes", "fixing", "updated", "updates", "updating",
    "changed", "changes", "changing", "removed", "removes",
];
const ATTRIBUTION: [&str; 3] = ["co-authored-by:", "generated with", "🤖"];

/// Reads `PR_TITLE` and `PR_BODY` from the environment, as set by the workflow, and fails on any problem.
pub(crate) fn run() -> Result<(), String> {
    let title = std::env::var("PR_TITLE").map_err(|_| String::from("PR_TITLE is not set"))?;
    let body = std::env::var("PR_BODY").unwrap_or_default();
    let found = problems(&title, &body);
    if found.is_empty() {
        println!("pr-lint: the title and description are fine");
        Ok(())
    } else {
        Err(format!(
            "the pull request is not ready to be a commit:\n  - {}",
            found.join("\n  - ")
        ))
    }
}

/// Everything wrong with a pull request's `title` and `body`; empty when it is fine.
pub(crate) fn problems(title: &str, body: &str) -> Vec<String> {
    let mut found = title_problems(title);
    found.extend(body_problems(body));
    found
}

fn title_problems(title: &str) -> Vec<String> {
    let mut found = Vec::new();
    if title.chars().count() > MAX_TITLE_CHARS {
        found.push(format!(
            "the title is longer than {MAX_TITLE_CHARS} characters"
        ));
    }
    if !title
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_uppercase())
    {
        found.push(String::from("the title must start with a capital letter"));
    }
    if title.trim_end().ends_with('.') {
        found.push(String::from("the title must not end with a period"));
    }
    let first_word = title
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if NOT_IMPERATIVE.contains(&first_word.as_str()) {
        found.push(format!(
            "the title must be imperative (\"Add\", not \"{first_word}\")"
        ));
    }
    if title.to_ascii_lowercase().contains("wip") {
        found.push(String::from(
            "the title says wip: finish it or leave it as a draft",
        ));
    }
    found
}

fn body_problems(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    for name in FILLED_SECTIONS {
        let text = section(body, name);
        if text.chars().count() < MIN_SECTION_CHARS || text.starts_with('<') {
            found.push(format!("fill in **{name}:** with real text"));
        }
    }
    let lowered = body.to_lowercase();
    for marker in ATTRIBUTION {
        if lowered.contains(marker) {
            found.push(format!("remove the tool attribution (`{marker}`)"));
        }
    }
    found
}

/// The text of the section that starts with `**<name>:**`, up to the next `**` line, trimmed.
fn section(body: &str, name: &str) -> String {
    let marker = format!("**{name}:**");
    let mut text = String::new();
    let mut inside = false;
    for line in body.lines() {
        if let Some(rest) = line.trim_start().strip_prefix(marker.as_str()) {
            inside = true;
            text.push_str(rest);
        } else if inside && line.trim_start().starts_with("**") {
            break;
        } else if inside {
            text.push(' ');
            text.push_str(line);
        }
    }
    text.trim().to_owned()
}

#[cfg(test)]
mod tests;
