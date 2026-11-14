use super::{Analog, History, MOST_YEARS, Population, Season, analog_years};
use crate::{BaselineYears, QueryError};
use firecal_testkit::SeededRandom;
use firecal_types::MonthIndex;

fn january_2001() -> MonthIndex {
    MonthIndex::new(2001, 1).unwrap()
}

fn nobody() -> Analog {
    Analog {
        year: 0,
        distance: -1.0,
    }
}

/// Monthly series from January 2001 where `years` lists, per year from 2001, its January and February.
fn series(years: &[[f64; 2]]) -> Vec<f64> {
    years
        .iter()
        .flat_map(|[january, february]| {
            let mut year = vec![0.0; 12];
            year[0] = *january;
            year[1] = *february;
            year
        })
        .collect()
}

// Fixture, by hand. Season: January and February of 2005; candidates 2001 to 2004.
//             Jan   Feb
//   2001       10     5
//   2002       50     1
//   2003       30     9
//   2004       20     3
//   2005       25     4      the season in progress
// Ranks among the five years, lowest is 0:
//   January: 2001 -> 0, 2004 -> 1, 2005 -> 2, 2003 -> 3, 2002 -> 4
//   February: 2002 -> 0, 2004 -> 1, 2005 -> 2, 2001 -> 3, 2003 -> 4
// Distance to 2005 (rank 2 and 2): 2001: |0-2| + |3-2| = 3; 2002: |4-2| + |0-2| = 4;
//   2003: |3-2| + |4-2| = 3; 2004: |1-2| + |1-2| = 2.
fn fixture() -> Vec<f64> {
    let mut value = series(&[
        [10.0, 5.0],
        [50.0, 1.0],
        [30.0, 9.0],
        [20.0, 3.0],
        [25.0, 4.0],
    ]);
    value.truncate(4 * 12 + 2);
    value
}

fn years(first: u16, last: u16) -> BaselineYears {
    BaselineYears::new(first, last).unwrap()
}

fn season_2005(months: u16) -> Season {
    Season::new(2005, months).unwrap()
}

#[test]
fn the_nearest_year_comes_first_and_ties_come_oldest_first() {
    let mut out = [nobody(); 4];
    let written = analog_years(
        january_2001(),
        &fixture(),
        years(2001, 2004),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 4);
    let order: Vec<(u16, f64)> = out
        .iter()
        .map(|analog| (analog.year, analog.distance))
        .collect();
    assert_eq!(order, [(2004, 2.0), (2001, 3.0), (2003, 3.0), (2002, 4.0)]);
}

#[test]
fn only_as_many_as_the_output_holds_are_written() {
    let mut out = [nobody(); 2];
    let written = analog_years(
        january_2001(),
        &fixture(),
        years(2001, 2004),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 2);
    assert_eq!([out[0].year, out[1].year], [2004, 2001]);
}

#[test]
fn an_output_larger_than_the_candidates_leaves_the_rest_alone() {
    let mut out = [nobody(); 6];
    let written = analog_years(
        january_2001(),
        &fixture(),
        years(2001, 2004),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 4);
    assert_eq!(out[4], nobody());
}

#[test]
fn a_one_month_season_ranks_by_that_month_alone() {
    // January only: ranks 2001 -> 0, 2004 -> 1, 2005 -> 2, 2003 -> 3, 2002 -> 4; distances 2, 1, 1, 2
    let mut out = [nobody(); 4];
    analog_years(
        january_2001(),
        &fixture(),
        years(2001, 2004),
        season_2005(1),
        &mut out,
    )
    .unwrap();
    let order: Vec<(u16, f64)> = out
        .iter()
        .map(|analog| (analog.year, analog.distance))
        .collect();
    assert_eq!(order, [(2003, 1.0), (2004, 1.0), (2001, 2.0), (2002, 2.0)]);
}

