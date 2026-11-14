//! Aggregates in, harmonized tiles out: VIIRS mapped to MODIS-equivalent counts, with sigma.
//!
//! The library is pure; only the binary reads and writes files.
//! The calibration specification and its hand-computed fixtures follow.
#![doc = include_str!("../calibration.md")]
#![doc = include_str!("../count-tiles.md")]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod calibration;

pub use calibration::{Calibration, CalibrationError, CountKnot, fit};

mod count_tiles;

pub use count_tiles::{CountPlan, CountTile, TileBuildError, apply, fit_aggregates};
