use super::Rectangle;
use crate::QueryError;

// Row r covers latitude 90 - 0.25 (r + 1) to 90 - 0.25 r; column c covers -180 + 0.25 c to -180 + 0.25 (c + 1).
// So latitude 10.0 to 10.25 is row 319 and longitude 0.0 to 0.25 is column 720.

fn rectangle(south: f64, north: f64, west: f64, east: f64) -> Rectangle {
    Rectangle::new(south, north, west, east).unwrap()
}

#[test]
fn the_whole_world_is_every_row_and_column() {
    let world = rectangle(-90.0, 90.0, -180.0, 180.0);
    assert_eq!(world.rows(), 0..720);
    assert_eq!(world.columns(), 0..1440);
}

#[test]
fn the_north_west_corner_cell_is_row_zero_column_zero() {
    let corner = rectangle(89.75, 90.0, -180.0, -179.75);
    assert_eq!((corner.rows(), corner.columns()), (0..1, 0..1));
}

#[test]
fn the_south_east_corner_cell_is_the_last_row_and_column() {
    let corner = rectangle(-90.0, -89.75, 179.75, 180.0);
    assert_eq!((corner.rows(), corner.columns()), (719..720, 1439..1440));
}

#[test]
fn a_rectangle_covering_one_cell_exactly_selects_only_that_cell() {
    let one = rectangle(10.0, 10.25, 0.0, 0.25);
    assert_eq!((one.rows(), one.columns()), (319..320, 720..721));
}

#[test]
fn a_rectangle_thinner_than_a_cell_selects_the_cell_it_is_in() {
    let thin = rectangle(10.1, 10.2, 0.1, 0.2);
    assert_eq!((thin.rows(), thin.columns()), (319..320, 720..721));
}

#[test]
fn a_rectangle_across_a_cell_edge_selects_both_cells() {
    let across = rectangle(9.9, 10.1, 0.2, 0.3);
    assert_eq!((across.rows(), across.columns()), (319..321, 720..722));
}

#[test]
fn a_rectangle_only_touching_a_cell_edge_does_not_select_it() {
    let above = rectangle(10.25, 10.5, 0.25, 0.5);
    assert_eq!((above.rows(), above.columns()), (318..319, 721..722));
}

#[test]
fn a_rectangle_touching_the_north_pole_starts_at_row_zero() {
    assert_eq!(rectangle(89.9, 90.0, 0.0, 1.0).rows(), 0..1);
}

#[test]
fn a_rectangle_touching_the_south_pole_ends_at_the_last_row() {
    assert_eq!(rectangle(-90.0, -89.9, 0.0, 1.0).rows(), 719..720);
}

#[test]
fn a_rectangle_with_no_height_or_width_is_empty() {
    assert_eq!(
        Rectangle::new(10.0, 10.0, 0.0, 1.0),
        Err(QueryError::EmptyArea)
    );
    assert_eq!(
        Rectangle::new(10.0, 11.0, 5.0, 5.0),
        Err(QueryError::EmptyArea)
    );
}

#[test]
fn a_rectangle_with_south_above_north_or_west_right_of_east_is_empty() {
    assert_eq!(
        Rectangle::new(11.0, 10.0, 0.0, 1.0),
        Err(QueryError::EmptyArea)
    );
    assert_eq!(
        Rectangle::new(10.0, 11.0, 1.0, 0.0),
        Err(QueryError::EmptyArea)
    );
}

#[test]
fn latitudes_beyond_the_poles_are_rejected() {
    assert_eq!(
        Rectangle::new(-90.5, 0.0, 0.0, 1.0),
        Err(QueryError::LatitudeOutOfRange)
    );
    assert_eq!(
        Rectangle::new(0.0, 90.5, 0.0, 1.0),
        Err(QueryError::LatitudeOutOfRange)
    );
}

#[test]
fn longitudes_beyond_the_antimeridian_are_rejected() {
    assert_eq!(
        Rectangle::new(0.0, 1.0, -180.5, 0.0),
        Err(QueryError::LongitudeOutOfRange)
    );
    assert_eq!(
        Rectangle::new(0.0, 1.0, 0.0, 190.0),
        Err(QueryError::LongitudeOutOfRange)
    );
}

#[test]
fn a_bound_that_is_not_a_number_is_rejected() {
    assert_eq!(
        Rectangle::new(f64::NAN, 1.0, 0.0, 1.0),
        Err(QueryError::LatitudeOutOfRange)
    );
    assert_eq!(
        Rectangle::new(0.0, 1.0, 0.0, f64::NAN),
        Err(QueryError::LongitudeOutOfRange)
    );
}
