//! FIRMS hotspot files in, per-satellite cell-month aggregates out (`.fca`, see `docs/data-format.md`).
//!
//! The library is pure: it parses bytes and aggregates. Only the future `firecal-ingest` binary opens files.
//!
//! Parse the header once per file; column order may differ between MODIS and VIIRS:
//!
//! ```
//! use firecal_ingest::{CsvHeader, ParseError};
//! use firecal_types::Satellite;
//!
//! let header = CsvHeader::parse(
//!     b"latitude,longitude,acq_date,satellite,instrument,confidence,frp,daynight,type"
//! )?;
//! let hotspot = header.parse_record(b"-33.5,-70.25,2024-01-01,Terra,MODIS,30,12.5,D,0")?;
//! assert_eq!(hotspot.satellite, Satellite::Terra);
//! assert_eq!(hotspot.month.index(), 288);
//! assert_eq!(
//!     header.parse_record(b"-33.5,-70.25,2024-01-01,Terra,MODIS,29,12.5,D,0"),
//!     Err(ParseError::LowConfidence)
//! );
//! # Ok::<(), ParseError>(())
//! ```
//!
//! Empty, low-confidence and non-vegetation rows have explicit skip variants;
//! other errors should be reported as malformed input. Required fields are
//! validated even on filtered rows. NRT files without `type` cannot be filtered
//! by source type. CSV records must occupy one physical line; quoted commas and
//! doubled quotes in unused fields are supported, embedded newlines are rejected.
#![doc = include_str!("../effort-proxy.md")]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod aggregate;
mod csv;
mod date;
mod grid;
mod parser;

pub use aggregate::{AggregateError, Aggregator};
pub use parser::{CsvHeader, Hotspot, ParseError};
