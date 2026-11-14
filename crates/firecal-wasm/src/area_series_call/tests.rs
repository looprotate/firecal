use super::run_area_series;
use crate::history::tests::{Call, doubles};
use crate::{INVALID, STATUS_EMPTY_AREA, STATUS_OK, STATUS_OUTPUT_TOO_SMALL};

#[test]
fn the_values_come_first_and_then_the_sigmas() {
    let (status, buffer) =
        Call::fixture().run(36 * 16, |m, r, l, b, n| run_area_series(m, r, l, b, n));
    assert_eq!(status, STATUS_OK);
    let numbers = doubles(&buffer);
    assert_eq!(numbers[..3], [10.0, 20.0, 100.0]);
    assert_eq!(numbers[36..39], [2.0, 2.0, 2.0]);
}

#[test]
fn a_polygon_gives_the_same_series_as_the_rectangle_of_its_cell() {
    let west_cell = [[10.0, 0.0], [10.25, 0.0], [10.25, 0.25], [10.0, 0.25]];
    let by_polygon =
        Call::polygon(&west_cell).run(36 * 16, |m, r, l, b, n| run_area_series(m, r, l, b, n));
    assert_eq!(
        by_polygon,
        Call::fixture().run(36 * 16, |m, r, l, b, n| run_area_series(m, r, l, b, n))
    );
}

#[test]
fn a_buffer_that_is_too_small_is_reported_and_left_alone() {
    let (status, buffer) =
        Call::fixture().run(36 * 16 - 1, |m, r, l, b, n| run_area_series(m, r, l, b, n));
    assert_eq!(status, STATUS_OUTPUT_TOO_SMALL);
    assert!(buffer.iter().all(|byte| *byte == 0xAA));
}

#[test]
fn a_bad_area_or_a_bad_buffer_is_reported() {
    let empty = Call {
        area: [10.0, 10.0, 0.0, 1.0],
        ..Call::fixture()
    };
    assert_eq!(
        empty
            .run(0, |m, r, l, b, n| run_area_series(m, r, l, b, n))
            .0,
        STATUS_EMPTY_AREA
    );
    let (mut image, request, _) = Call::fixture().image(0);
    let status = run_area_series(
        &mut crate::SliceMemory::new(&mut image),
        request,
        80,
        0,
        1000,
    );
    assert_eq!(status, INVALID);
}
