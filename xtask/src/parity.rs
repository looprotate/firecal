//! `parity`: the same calls through the native build and through `firecal.wasm` under Node must give
//! identical bytes. A difference means the browser would show other numbers than the pipeline.

mod cases;
mod series_cases;

use crate::build_wasm::{BUILT, run_build};
use crate::shell::run as shell;
use cases::{all_cases, encode, native_results};
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
    let at = native
        .iter()
        .zip(wasm)
        .position(|(left, right)| left != right);
    if at.is_none() && native.len() == wasm.len() {
        return None;
    }
    let offset = at.unwrap_or_else(|| native.len().min(wasm.len()));
    Some(format!(
        "call {} differs at byte {offset} of the answers: native has {} bytes, wasm {}",
        call_containing(native, offset),
        native.len(),
        wasm.len()
    ))
}

/// The position of the call whose framed answer holds `offset`; answers are a `u32` length then the bytes.
fn call_containing(answers: &[u8], offset: usize) -> usize {
    let mut start = 0;
    let mut call = 0;
    while let Some(length) = answers
        .get(start..start + 4)
        .and_then(|word| <[u8; 4]>::try_from(word).ok())
        .and_then(|word| usize::try_from(u32::from_le_bytes(word)).ok())
    {
        start += 4 + length;
        if offset < start {
            break;
        }
        call += 1;
    }
    call
}

#[cfg(test)]
mod tests;
