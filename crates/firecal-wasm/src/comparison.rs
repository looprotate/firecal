//! How each month of a history compares with the median of its calendar month over the baseline years.

use crate::analysis_request::AnalysisRequest;
use crate::history::History;
use crate::status::{STATUS_BASELINE_INVALID, status_of};
use core::slice::{from_mut, from_ref};
use firecal_query::{
    BaselineYears, ComparisonOut, QueryError, climatology, compare_with_climatology,
};
use firecal_types::MonthIndex;

/// One entry per month. A month whose calendar month has no baseline value has a NaN median and anomaly and is
/// not unusual: the page shows it as unknown, not as zero.
pub(crate) struct Comparison {
    pub(crate) median: Vec<f64>,
    pub(crate) anomaly: Vec<f64>,
    pub(crate) unusual: Vec<bool>,
}

/// The baseline years of the request.
pub(crate) fn baseline_of(request: &AnalysisRequest) -> Result<BaselineYears, u32> {
    let first = u16::try_from(request.baseline_first).map_err(|_| STATUS_BASELINE_INVALID)?;
    let last = u16::try_from(request.baseline_last).map_err(|_| STATUS_BASELINE_INVALID)?;
    BaselineYears::new(first, last).map_err(status_of)
}

/// Compares every month of `history` with the climatology of the request's baseline.
pub(crate) fn compare(request: &AnalysisRequest, history: &History) -> Result<Comparison, u32> {
    let normal = climatology(history.first_month, &history.value, baseline_of(request)?)
        .map_err(status_of)?;
    let months = history.value.len();
    let mut out = Comparison {
        median: vec![f64::NAN; months],
        anomaly: vec![f64::NAN; months],
        unusual: vec![false; months],
    };
    let rows = history
        .value
        .iter()
        .zip(&history.sigma)
        .zip(out.median.iter_mut().zip(out.anomaly.iter_mut()))
        .zip(out.unusual.iter_mut())
        .enumerate();
    for (offset, (((value, sigma), (median, anomaly)), unusual)) in rows {
        let at = u16::try_from(offset)
            .ok()
            .and_then(|offset| history.first_month.index().checked_add(offset))
            .ok_or(crate::status::STATUS_MONTH_RANGE_INVALID)?;
        let month = MonthIndex::from_index(at);
        let one = ComparisonOut {
            anomaly: from_mut(anomaly),
            unusual: from_mut(unusual),
        };
        match compare_with_climatology(&normal, month, from_ref(value), from_ref(sigma), one) {
            Ok(()) => *median = normal.median(month).unwrap_or(f64::NAN),
            Err(QueryError::NoBaselineForMonth) => {}
            Err(error) => return Err(status_of(error)),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
