//! Builds the browser module and puts it where the web app serves it.

use crate::shell::{cargo, run as shell};
use std::fs;
use std::path::Path;

pub(crate) const BUILT: &str = "target/wasm32-unknown-unknown/release/firecal_wasm.wasm";
const SERVED: &str = "web/public/firecal.wasm";

pub(crate) fn run() -> Result<(), String> {
    run_build()?;
    let served = Path::new(SERVED);
    if let Some(folder) = served.parent() {
        fs::create_dir_all(folder)
            .map_err(|error| format!("cannot create {}: {error}", folder.display()))?;
    }
    fs::copy(BUILT, served).map_err(|error| format!("cannot copy {BUILT} to {SERVED}: {error}"))?;
    println!("build-wasm: {SERVED}");
    Ok(())
}

pub(crate) fn run_build() -> Result<(), String> {
    shell(
        &cargo(),
        &[
            "build",
            "--release",
            "--package",
            "firecal-wasm",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )
}
