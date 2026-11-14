//! Questions asked of harmonized data: the series of an area, its climatology, anomaly, analog years and
//! CSV text. Pure functions over slices; nothing here touches the world.
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod analogs;
mod area;
mod baseline;
mod climatology;
mod comparison;
mod error;
mod root;
mod series;
mod span;

pub use analogs::{Analog, Season, analog_years};
pub use area::Rectangle;
pub use baseline::BaselineYears;
pub use climatology::{Climatology, climatology};
pub use comparison::{ComparisonOut, compare_with_climatology};
pub use error::QueryError;
pub use series::{SeriesOut, series};
pub use span::month_span;
