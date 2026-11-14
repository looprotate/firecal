use super::{ComparisonOut, compare_with_climatology};
use crate::{BaselineYears, QueryError, climatology};
use firecal_testkit::SeededRandom;
use firecal_types::MonthIndex;

fn month(year: u16, month: u16) -> MonthIndex {
    MonthIndex::new(year, month).unwrap()
}

// Normal, by hand: from three baseline years, January 20, February 8 and every other month 0 (median of the
// values 10, 30, 20 and 4, 8, 100 and zeros). The ranges are January 10 to 30, February 4 to 100, March 0 to 0.
fn normal() -> crate::Climatology {
    let mut value = vec![0.0; 36];
    for (at, number) in [
        (0, 10.0),
        (12, 30.0),
        (24, 20.0),
        (1, 4.0),
        (13, 8.0),
        (25, 100.0),
    ] {
        value[at] = number;
    }
    climatology(
        month(2001, 1),
        &value,
        BaselineYears::new(2001, 2003).unwrap(),
    )
    .unwrap()
}

fn compare(
    first: MonthIndex,
    value: &[f64],
    sigma: &[f64],
) -> Result<(Vec<f64>, Vec<bool>), QueryError> {
    let mut anomaly = vec![-9.0; value.len()];
    let mut unusual = vec![true; value.len()];
    let out = ComparisonOut {
        anomaly: &mut anomaly,
        unusual: &mut unusual,
    };
    compare_with_climatology(&normal(), first, value, sigma, out)?;
    Ok((anomaly, unusual))
}

#[test]
fn a_month_above_normal_has_a_positive_anomaly() {
    // January 30 against median 20: (30 - 20) / 20 = 0.5
    let (anomaly, _) = compare(month(2004, 1), &[30.0], &[4.0]).unwrap();
    assert_eq!(anomaly, [0.5]);
}

#[test]
fn a_month_with_no_fire_has_anomaly_minus_one() {
    // February 0 against median 8: (0 - 8) / 8 = -1
    let (anomaly, _) = compare(month(2004, 2), &[0.0], &[1.0]).unwrap();
    assert_eq!(anomaly, [-1.0]);
}

#[test]
fn a_month_equal_to_its_median_has_anomaly_zero_and_is_not_unusual() {
    let (anomaly, unusual) = compare(month(2004, 2), &[8.0], &[1.0]).unwrap();
    assert_eq!((anomaly, unusual), (vec![0.0], vec![false]));
}

#[test]
fn a_difference_over_two_sigma_and_beyond_the_baseline_is_unusual() {
    // |35 - 20| = 15 against 2 * 4 = 8, and 35 is above the January maximum 30
    let (_, unusual) = compare(month(2004, 1), &[35.0], &[4.0]).unwrap();
    assert_eq!(unusual, [true]);
}

#[test]
fn a_value_over_two_sigma_but_inside_the_baseline_range_is_not_unusual() {
    // February 60: |60 - 8| = 52 against 2 * 4 = 8 passes the sensor band, but the baseline has 4 to 100
    let (_, unusual) = compare(month(2004, 2), &[60.0], &[4.0]).unwrap();
    assert_eq!(unusual, [false]);
}

#[test]
fn a_value_beyond_the_baseline_range_but_inside_two_sigma_is_not_unusual() {
    // January 35 is above the maximum 30, but |35 - 20| = 15 against 2 * 8 = 16 is inside the sensor band
    let (_, unusual) = compare(month(2004, 1), &[35.0], &[8.0]).unwrap();
    assert_eq!(unusual, [false]);
}

#[test]
fn a_value_equal_to_the_baseline_minimum_or_maximum_is_not_unusual() {
    // January 10 and 30 are the extremes of the baseline, and both are far over 2 * 1 from the median 20
    let (_, unusual) = compare(month(2004, 1), &[10.0, 30.0], &[1.0, 1.0]).unwrap();
    assert_eq!(unusual, [false, false]);
}

