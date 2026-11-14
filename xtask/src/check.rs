//! The check every change passes before a pull request.

use crate::shell::{cargo, run as shell};
use std::path::Path;

/// Format, lints, tests, the wasm build and the Node test that loads it. With `full`, also the release profile, which has its own
/// optimizer and must give the same answers. Once `web/package.json` exists, the web app is checked too.
pub(crate) fn run(full: bool) -> Result<(), String> {
    let cargo = cargo();
    shell(&cargo, &["fmt", "--all", "--", "--check"])?;
    shell(
        &cargo,
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    shell(&cargo, &["test", "--workspace"])?;
    shell(
        &cargo,
        &[
            "build",
            "--package",
            "firecal-wasm",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    shell(
        &cargo,
        &[
            "clippy",
            "--package",
            "firecal-wasm",
            "--target",
            "wasm32-unknown-unknown",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    shell("node", &["--test", "wasm/firecal.test.ts"])?;
    if full {
        shell(
            &cargo,
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--release",
                "--",
                "-D",
                "warnings",
            ],
        )?;
        shell(&cargo, &["test", "--workspace", "--release"])?;
    }
    for step in web_steps(Path::new("web/package.json").exists()) {
        shell(npm(), step)?;
    }
    println!("check: green");
    Ok(())
}

/// The npm steps that check the web app: none until it exists, then install exactly, the types of the app and of
/// the wasm loader, tests, build.
pub(crate) fn web_steps(web_exists: bool) -> Vec<&'static [&'static str]> {
    if !web_exists {
        return Vec::new();
    }
    vec![
        &["--prefix", "web", "ci"],
        &["--prefix", "web", "run", "typecheck"],
        &[
            "--prefix",
            "web",
            "exec",
            "--",
            "tsc",
            "-p",
            "wasm/tsconfig.json",
        ],
        &["--prefix", "web", "test"],
        &["--prefix", "web", "run", "build"],
    ]
}

/// `npm` is a batch file on Windows, which cannot be started by its short name.
const fn npm() -> &'static str {
    if cfg!(windows) { "npm.cmd" } else { "npm" }
}

#[cfg(test)]
mod tests;
