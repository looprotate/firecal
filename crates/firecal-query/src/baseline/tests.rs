use super::BaselineYears;
use crate::QueryError;

#[test]
fn a_baseline_contains_both_ends_and_nothing_outside() {
    let baseline = BaselineYears::new(2005, 2007).unwrap();
    assert!(!baseline.contains(2004));
    assert!(baseline.contains(2005) && baseline.contains(2006) && baseline.contains(2007));
    assert!(!baseline.contains(2008));
}

#[test]
fn one_year_is_a_baseline() {
    assert!(BaselineYears::new(2010, 2010).unwrap().contains(2010));
}

#[test]
fn a_baseline_that_ends_before_it_starts_is_empty() {
    assert_eq!(
        BaselineYears::new(2010, 2009),
        Err(QueryError::EmptyBaseline)
    );
}

#[test]
fn the_default_baseline_is_every_complete_year_before_the_current_one() {
    assert_eq!(BaselineYears::before(2026), BaselineYears::new(2001, 2025));
}

#[test]
fn there_is_no_default_baseline_before_the_second_year() {
    assert_eq!(BaselineYears::before(2001), Err(QueryError::EmptyBaseline));
    assert_eq!(BaselineYears::before(0), Err(QueryError::EmptyBaseline));
}
