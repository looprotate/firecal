//! Aggregates in, harmonized tiles out: VIIRS mapped to MODIS-equivalent counts, with sigma.
//!
//! The library is pure; only the future binary reads and writes files.
//! The calibration specification and its hand-computed fixtures follow.
#![doc = include_str!("../calibration.md")]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod calibration;

pub use calibration::{Calibration, CalibrationError, CountKnot, fit};
