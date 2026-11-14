use super::climatology;
use crate::{BaselineYears, QueryError};
use firecal_testkit::SeededRandom;
use firecal_types::MonthIndex;

// Fixture, by hand: three years from January 2001 (index 12), 36 months. Only January and February are not
// zero:
//            2001   2002   2003
//   January    10     30     20
//   February    4      8    100
fn three_years() -> Vec<f64> {
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
    value
}

fn month(year: u16, month: u16) -> MonthIndex {
    MonthIndex::new(year, month).unwrap()
}

fn start() -> MonthIndex {
    month(2001, 1)
}

fn years(first: u16, last: u16) -> BaselineYears {
    BaselineYears::new(first, last).unwrap()
}

#[test]
fn the_median_of_three_years_is_the_middle_one() {
    let normal = climatology(start(), &three_years(), years(2001, 2003)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), Some(20.0));
    assert_eq!(normal.median(month(2030, 2)), Some(8.0));
}

#[test]
fn the_median_of_two_years_is_their_mean() {
    let normal = climatology(start(), &three_years(), years(2001, 2002)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), Some(20.0));
    assert_eq!(normal.median(month(2030, 2)), Some(6.0));
}

#[test]
fn a_baseline_of_one_year_gives_that_year() {
    let normal = climatology(start(), &three_years(), years(2002, 2002)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), Some(30.0));
}

#[test]
fn a_month_with_only_zeros_has_median_zero() {
    let normal = climatology(start(), &three_years(), years(2001, 2003)).unwrap();
    assert_eq!(normal.median(month(2030, 7)), Some(0.0));
}

#[test]
fn a_baseline_outside_the_series_has_no_median() {
    let normal = climatology(start(), &three_years(), years(2010, 2012)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), None);
}

#[test]
fn a_series_shorter_than_a_year_has_no_median_for_the_months_it_lacks() {
    let normal = climatology(month(2001, 6), &[5.0, 6.0], years(2001, 2001)).unwrap();
    assert_eq!(normal.median(month(2030, 6)), Some(5.0));
    assert_eq!(normal.median(month(2030, 7)), Some(6.0));
    assert_eq!(normal.median(month(2030, 8)), None);
}

#[test]
fn an_empty_series_has_no_median() {
    let normal = climatology(start(), &[], years(2001, 2003)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), None);
}

#[test]
fn the_order_of_the_years_does_not_change_the_median() {
    let mut value = vec![0.0; 36];
    for (at, number) in [(0, 30.0), (12, 20.0), (24, 10.0)] {
        value[at] = number;
    }
    let normal = climatology(start(), &value, years(2001, 2003)).unwrap();
    assert_eq!(normal.median(month(2030, 1)), Some(20.0));
}

#[test]
fn more_than_sixty_four_years_of_one_month_is_too_long() {
    let value = vec![1.0; 12 * 65];
    assert_eq!(
        climatology(start(), &value, years(2001, 2100)),
        Err(QueryError::BaselineTooLong)
    );
}

#[test]
fn the_median_lies_between_the_smallest_and_largest_value() {
    const SEED: u64 = 0xC11A;
    let mut random = SeededRandom::new(SEED);
    for case in 0..300 {
        let months = 12 * usize::try_from(1 + random.next_below(20)).unwrap();
        let value: Vec<f64> = (0..months)
            .map(|_| f64::from(u32::try_from(random.next_below(5000)).unwrap()))
            .collect();
        let normal = climatology(start(), &value, years(2001, 2030)).unwrap();
        let january: Vec<f64> = value.iter().step_by(12).copied().collect();
        let low = january.iter().copied().fold(f64::INFINITY, f64::min);
        let high = january.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let median = normal.median(month(2030, 1)).unwrap();
        assert!(
            low <= median && median <= high,
            "seed {SEED:#x}, case {case}"
        );
    }
}
