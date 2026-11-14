//! The series request block and the tile table entries, as bytes in linear memory. The layout is described
//! in `wasm/firecal.d.ts`; this module is the only Rust code that knows it.

/// Bytes of a request block.
pub(crate) const REQUEST_BYTES: u32 = 56;
/// Bytes of one entry of the tile table.
pub(crate) const TILE_ENTRY_BYTES: u32 = 16;

/// A request block, read from its bytes. Little-endian.
///
/// | offset | type  | field |
/// |---|---|---|
/// | 0, 8, 16, 24 | `f64` | south, north, west, east, in degrees |
/// | 32 | `u32` | first month index |
/// | 36 | `u32` | month count |
/// | 40 | `u32` | plane: 0 count, 1 energy |
/// | 44 | `u32` | tile count |
/// | 48 | `u32` | address of the tile table |
/// | 52 | `u32` | address of the output |
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Request {
    pub(crate) south: f64,
    pub(crate) north: f64,
    pub(crate) west: f64,
    pub(crate) east: f64,
    pub(crate) first_month: u32,
    pub(crate) month_count: u32,
    pub(crate) plane: u32,
    pub(crate) tile_count: u32,
    pub(crate) tile_table: u32,
    pub(crate) output: u32,
}

/// One tile of the table: where it is in the 18 x 36 tile grid and where its file is in memory.
///
/// | offset | type  | field |
/// |---|---|---|
/// | 0 | `u32` | tile row |
/// | 4 | `u32` | tile column |
/// | 8 | `u32` | address of the tile file |
/// | 12 | `u32` | length of the tile file |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TileEntry {
    pub(crate) row: u32,
    pub(crate) col: u32,
    pub(crate) address: u32,
    pub(crate) len: u32,
}

fn word(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

fn double(bytes: &[u8], at: usize) -> Option<f64> {
    Some(f64::from_le_bytes(bytes.get(at..at + 8)?.try_into().ok()?))
}

impl Request {
    /// Reads a request block; `None` unless `bytes` is exactly [`REQUEST_BYTES`] long.
    pub(crate) fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != usize::try_from(REQUEST_BYTES).ok()? {
            return None;
        }
        Some(Self {
            south: double(bytes, 0)?,
            north: double(bytes, 8)?,
            west: double(bytes, 16)?,
            east: double(bytes, 24)?,
            first_month: word(bytes, 32)?,
            month_count: word(bytes, 36)?,
            plane: word(bytes, 40)?,
            tile_count: word(bytes, 44)?,
            tile_table: word(bytes, 48)?,
            output: word(bytes, 52)?,
        })
    }
}

impl TileEntry {
    /// Reads the tile table, `bytes` being `tile_count * 16` bytes; `None` for a length that is not a multiple.
    pub(crate) fn parse_table(bytes: &[u8]) -> Option<Vec<Self>> {
        let width = usize::try_from(TILE_ENTRY_BYTES).ok()?;
        if !bytes.len().is_multiple_of(width) {
            return None;
        }
        bytes
            .chunks_exact(width)
            .map(|entry| {
                Some(Self {
                    row: word(entry, 0)?,
                    col: word(entry, 4)?,
                    address: word(entry, 8)?,
                    len: word(entry, 12)?,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
