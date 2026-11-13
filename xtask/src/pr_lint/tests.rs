use super::problems;

const GOOD_BODY: &str = "**What:** Count hotspots per cell and month.\n\
**Why:** The calendar needs one number per cell, per month, per satellite.\n\
**Issue:** closes #12\n\
**Contract:** untouched\n\
**Ran:** cargo xtask check --full, green on macOS.\n\
**Not run:** Windows.\n\
**Evidence:** fixture of ten hotspots gives the counts written by hand.";

fn no_problems() -> Vec<String> {
    Vec::new()
}

#[test]
fn a_good_title_and_body_have_no_problems() {
    assert_eq!(
        problems("Aggregate hotspots per cell and month", GOOD_BODY),
        Vec::<String>::new()
    );
}

#[test]
fn a_title_over_72_characters_is_rejected() {
    let title = format!("Add {}", "x".repeat(70));
    assert_eq!(problems(&title, GOOD_BODY).len(), 1);
}

#[test]
fn a_title_of_exactly_72_characters_is_accepted() {
    let title = format!("Add {}", "x".repeat(68));
    assert_eq!(title.chars().count(), 72);
    assert_eq!(problems(&title, GOOD_BODY), no_problems());
}

#[test]
fn a_lowercase_title_is_rejected() {
    assert_eq!(problems("add the aggregator", GOOD_BODY).len(), 1);
}

#[test]
fn a_title_with_a_final_period_is_rejected() {
    assert_eq!(problems("Add the aggregator.", GOOD_BODY).len(), 1);
}

#[test]
fn a_non_imperative_first_word_is_rejected() {
    assert_eq!(problems("Added the aggregator", GOOD_BODY).len(), 1);
    assert_eq!(problems("Fixes the aggregator", GOOD_BODY).len(), 1);
}

#[test]
fn imperative_words_that_end_in_s_are_fine() {
    assert_eq!(
        problems("Process the archive in parallel", GOOD_BODY),
        no_problems()
    );
}

#[test]
fn wip_in_the_title_is_rejected() {
    assert_eq!(problems("Add the aggregator WIP", GOOD_BODY).len(), 1);
}

#[test]
fn an_empty_title_reports_the_capital_letter() {
    assert_ne!(problems("", GOOD_BODY), no_problems());
}

#[test]
fn an_empty_body_asks_for_every_required_section() {
    assert_eq!(problems("Add the aggregator", "").len(), 4);
}

#[test]
fn a_template_placeholder_is_not_real_text() {
    let body = GOOD_BODY.replace(
        "The calendar needs one number per cell, per month, per satellite.",
        "<the reason>",
    );
    let found = problems("Add the aggregator", &body);
    assert_eq!(found, vec![String::from("fill in **Why:** with real text")]);
}

#[test]
fn a_section_may_continue_on_the_next_lines() {
    let body = GOOD_BODY.replace("**Why:** The calendar", "**Why:**\nThe calendar");
    assert_eq!(problems("Add the aggregator", &body), no_problems());
}

#[test]
fn tool_attribution_is_rejected() {
    let body = format!("{GOOD_BODY}\n\nCo-Authored-By: Somebody <a@b.c>");
    assert_eq!(problems("Add the aggregator", &body).len(), 1);
    let body = format!("{GOOD_BODY}\n\nGenerated with a tool");
    assert_eq!(problems("Add the aggregator", &body).len(), 1);
}
