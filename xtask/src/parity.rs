//! `parity`: the same calls through the native build and through `firecal.wasm` under Node must give
//! identical bytes. A difference means the browser would show other numbers than the pipeline.

mod cases;

use crate::build_wasm::{BUILT, run_build};
use crate::shell::run as shell;
use cases::{CASE_BYTES, all_cases, encode, native_results};
use std::fs;
use std::path::{Path, PathBuf};

const NODE_RUNNER: &str = "xtask/parity/run.mjs";

pub(crate) fn run() -> Result<(), String> {
    run_build()?;
    let cases = all_cases();
    let folder = scratch_folder();
    fs::create_dir_all(&folder)
        .map_err(|error| format!("cannot create {}: {error}", folder.display()))?;
    let outcome = compare_through_node(&folder, &encode(&cases), &native_results(&cases));
    // Best effort: a leftover temp folder must not hide the real outcome.
    drop(fs::remove_dir_all(&folder));
    outcome?;
    println!("parity: {} calls, identical bytes", cases.len());
    Ok(())
}

fn scratch_folder() -> PathBuf {
    std::env::temp_dir().join(format!("firecal-parity-{}", std::process::id()))
}

fn compare_through_node(folder: &Path, cases: &[u8], native: &[u8]) -> Result<(), String> {
    let cases_file = folder.join("cases.bin");
    let results_file = folder.join("results.bin");
    fs::write(&cases_file, cases)
        .map_err(|error| format!("cannot write {}: {error}", cases_file.display()))?;
    shell(
        "node",
        &[
            NODE_RUNNER,
            BUILT,
            &cases_file.to_string_lossy(),
            &results_file.to_string_lossy(),
        ],
    )?;
    let wasm = fs::read(&results_file)
        .map_err(|error| format!("cannot read {}: {error}", results_file.display()))?;
    first_difference(native, &wasm).map_or(Ok(()), Err)
}

/// A sentence naming the first call whose answers differ, or `None` when the bytes are identical.
pub(crate) fn first_difference(native: &[u8], wasm: &[u8]) -> Option<String> {
    if native.len() != wasm.len() {
        return Some(format!(
            "native answered {} bytes, wasm {}",
            native.len(),
            wasm.len()
        ));
    }
    let at = native
        .iter()
        .zip(wasm)
        .position(|(left, right)| left != right)?;
    Some(format!(
        "call {} differs: native {:?}, wasm {:?} (answers are 4 bytes; a call is {CASE_BYTES} bytes)",
        at / 4,
        native.get(at / 4 * 4..at / 4 * 4 + 4),
        wasm.get(at / 4 * 4..at / 4 * 4 + 4),
    ))
}

#[cfg(test)]
mod tests;
