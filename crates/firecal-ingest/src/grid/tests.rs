use super::cell;
use firecal_types::CellIndex;

#[test]
fn points_just_either_side_of_an_edge_do_not_round_into_the_same_cell() {
    assert_eq!(
        cell(1e-15, -1e-15).unwrap(),
        CellIndex::new(359, 719).unwrap()
    );
    assert_eq!(
        cell(-1e-15, 1e-15).unwrap(),
        CellIndex::new(360, 720).unwrap()
    );
}

#[test]
fn poles_antimeridian_and_grid_edges_have_explicit_cells() {
    for (latitude, longitude, row, col) in [
        (90.0, -180.0, 0, 0),
        (-90.0, 180.0, 719, 1439),
        (0.0, 0.0, 360, 720),
        (89.75, -179.75, 1, 1),
        (89.875, -179.875, 0, 0),
        (-33.5, -70.25, 494, 439),
    ] {
        assert_eq!(
            cell(latitude, longitude).unwrap(),
            CellIndex::new(row, col).unwrap()
        );
    }
}

#[test]
fn every_cell_center_maps_back_to_its_row_and_column() {
    for row in 0..720 {
        for col in 0..1440 {
            let latitude_offset = (f64::from(row) + 0.5) * 0.25;
            let longitude_offset = (f64::from(col) + 0.5) * 0.25;
            let latitude = 90.0 - latitude_offset;
            let longitude = -180.0 + longitude_offset;
            assert_eq!(
                cell(latitude, longitude).unwrap(),
                CellIndex::new(row, col).unwrap()
            );
        }
    }
}

#[test]
fn invalid_or_nonfinite_coordinates_are_errors() {
    for (latitude, longitude) in [
        (90.01, 0.0),
        (-90.01, 0.0),
        (0.0, 180.01),
        (0.0, -180.01),
        (f64::NAN, 0.0),
        (0.0, f64::NAN),
        (f64::INFINITY, 0.0),
    ] {
        assert!(cell(latitude, longitude).is_err());
    }
}