#[test]
fn a_value_just_past_the_baseline_range_is_unusual() {
    // February 101 is above the maximum 100 and |101 - 8| = 93 against 2 * 1; January 9 is below the minimum 10
    let (_, february) = compare(month(2004, 2), &[101.0], &[1.0]).unwrap();
    let (_, january) = compare(month(2004, 1), &[9.0], &[1.0]).unwrap();
    assert_eq!((february, january), (vec![true], vec![true]));
}

#[test]
fn a_difference_of_exactly_two_sigma_is_not_unusual() {
    // |35 - 20| = 15 against 2 * 7.5 = 15: the rule is strict, though 35 is beyond the baseline range
    let (_, unusual) = compare(month(2004, 1), &[35.0], &[7.5]).unwrap();
    assert_eq!(unusual, [false]);
}

#[test]
fn a_drop_below_normal_is_unusual_too() {
    // |5 - 20| = 15 against 2 * 4 = 8
    let (anomaly, unusual) = compare(month(2004, 1), &[5.0], &[4.0]).unwrap();
    assert_eq!((anomaly, unusual), (vec![-0.75], vec![true]));
}

#[test]
fn a_median_below_one_divides_by_one() {
    // March has median 0: (3 - 0) / max(0, 1) = 3, and |3| > 2 * 1
    let (anomaly, unusual) = compare(month(2004, 3), &[3.0], &[1.0]).unwrap();
    assert_eq!((anomaly, unusual), (vec![3.0], vec![true]));
}

#[test]
fn each_month_uses_the_median_of_its_own_calendar_month() {
    // January 20 and February 8 are both exactly normal
    let (anomaly, _) = compare(month(2004, 1), &[20.0, 8.0], &[1.0, 1.0]).unwrap();
    assert_eq!(anomaly, [0.0, 0.0]);
}

#[test]
fn old_contents_of_the_outputs_are_overwritten() {
    let (anomaly, unusual) = compare(month(2004, 2), &[8.0], &[1.0]).unwrap();
    assert_eq!(
        (anomaly, unusual),
        (vec![0.0], vec![false]),
        "they started at -9 and true"
    );
}

#[test]
fn a_month_the_baseline_lacks_is_an_error() {
    let empty = climatology(month(2001, 1), &[], BaselineYears::new(2001, 2003).unwrap()).unwrap();
    let (mut anomaly, mut unusual) = ([0.0], [false]);
    let out = ComparisonOut {
        anomaly: &mut anomaly,
        unusual: &mut unusual,
    };
    assert_eq!(
        compare_with_climatology(&empty, month(2004, 1), &[1.0], &[1.0], out),
        Err(QueryError::NoBaselineForMonth)
    );
}

#[test]
fn buffers_of_different_length_are_an_error() {
    let (mut anomaly, mut unusual) = ([0.0; 2], [false; 2]);
    let out = ComparisonOut {
        anomaly: &mut anomaly,
        unusual: &mut unusual,
    };
    assert_eq!(
        compare_with_climatology(&normal(), month(2004, 1), &[1.0, 2.0], &[1.0], out),
        Err(QueryError::OutputLengthMismatch)
    );
}

#[test]
fn the_anomaly_is_zero_whenever_the_value_equals_the_median() {
    const SEED: u64 = 0xA00A;
    let mut random = SeededRandom::new(SEED);
    for case in 0..300 {
        let value: Vec<f64> = (0..36)
            .map(|_| f64::from(u32::try_from(random.next_below(1000)).unwrap()))
            .collect();
        let baseline = BaselineYears::new(2001, 2003).unwrap();
        let normal = climatology(month(2001, 1), &value, baseline).unwrap();
        let first = month(2010, 1);
        let medians: Vec<f64> = (0..12)
            .map(|offset| {
                normal
                    .median(MonthIndex::from_index(first.index() + offset))
                    .unwrap()
            })
            .collect();
        let (mut anomaly, mut unusual) = ([-1.0; 12], [true; 12]);
        let out = ComparisonOut {
            anomaly: &mut anomaly,
            unusual: &mut unusual,
        };
        compare_with_climatology(&normal, first, &medians, &[0.0; 12], out).unwrap();
        assert_eq!(anomaly, [0.0; 12], "seed {SEED:#x}, case {case}");
        assert_eq!(unusual, [false; 12], "seed {SEED:#x}, case {case}");
    }
}
