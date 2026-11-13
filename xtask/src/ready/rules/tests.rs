use super::problems;

fn none() -> Vec<String> {
    Vec::new()
}

fn added(path: &str, lines: &[&str]) -> String {
    let body = lines
        .iter()
        .fold(String::new(), |text, line| text + "+" + line + "\n");
    format!("diff --git a/{path} b/{path}\n--- a/{path}\n+++ b/{path}\n@@ -0,0 +1 @@\n{body}")
}

#[test]
fn an_empty_diff_is_fine() {
    assert_eq!(problems(""), none());
}

#[test]
fn a_function_with_its_test_is_fine() {
    let diff = added(
        "crates/firecal-types/src/month.rs",
        &["pub fn year(self) -> u16 {"],
    ) + &added(
        "crates/firecal-types/src/month/tests.rs",
        &["fn year_is_two_thousand() {"],
    );
    assert_eq!(problems(&diff), none());
}

#[test]
fn a_new_function_without_any_test_change_is_flagged() {
    let diff = added(
        "crates/firecal-types/src/month.rs",
        &["    pub const fn year(self) -> u16 {"],
    );
    let found = problems(&diff);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("no test file changed"));
}

#[test]
fn a_function_in_a_test_file_needs_no_other_test() {
    let diff = added(
        "crates/firecal-types/src/month/tests.rs",
        &["fn helper() {"],
    );
    assert_eq!(problems(&diff), none());
}

#[test]
fn a_function_in_the_web_or_docs_is_not_a_rust_source() {
    let diff = added("docs/engineering.md", &["fn example() {"]);
    assert_eq!(problems(&diff), none());
}

#[test]
fn data_tiles_and_env_files_are_flagged() {
    for path in [
        "data/a.csv",
        "tiles/t_1_1.bin",
        ".env",
        "web/.env",
        "keys/x.pem",
        "x.key",
    ] {
        assert_eq!(problems(&added(path, &["x"])).len(), 1, "{path}");
    }
}

#[test]
fn a_map_key_with_a_value_is_flagged() {
    let diff = added("README.md", &["FIRMS_MAP_KEY=abcd1234efgh5678"]);
    assert_eq!(problems(&diff).len(), 1);
}

#[test]
fn a_map_key_mentioned_by_name_only_is_fine() {
    let diff = added(
        "README.md",
        &["Set FIRMS_MAP_KEY=<your key>", "export FIRMS_MAP_KEY=$KEY"],
    );
    assert_eq!(problems(&diff), none());
}

#[test]
fn a_private_key_block_is_flagged() {
    let diff = added("notes.txt", &["-----BEGIN PRIVATE KEY-----"]);
    assert_eq!(problems(&diff).len(), 1);
}

#[test]
fn a_todo_needs_an_issue_number() {
    assert_eq!(problems(&added("README.md", &["TODO fix later"])).len(), 1);
    assert_eq!(problems(&added("README.md", &["FIXME"])).len(), 1);
    assert_eq!(
        problems(&added("README.md", &["TODO(#12) fix later"])),
        none()
    );
}

#[test]
fn markdown_may_talk_about_todos() {
    assert_eq!(
        problems(&added("docs/protocol.md", &["no TODO without an issue"])),
        none()
    );
}

#[test]
fn the_rules_source_may_name_what_it_looks_for() {
    let diff = added(
        "xtask/src/ready/rules.rs",
        &["FIRMS_MAP_KEY=abcd1234efgh5678", "TODO"],
    );
    assert_eq!(problems(&diff), none());
}

#[test]
fn removed_lines_are_not_judged() {
    let diff = "+++ b/README.md\n-FIRMS_MAP_KEY=abcd1234efgh5678\n-TODO later\n";
    assert_eq!(problems(diff), none());
}
