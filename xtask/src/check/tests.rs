use super::web_steps;

#[test]
fn there_are_no_web_steps_until_the_web_app_exists() {
    assert_eq!(web_steps(false), Vec::<&[&str]>::new());
}

#[test]
fn the_web_app_is_installed_exactly_then_typed_tested_and_built() {
    let steps = web_steps(true);
    assert_eq!(steps.len(), 5);
    assert_eq!(steps[0], ["--prefix", "web", "ci"]);
    assert_eq!(steps[1], ["--prefix", "web", "run", "typecheck"]);
    assert_eq!(
        steps[2],
        [
            "--prefix",
            "web",
            "exec",
            "--",
            "tsc",
            "-p",
            "wasm/tsconfig.json"
        ]
    );
    assert_eq!(steps[3], ["--prefix", "web", "test"]);
    assert_eq!(steps[4], ["--prefix", "web", "run", "build"]);
}
