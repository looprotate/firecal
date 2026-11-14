use super::run_comparison;
use crate::history::tests::{Call, doubles};
use crate::{STATUS_BASELINE_INVALID, STATUS_OK, STATUS_OUTPUT_TOO_SMALL};

// Baseline 2001 to 2002: the median of January is (10 + 30) / 2 = 20, of February (20 + 40) / 2 = 30, of the
// other months 100. Sigma is 2, so a difference of more than 4 clears the sensor band; the baseline ranges are
// January 10 to 30, February 20 to 40, and 100 to 100 for the other months. Unusual needs both.
fn run(call: &Call) -> (u32, Vec<f64>, Vec<f64>, Vec<u8>) {
    let (status, buffer) = call.run(36 * 17, |m, r, l, b, n| run_comparison(m, r, l, b, n));
    let (numbers, flags) = buffer.split_at(36 * 16);
    let (median, anomaly) = numbers.split_at(36 * 8);
    (status, doubles(median), doubles(anomaly), flags.to_vec())
}

#[test]
fn the_current_season_is_compared_with_the_baseline_median() {
    let (status, median, anomaly, flags) = run(&Call::fixture());
    assert_eq!(status, STATUS_OK);
    assert_eq!(median[24..27], [20.0, 30.0, 100.0]);
    // January 2003: (5 - 20) / 20 = -0.75; February: (22 - 30) / 30; March: 0.
    assert_eq!(anomaly[24].to_bits(), (-0.75_f64).to_bits());
    assert_eq!(anomaly[25].to_bits(), (-8.0_f64 / 30.0).to_bits());
    assert_eq!(anomaly[26].to_bits(), 0.0_f64.to_bits());
    // January 5 is below 10 and 15 from the median: unusual. February 22 is 8 from the median, over the sensor
    // band, but inside 20 to 40: not unusual. March is the median.
    assert_eq!(flags[24..27], [1, 0, 0]);
}

#[test]
fn a_difference_of_exactly_twice_sigma_is_not_unusual() {
    // November 2001 is month 10 of the series and has 100, like the median: zero difference, not unusual.
    let (_, _, anomaly, flags) = run(&Call::fixture());
    assert_eq!(anomaly[10].to_bits(), 0.0_f64.to_bits());
    assert_eq!(flags[10], 0);
}

#[test]
fn months_without_a_baseline_are_nan_and_not_unusual() {
    let call = Call {
        baseline: (2010, 2011),
        ..Call::fixture()
    };
    let (status, median, anomaly, flags) = run(&call);
    assert_eq!(status, STATUS_OK);
    assert!(median.iter().chain(&anomaly).all(|number| number.is_nan()));
    assert!(flags.iter().all(|flag| *flag == 0));
}

#[test]
fn a_baseline_that_ends_before_it_starts_is_refused() {
    let call = Call {
        baseline: (2002, 2001),
        ..Call::fixture()
    };
    assert_eq!(
        call.run(36 * 17, |m, r, l, b, n| run_comparison(m, r, l, b, n))
            .0,
        STATUS_BASELINE_INVALID
    );
    let wide = Call {
        baseline: (2001, 70_000),
        ..Call::fixture()
    };
    assert_eq!(
        wide.run(36 * 17, |m, r, l, b, n| run_comparison(m, r, l, b, n))
            .0,
        STATUS_BASELINE_INVALID
    );
}

#[test]
fn a_buffer_that_is_too_small_is_reported() {
    assert_eq!(
        Call::fixture()
            .run(36 * 17 - 1, |m, r, l, b, n| run_comparison(m, r, l, b, n))
            .0,
        STATUS_OUTPUT_TOO_SMALL
    );
}
