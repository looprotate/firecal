//! Synthetic FCA1 files in isolated temporary directories.

use firecal_types::{
    AggregateRecord, CellIndex, MonthIndex, Satellite, aggregate_len, write_aggregate,
};
use std::path::{Path, PathBuf};

pub(super) struct Fixture {
    pub(super) root: PathBuf,
    pub(super) inputs: Vec<PathBuf>,
}
impl Fixture {
    pub(super) fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("firecal-count-{}-{name}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let mut inputs = Vec::new();
        for (satellite, month, counts) in [
            (Satellite::Terra, 276, vec![2, 4]),
            (Satellite::SuomiNpp, 276, vec![4, 8]),
            (Satellite::Terra, 277, vec![]),
            (Satellite::SuomiNpp, 277, vec![8]),
            (Satellite::SuomiNpp, 278, vec![3, 6]),
        ] {
            let path = root.join(format!("{}-{month}.fca", satellite.code()));
            write(&path, satellite, month, &counts);
            inputs.push(path);
        }
        Self { root, inputs }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).unwrap();
    }
}
fn write(path: &Path, satellite: Satellite, month: u16, counts: &[u32]) {
    let records: Vec<_> = counts
        .iter()
        .enumerate()
        .map(|(col, &count)| AggregateRecord {
            cell: CellIndex::new(0, u32::try_from(col).unwrap()).unwrap(),
            count,
            frp_megawatts: 0.0,
            day_count: 0,
        })
        .collect();
    let mut bytes = vec![0; aggregate_len(records.len()).unwrap()];
    write_aggregate(
        satellite,
        MonthIndex::from_index(month),
        &records,
        &mut bytes,
    )
    .unwrap();
    std::fs::write(path, bytes).unwrap();
}