#[test]
fn a_baseline_that_leaves_years_out_ranks_only_the_rest() {
    // Without 2001 and 2002 the Januaries are 2004 -> 0, 2005 -> 1, 2003 -> 2 and Februaries 2004 -> 0,
    // 2005 -> 1, 2003 -> 2: both candidates are one rank from 2005 in both months.
    let mut out = [nobody(); 2];
    analog_years(
        january_2001(),
        &fixture(),
        years(2003, 2004),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!([out[0].distance, out[1].distance], [2.0, 2.0]);
    assert_eq!([out[0].year, out[1].year], [2003, 2004]);
}

#[test]
fn tied_values_share_the_middle_rank() {
    // Three equal Januaries: each has middle rank 1, so every distance is 0.
    let value = series(&[[10.0, 0.0], [10.0, 0.0], [10.0, 0.0]]);
    let mut out = [nobody(); 2];
    let written = analog_years(
        january_2001(),
        &value,
        years(2001, 2002),
        Season::new(2003, 1).unwrap(),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 2);
    assert_eq!([out[0].distance, out[1].distance], [0.0, 0.0]);
}

#[test]
fn a_tie_with_the_season_is_half_a_rank_from_its_neighbours() {
    // Januaries 10, 20, 20 (the season): the two 20s share the middle of ranks 1 and 2, which is 1.5, so 2001
    // (rank 0) is 1.5 from the season and 2002, tied with it, is 0 from it.
    let value = series(&[[10.0, 0.0], [20.0, 0.0], [20.0, 0.0]]);
    let mut out = [nobody(); 2];
    analog_years(
        january_2001(),
        &value,
        years(2001, 2002),
        Season::new(2003, 1).unwrap(),
        &mut out,
    )
    .unwrap();
    assert_eq!(
        [
            (out[0].year, out[0].distance),
            (out[1].year, out[1].distance)
        ],
        [(2002, 0.0), (2001, 1.5)]
    );
}

#[test]
fn a_year_missing_a_month_of_the_season_is_not_a_candidate() {
    // The series starts in March 2001, so 2001 has neither January nor February and is left out. The Januaries
    // are 2004 -> 0, 2005 -> 1, 2003 -> 2, 2002 -> 3 and the Februaries 2002 -> 0, 2004 -> 1, 2005 -> 2,
    // 2003 -> 3; the season is at rank 1 and 2: distances 2002: 2 + 2 = 4, 2003: 1 + 1 = 2, 2004: 1 + 1 = 2.
    let mut value = vec![0.0; 10];
    value.extend(series(&[
        [50.0, 1.0],
        [30.0, 9.0],
        [20.0, 3.0],
        [25.0, 4.0],
    ]));
    value.truncate(10 + 3 * 12 + 2);
    let mut out = [nobody(); 4];
    let march_2001 = MonthIndex::new(2001, 3).unwrap();
    let written = analog_years(
        march_2001,
        &value,
        years(2001, 2004),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 3);
    let order: Vec<(u16, f64)> = out[..3]
        .iter()
        .map(|analog| (analog.year, analog.distance))
        .collect();
    assert_eq!(order, [(2003, 2.0), (2004, 2.0), (2002, 4.0)]);
}

#[test]
fn a_season_in_progress_missing_a_month_is_an_error() {
    let mut value = fixture();
    value.pop();
    let mut out = [nobody(); 4];
    assert_eq!(
        analog_years(
            january_2001(),
            &value,
            years(2001, 2004),
            season_2005(2),
            &mut out
        ),
        Err(QueryError::SeasonIncomplete)
    );
}

#[test]
fn the_season_year_is_never_its_own_analog() {
    let mut out = [nobody(); 8];
    let written = analog_years(
        january_2001(),
        &fixture(),
        years(2001, 2005),
        season_2005(2),
        &mut out,
    )
    .unwrap();
    assert_eq!(written, 4);
    assert!(out[..4].iter().all(|analog| analog.year != 2005));
}

#[test]
fn a_season_of_zero_or_thirteen_months_cannot_be_built() {
    assert_eq!(
        Season::new(2005, 0),
        Err(QueryError::SeasonMonthsOutOfRange)
    );
    assert_eq!(
        Season::new(2005, 13),
        Err(QueryError::SeasonMonthsOutOfRange)
    );
    assert!(Season::new(2005, 12).is_ok());
}

#[test]
fn more_than_sixty_four_candidate_years_are_too_many() {
    let value = vec![1.0; 12 * 70];
    let mut out = [nobody(); 4];
    assert_eq!(
        analog_years(
            january_2001(),
            &value,
            years(2001, 2068),
            Season::new(2070, 1).unwrap(),
            &mut out
        ),
        Err(QueryError::BaselineTooLong)
    );
    assert_eq!(MOST_YEARS, 64);
}

fn random_population(random: &mut SeededRandom) -> (Vec<f64>, Population, u16) {
    let year_count = 2 + usize::try_from(random.next_below(12)).unwrap();
    let value: Vec<f64> = (0..year_count * 12)
        .map(|_| f64::from(u32::try_from(random.next_below(20)).unwrap()))
        .collect();
    let mut population = Population {
        years: [0; MOST_YEARS + 1],
        count: year_count,
    };
    for (at, slot) in population.years.iter_mut().take(year_count).enumerate() {
        *slot = 2001 + u16::try_from(at).unwrap();
    }
    let months = 1 + u16::try_from(random.next_below(12)).unwrap();
    (value, population, months)
}

#[test]
fn a_years_distance_to_itself_is_zero_and_the_distance_is_symmetric() {
    const SEED: u64 = 0xA9A1;
    let mut random = SeededRandom::new(SEED);
    for case in 0..300 {
        let (value, population, months) = random_population(&mut random);
        let history = History {
            first_month: january_2001(),
            value: &value,
        };
        let years = population.years().to_vec();
        for a in &years {
            assert_eq!(
                population.double_distance(&history, months, *a, *a),
                0,
                "seed {SEED:#x}, case {case}"
            );
            for b in &years {
                assert_eq!(
                    population.double_distance(&history, months, *a, *b),
                    population.double_distance(&history, months, *b, *a),
                    "seed {SEED:#x}, case {case}"
                );
            }
        }
    }
}

#[test]
fn the_ranks_of_a_month_add_up_to_what_ranks_must_add_up_to() {
    // With n years the middle ranks of any month sum to n (n - 1) / 2, so twice that is n (n - 1).
    const SEED: u64 = 0x4A4C;
    let mut random = SeededRandom::new(SEED);
    for case in 0..300 {
        let (value, population, months) = random_population(&mut random);
        let history = History {
            first_month: january_2001(),
            value: &value,
        };
        let n = u32::try_from(population.count).unwrap();
        let total: u32 = population
            .years()
            .iter()
            .map(|year| population.double_rank(&history, *year, months))
            .sum();
        assert_eq!(total, n * (n - 1), "seed {SEED:#x}, case {case}");
    }
}
