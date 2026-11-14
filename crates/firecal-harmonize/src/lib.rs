//! Aggregates in, harmonized tiles out: MODIS-equivalent counts, estimated count sigma and summed FRP.
//!
//! The library is pure; only the binary reads and writes files.
//! The calibration specification and its hand-computed fixtures follow.
#![doc = include_str!("../calibration.md")]
#![doc = include_str!("../count-tiles.md")]
#![doc = include_str!("../sigma.md")]
#![doc = include_str!("../energy.md")]
#![doc = include_str!("../overlap-report.md")]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod calibration;

pub use calibration::{Calibration, CalibrationError, CountKnot, fit};

mod count_tiles;

pub use count_tiles::{
    CountPlan, CountTile, TileBuildError, apply, apply_with_energy, apply_with_sigma,
    fit_aggregates, fit_energy_aggregates, fit_sigma_aggregates,
};

mod uncertainty;

pub use uncertainty::{OverlapCount, SigmaError, SigmaModel, fit_sigma};

mod energy;

pub use energy::{EnergyCalibration, EnergyError, EnergyKnot, fit_energy};

pub use count_tiles::{OverlapReport, ReportError, overlap_report};
