use super::run_analogs;
use crate::history::tests::{Call, doubles};
use crate::{STATUS_BASELINE_INVALID, STATUS_OK, STATUS_OUTPUT_TOO_SMALL, STATUS_SEASON_INVALID};

// Season: January and February 2003, against 2001 and 2002. January ranks, lowest first: 2003 (5), 2001 (10),
// 2002 (30). February: 2001 (20), 2003 (22), 2002 (40). With ranks 0, 1, 2 the distance of 2003 to 2001 is
// |0 - 1| + |1 - 0| = 2 and to 2002 |0 - 2| + |1 - 2| = 3.

fn found(buffer: &[u8]) -> (u32, Vec<(u32, f64)>) {
    let count = u32::from_le_bytes(buffer[..4].try_into().unwrap());
    let analogs = buffer[4..]
        .as_chunks::<12>()
        .0
        .iter()
        .take(usize::try_from(count).unwrap())
        .map(|slot| {
            (
                u32::from_le_bytes(slot[..4].try_into().unwrap()),
                doubles(&slot[4..])[0],
            )
        })
        .collect();
    (count, analogs)
}

#[test]
fn the_nearest_years_come_first_with_their_distances() {
    let (status, buffer) =
        Call::fixture().run(4 + 3 * 12, |m, r, l, b, n| run_analogs(m, r, l, b, n));
    assert_eq!(status, STATUS_OK);
    assert_eq!(found(&buffer), (2, vec![(2001, 2.0), (2002, 3.0)]));
}

#[test]
fn the_buffer_decides_how_many_are_asked_for() {
    let (status, buffer) = Call::fixture().run(4 + 12, |m, r, l, b, n| run_analogs(m, r, l, b, n));
    assert_eq!(status, STATUS_OK);
    assert_eq!(found(&buffer), (1, vec![(2001, 2.0)]));
}

#[test]
fn a_season_the_series_does_not_hold_is_refused() {
    let call = Call {
        season: (2004, 2),
        ..Call::fixture()
    };
    assert_eq!(
        call.run(40, |m, r, l, b, n| run_analogs(m, r, l, b, n)).0,
        STATUS_SEASON_INVALID
    );
    let too_long = Call {
        season: (2003, 13),
        ..Call::fixture()
    };
    assert_eq!(
        too_long
            .run(40, |m, r, l, b, n| run_analogs(m, r, l, b, n))
            .0,
        STATUS_SEASON_INVALID
    );
    let too_wide = Call {
        season: (70_000, 2),
        ..Call::fixture()
    };
    assert_eq!(
        too_wide
            .run(40, |m, r, l, b, n| run_analogs(m, r, l, b, n))
            .0,
        STATUS_SEASON_INVALID
    );
}

#[test]
fn a_bad_baseline_or_a_buffer_without_room_for_the_count_is_reported() {
    let call = Call {
        baseline: (2002, 2001),
        ..Call::fixture()
    };
    assert_eq!(
        call.run(40, |m, r, l, b, n| run_analogs(m, r, l, b, n)).0,
        STATUS_BASELINE_INVALID
    );
    assert_eq!(
        Call::fixture()
            .run(3, |m, r, l, b, n| run_analogs(m, r, l, b, n))
            .0,
        STATUS_OUTPUT_TOO_SMALL
    );
}
