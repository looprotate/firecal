//! The monthly series of an analysis request: the whole history the climatology and the analogs are read from.

use crate::analysis_request::{
    ANALYSIS_REQUEST_BYTES, AnalysisRequest, VERTEX_BYTES, parse_vertices,
};
use crate::memory::Memory;
use crate::series_call::{prepare, read_table, rectangle_of, sum_over, tile_files};
use crate::status::{INVALID, status_of};
use firecal_query::{PolygonArea, Vertex};
use firecal_types::MonthIndex;

/// The value and sigma of each month, starting at `first_month`.
pub(crate) struct History {
    pub(crate) first_month: MonthIndex,
    pub(crate) value: Vec<f64>,
    pub(crate) sigma: Vec<f64>,
}

/// Reads the analysis request at `address` and sums its series over its rectangle or polygon.
///
/// Everything is read and checked here; the caller writes only after this succeeds.
pub(crate) fn read_history<M: Memory>(
    memory: &M,
    address: u32,
    len: u32,
) -> Result<(AnalysisRequest, History), u32> {
    if len != ANALYSIS_REQUEST_BYTES {
        return Err(INVALID);
    }
    let request =
        AnalysisRequest::parse(memory.read(address, len).ok_or(INVALID)?).ok_or(INVALID)?;
    let entries = read_table(memory, &request.series)?;
    let files = tile_files(memory, &entries)?;
    let prepared = prepare(&request.series, &files)?;
    let (value, sigma) = if request.vertex_count == 0 {
        sum_over(&prepared, &rectangle_of(&request.series)?)?
    } else {
        let corners = corners_of(memory, &request)?;
        let mut scratch = corners.clone();
        let polygon = PolygonArea::new(&corners, &mut scratch).map_err(status_of)?;
        sum_over(&prepared, &polygon)?
    };
    let history = History {
        first_month: prepared.first_month,
        value,
        sigma,
    };
    Ok((request, history))
}

fn corners_of<M: Memory>(memory: &M, request: &AnalysisRequest) -> Result<Vec<Vertex>, u32> {
    let len = request
        .vertex_count
        .checked_mul(VERTEX_BYTES)
        .ok_or(INVALID)?;
    let bytes = memory.read(request.vertices, len).ok_or(INVALID)?;
    parse_vertices(bytes).ok_or(INVALID)
}

#[cfg(test)]
pub(crate) mod tests;
