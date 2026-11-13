//! Shared vocabulary of firecal: the grid, months, satellites and errors.
//!
//! This crate is a contract. It is `no_std` so that the compiler itself proves it can run in the browser.
//! Changing a public item here is a contract change: see `docs/engineering.md`.
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod aggregate;
mod bytes;
mod cell;
mod error;
mod month;
mod satellite;
mod tile;

pub use aggregate::{
    AGGREGATE_MAGIC, AggregateRecord, AggregateView, aggregate_len, write_aggregate,
};
pub use cell::{CellIndex, GRID_COLUMNS, GRID_ROWS};
pub use error::Error;
pub use month::{BASE_YEAR, FIRST_FIRMS_MONTH, MonthIndex};
pub use satellite::Satellite;
pub use tile::{
    Plane, TILE_CELLS, TILE_COLUMNS, TILE_MAGIC, TILE_ROWS, TILE_SIDE, TileId, TileView, tile_len,
    write_tile,
};
