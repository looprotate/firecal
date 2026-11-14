//! The analysis request block: a series request plus a baseline, a season and an optional polygon, as bytes in
//! linear memory. The layout is described in `wasm/firecal.d.ts`; this module is the only Rust code that knows it.

use crate::request::{REQUEST_BYTES, Request, double, word};
use firecal_query::Vertex;

/// Bytes of an analysis request block.
pub(crate) const ANALYSIS_REQUEST_BYTES: u32 = 80;
/// Bytes of one polygon corner: latitude then longitude, `f64`.
pub(crate) const VERTEX_BYTES: u32 = 16;

/// An analysis request block, read from its bytes. Little-endian.
///
/// | offset | type  | field |
/// |---|---|---|
/// | 0 to 55 | | a series request ([`Request`]); its output address is not used |
/// | 56, 60 | `u32` | first and last baseline year |
/// | 64 | `u32` | year of the season in progress |
/// | 68 | `u32` | months of the season, 1 to 12 |
/// | 72 | `u32` | polygon corner count; 0 means the rectangle of the series request |
/// | 76 | `u32` | address of the polygon corners |
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AnalysisRequest {
    pub(crate) series: Request,
    pub(crate) baseline_first: u32,
    pub(crate) baseline_last: u32,
    pub(crate) season_year: u32,
    pub(crate) season_months: u32,
    pub(crate) vertex_count: u32,
    pub(crate) vertices: u32,
}

impl AnalysisRequest {
    /// Reads a block; `None` unless `bytes` is exactly [`ANALYSIS_REQUEST_BYTES`] long.
    pub(crate) fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != usize::try_from(ANALYSIS_REQUEST_BYTES).ok()? {
            return None;
        }
        let series_end = usize::try_from(REQUEST_BYTES).ok()?;
        Some(Self {
            series: Request::parse(bytes.get(..series_end)?)?,
            baseline_first: word(bytes, 56)?,
            baseline_last: word(bytes, 60)?,
            season_year: word(bytes, 64)?,
            season_months: word(bytes, 68)?,
            vertex_count: word(bytes, 72)?,
            vertices: word(bytes, 76)?,
        })
    }
}

/// The polygon corners in `bytes`, `vertex_count * VERTEX_BYTES` long; `None` for any other length.
pub(crate) fn parse_vertices(bytes: &[u8]) -> Option<Vec<Vertex>> {
    let width = usize::try_from(VERTEX_BYTES).ok()?;
    if !bytes.len().is_multiple_of(width) {
        return None;
    }
    bytes
        .chunks_exact(width)
        .map(|corner| {
            Some(Vertex {
                latitude: double(corner, 0)?,
                longitude: double(corner, 8)?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;
