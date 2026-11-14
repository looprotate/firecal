//! Questions asked of harmonized data: the series of an area, its climatology, anomaly, analog years and
//! CSV text. Pure functions over slices; nothing here touches the world.
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod area;
mod error;
mod root;
mod series;
mod span;

pub use area::Rectangle;
pub use error::QueryError;
pub use series::{SeriesOut, series};
pub use span::month_span;
