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
mod memory;
mod request;
mod series_call;
mod status;

pub use memory::{Memory, SliceMemory};
pub use series_call::run_series;
pub use status::{
    INVALID, STATUS_AREA_OUT_OF_RANGE, STATUS_EMPTY_AREA, STATUS_MONTH_NOT_IN_TILE,
    STATUS_MONTH_RANGE_INVALID, STATUS_OK, STATUS_TILE_INVALID,
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

/// Answers the series request block of `request_len` bytes at `request_address`: see [`run_series`].
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn fc_series(request_address: u32, request_len: u32) -> u32 {
    run_series(&mut memory::LinearMemory, request_address, request_len)
}

#[cfg(test)]
mod tests;
