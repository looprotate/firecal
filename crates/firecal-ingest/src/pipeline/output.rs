//! New output directories only; FCA1 encoding belongs to the shared library.

use super::Groups;
use firecal_types::{Satellite, aggregate_len};
use std::{fs, io::Write, path::Path};

pub(super) fn write(directory: &Path, groups: &Groups) -> Result<u32, String> {
    if let Some(parent) = directory.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::create_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    for ((satellite, month), aggregator) in groups {
        let folder = directory.join(name(*satellite));
        fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        let path = folder.join(format!("{:04}{:02}.fca", month.year(), month.month()));
        let records = aggregator.records().map_err(|e| format!("{e:?}"))?;
        let mut bytes = vec![0; aggregate_len(records.len()).ok_or("aggregate size overflow")?];
        aggregator.write(&mut bytes).map_err(|e| format!("{e:?}"))?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        file.write_all(&bytes)
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    u32::try_from(groups.len()).map_err(|_| "too many aggregate files".into())
}

const fn name(satellite: Satellite) -> &'static str {
    match satellite {
        Satellite::Terra => "terra",
        Satellite::Aqua => "aqua",
        Satellite::SuomiNpp => "snpp",
        Satellite::Noaa20 => "noaa20",
        Satellite::Noaa21 => "noaa21",
    }
}
