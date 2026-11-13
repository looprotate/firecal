//! The `.fca` aggregate (FCA1): one satellite, one month, sparse per-cell counts. Written by ingest, read by
//! harmonize. The layout is in `docs/data-format.md`; this module is the only code that knows it.

use crate::bytes::{Writer, read_f32, read_u16, read_u32};
use crate::{CellIndex, Error, GRID_COLUMNS, MonthIndex, Satellite};

/// The first four bytes of every aggregate file.
pub const AGGREGATE_MAGIC: [u8; 4] = *b"FCA1";

const HEADER_BYTES: usize = 12;
const RECORD_BYTES: usize = 16;

/// What was detected in one cell in one month by one satellite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AggregateRecord {
    /// The cell.
    pub cell: CellIndex,
    /// Detections kept (nominal and high confidence).
    pub count: u32,
    /// Sum of the fire radiative power of those detections, in megawatts.
    pub frp_megawatts: f32,
    /// How many of `count` were made in daylight.
    pub day_count: u32,
}

/// The exact size in bytes of an aggregate with `records` records, or `None` if it overflows.
#[must_use]
pub const fn aggregate_len(records: usize) -> Option<usize> {
    match records.checked_mul(RECORD_BYTES) {
        Some(body) => body.checked_add(HEADER_BYTES),
        None => None,
    }
}

/// Writes an aggregate into `out` and returns how many bytes were written.
///
/// # Errors
/// [`Error::RecordsNotSorted`] unless cells strictly increase, [`Error::RecordInvalid`] for a negative or
/// non-finite energy or `day_count > count`, [`Error::TooManyRecords`], and [`Error::BufferTooSmall`].
pub fn write_aggregate(
    satellite: Satellite,
    month: MonthIndex,
    records: &[AggregateRecord],
    out: &mut [u8],
) -> Result<usize, Error> {
    let record_count = u32::try_from(records.len()).map_err(|_| Error::TooManyRecords)?;
    let mut writer = Writer::new(out);
    writer.put(&AGGREGATE_MAGIC)?;
    writer.put(&[satellite.code(), 0])?;
    writer.put(&month.index().to_le_bytes())?;
    writer.put(&record_count.to_le_bytes())?;
    let mut previous: Option<CellIndex> = None;
    for record in records {
        check_record(record)?;
        if previous.is_some_and(|before| before >= record.cell) {
            return Err(Error::RecordsNotSorted);
        }
        previous = Some(record.cell);
        writer.put(&record.cell.index().to_le_bytes())?;
        writer.put(&record.count.to_le_bytes())?;
        writer.put(&record.frp_megawatts.to_le_bytes())?;
        writer.put(&record.day_count.to_le_bytes())?;
    }
    Ok(writer.written())
}

fn check_record(record: &AggregateRecord) -> Result<(), Error> {
    let energy_is_valid = record.frp_megawatts.is_finite() && record.frp_megawatts >= 0.0;
    if energy_is_valid && record.day_count <= record.count {
        Ok(())
    } else {
        Err(Error::RecordInvalid)
    }
}

/// A validated aggregate file, read in place without copying.
#[derive(Debug, Clone, Copy)]
pub struct AggregateView<'a> {
    satellite: Satellite,
    month: MonthIndex,
    body: &'a [u8],
}

impl<'a> AggregateView<'a> {
    /// Checks the whole file, so every later read is infallible.
    ///
    /// # Errors
    /// [`Error::Truncated`], [`Error::TrailingBytes`], [`Error::BadMagic`], [`Error::ReservedNotZero`],
    /// [`Error::UnknownSatellite`], [`Error::CellOutsideGrid`], [`Error::RecordsNotSorted`] and
    /// [`Error::RecordInvalid`].
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        let header = bytes.get(..HEADER_BYTES).ok_or(Error::Truncated)?;
        if header.get(..4) != Some(&AGGREGATE_MAGIC[..]) {
            return Err(Error::BadMagic);
        }
        let satellite = Satellite::from_code(*header.get(4).ok_or(Error::Truncated)?)?;
        if header.get(5) != Some(&0) {
            return Err(Error::ReservedNotZero);
        }
        let month = MonthIndex::from_index(read_u16(header, 6).ok_or(Error::Truncated)?);
        let record_count = read_u32(header, 8).ok_or(Error::Truncated)?;
        let body_len = usize::try_from(record_count)
            .ok()
            .and_then(|count| count.checked_mul(RECORD_BYTES))
            .ok_or(Error::Truncated)?;
        let body = bytes.get(HEADER_BYTES..).ok_or(Error::Truncated)?;
        match body.len().cmp(&body_len) {
            core::cmp::Ordering::Less => return Err(Error::Truncated),
            core::cmp::Ordering::Greater => return Err(Error::TrailingBytes),
            core::cmp::Ordering::Equal => {}
        }
        let view = Self {
            satellite,
            month,
            body,
        };
        let mut previous: Option<CellIndex> = None;
        for chunk in body.as_chunks::<RECORD_BYTES>().0 {
            let record = decode_record(chunk)?;
            if previous.is_some_and(|before| before >= record.cell) {
                return Err(Error::RecordsNotSorted);
            }
            previous = Some(record.cell);
        }
        Ok(view)
    }

    /// The satellite that made these detections.
    #[must_use]
    pub const fn satellite(&self) -> Satellite {
        self.satellite
    }

    /// The month they were made in.
    #[must_use]
    pub const fn month(&self) -> MonthIndex {
        self.month
    }

    /// The number of records.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.body.len() / RECORD_BYTES
    }

    /// True when no cell had a detection.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    /// The records, in increasing cell order.
    pub fn records(&self) -> impl Iterator<Item = AggregateRecord> + '_ {
        self.body
            .as_chunks::<RECORD_BYTES>()
            .0
            .iter()
            .filter_map(|chunk| decode_record(chunk).ok())
    }
}

fn decode_record(chunk: &[u8]) -> Result<AggregateRecord, Error> {
    let cell_index = read_u32(chunk, 0).ok_or(Error::Truncated)?;
    let cell = CellIndex::new(cell_index / GRID_COLUMNS, cell_index % GRID_COLUMNS)?;
    let record = AggregateRecord {
        cell,
        count: read_u32(chunk, 4).ok_or(Error::Truncated)?,
        frp_megawatts: read_f32(chunk, 8).ok_or(Error::Truncated)?,
        day_count: read_u32(chunk, 12).ok_or(Error::Truncated)?,
    };
    check_record(&record)?;
    Ok(record)
}

#[cfg(test)]
mod tests;
