//! Stable recursive discovery without following symlinks into duplicate files or loops.

use std::{
    fs,
    path::{Path, PathBuf},
};

pub(super) fn csv_files(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect(directory, &mut paths)?;
    paths.sort();
    if paths.is_empty() {
        return Err("no CSV files found in input directory".into());
    }
    Ok(paths)
}

fn collect(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        let path = entry.path();
        if kind.is_dir() {
            collect(&path, paths)?;
        } else if kind.is_file()
            && path
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("csv"))
        {
            paths.push(path);
        }
    }
    Ok(())
}
