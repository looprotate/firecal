//! Questions asked of harmonized data: the series of an area, its climatology, anomaly, analog years and
//! CSV text. Pure functions over slices; nothing here touches the world.
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod analogs;
mod area;
mod baseline;
mod cells_csv;
mod climatology;
mod comparison;
mod csv_text;
mod error;
mod polygon;
mod root;
mod series;
mod series_csv;
mod span;

pub use analogs::{Analog, Season, analog_years};
pub use area::{Area, Rectangle};
pub use baseline::BaselineYears;
pub use cells_csv::write_cells_csv;
pub use climatology::{Climatology, climatology};
pub use comparison::{ComparisonOut, compare_with_climatology};
pub use error::QueryError;
pub use polygon::{PolygonArea, Vertex};
pub use series::{SeriesOut, series};
pub use series_csv::{SeriesRows, write_series_csv};
pub use span::month_span;
