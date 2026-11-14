//! The browser build. Every export is a plain `extern "C"` function over numbers or linear-memory
//! buffers, described in `wasm/firecal.d.ts`; there is no `wasm-bindgen`.
//!
//! Status convention: a function that can fail returns a status, and [`INVALID`] (`u32::MAX`) for anything it
//! cannot read, instead of trapping.
#![expect(
    unsafe_code,
    reason = "the unsafe here is the no_mangle attribute that exports a function, and reading linear memory"
)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

#[cfg(target_arch = "wasm32")]
mod allocation;
mod analogs_call;
mod analysis_request;
mod area_series_call;
mod cells_csv_call;
mod comparison;
mod comparison_call;
mod history;
mod memory;
mod output;
mod request;
mod series_call;
mod series_csv_call;
mod status;

pub use analogs_call::run_analogs;
pub use area_series_call::run_area_series;
pub use cells_csv_call::run_cells_csv;
pub use comparison_call::run_comparison;
pub use memory::{Memory, SliceMemory};
pub use series_call::run_series;
pub use series_csv_call::run_series_csv;
pub use status::{
    INVALID, STATUS_AREA_OUT_OF_RANGE, STATUS_BASELINE_INVALID, STATUS_EMPTY_AREA,
    STATUS_MONTH_NOT_IN_TILE, STATUS_MONTH_RANGE_INVALID, STATUS_OK, STATUS_OUTPUT_TOO_SMALL,
    STATUS_POLYGON_INVALID, STATUS_SEASON_INVALID, STATUS_TILE_INVALID,
};

use firecal_query::month_span;
use firecal_types::MonthIndex;

/// The month index of `month` (1 to 12) of `year`, or [`INVALID`].
#[unsafe(no_mangle)]
pub extern "C" fn fc_month_index(year: u32, month: u32) -> u32 {
    let (Ok(year), Ok(month)) = (u16::try_from(year), u16::try_from(month)) else {
        return INVALID;
    };
    MonthIndex::new(year, month).map_or(INVALID, |index| u32::from(index.index()))
}

/// The number of months from `first` to `last` inclusive, or [`INVALID`].
#[unsafe(no_mangle)]
pub extern "C" fn fc_month_span(first: u32, last: u32) -> u32 {
    let (Ok(first), Ok(last)) = (u16::try_from(first), u16::try_from(last)) else {
        return INVALID;
    };
    month_span(MonthIndex::from_index(first), MonthIndex::from_index(last))
        .map_or(INVALID, u32::from)
}

/// A zeroed block of `len` bytes of linear memory for the page to fill; its address, or [`INVALID`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_alloc(len: u32) -> u32 {
    allocation::allocate(len)
}

/// Gives back a block from [`fc_alloc`]: zero, or [`INVALID`] when `address` is not a block held.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_free(address: u32) -> u32 {
    allocation::free(address)
}

/// Writes the `cells.csv` of the request block at `request_address` into the buffer at `buffer_address`: see
/// [`run_cells_csv`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_cells_csv(
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    run_cells_csv(
        &mut memory::LinearMemory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    )
}

/// Answers the series request block of `request_len` bytes at `request_address`: see [`run_series`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_series(request_address: u32, request_len: u32) -> u32 {
    run_series(&mut memory::LinearMemory, request_address, request_len)
}

/// The series of an analysis request block over its rectangle or polygon: see [`run_area_series`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_area_series(
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    run_area_series(
        &mut memory::LinearMemory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    )
}

/// The median, anomaly and unusual flag of each month of an analysis request block: see [`run_comparison`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_comparison(
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    run_comparison(
        &mut memory::LinearMemory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    )
}

/// The analog years of an analysis request block: see [`run_analogs`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_analogs(
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    run_analogs(
        &mut memory::LinearMemory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    )
}

/// The `series.csv` of an analysis request block: see [`run_series_csv`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_series_csv(
    request_address: u32,
    request_len: u32,
    buffer_address: u32,
    buffer_len: u32,
) -> u32 {
    run_series_csv(
        &mut memory::LinearMemory,
        request_address,
        request_len,
        buffer_address,
        buffer_len,
    )
}

#[cfg(test)]
mod tests;
