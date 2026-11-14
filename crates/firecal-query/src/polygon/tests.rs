use super::{PolygonArea, Vertex};
use crate::{Area, QueryError, Rectangle, SeriesOut, series};
use firecal_testkit::SeededRandom;
use firecal_types::{MonthIndex, Plane, TILE_CELLS, TileId, TileView, tile_len, write_tile};

fn corner(latitude: f64, longitude: f64) -> Vertex {
    Vertex {
        latitude,
        longitude,
    }
}

fn corners(points: &[(f64, f64)]) -> Vec<Vertex> {
    points
        .iter()
        .map(|(latitude, longitude)| corner(latitude.to_owned(), longitude.to_owned()))
        .collect()
}

fn selected(points: &[(f64, f64)]) -> Result<Vec<(u32, u32)>, QueryError> {
    let vertices = corners(points);
    let mut scratch = vec![corner(0.0, 0.0); vertices.len()];
    let area = PolygonArea::new(&vertices, &mut scratch)?;
    let mut cells = Vec::new();
    for row in area.rows() {
        for col in area.columns() {
            if area.selects(row, col) {
                cells.push((row, col));
            }
        }
    }
    Ok(cells)
}

// Row r has its center at latitude 90 - 0.25 (r + 0.5) and column c at longitude -180 + 0.25 (c + 0.5).
// So latitude 10.0 to 10.25 is row 319 and longitude 0.0 to 0.25 is column 720.

#[test]
fn a_square_with_corners_on_cell_edges_selects_the_four_cells_inside() {
    let square = [(10.0, 0.0), (10.0, 0.5), (10.5, 0.5), (10.5, 0.0)];
    assert_eq!(
        selected(&square).unwrap(),
        [(318, 720), (318, 721), (319, 720), (319, 721)]
    );
}

#[test]
fn clockwise_and_counter_clockwise_windings_select_the_same_cells() {
    let counter = [(10.0, 0.0), (10.0, 0.5), (10.5, 0.5), (10.5, 0.0)];
    let mut clockwise = counter;
    clockwise.reverse();
    assert_eq!(selected(&counter), selected(&clockwise));
}

#[test]
fn a_closing_corner_equal_to_the_first_changes_nothing() {
    let open = [(10.0, 0.0), (10.0, 0.5), (10.5, 0.5), (10.5, 0.0)];
    let closed = [
        (10.0, 0.0),
        (10.0, 0.5),
        (10.5, 0.5),
        (10.5, 0.0),
        (10.0, 0.0),
    ];
    assert_eq!(selected(&open), selected(&closed));
}

#[test]
fn a_triangle_selects_the_cells_whose_centers_it_contains() {
    // Corners (10, 0), (10, 1), (10.9, 0). With a and b the center's degrees north of 10 and east of 0, a
    // center is inside when a < 0.9 (1 - b): six of the sixteen centers.
    let triangle = [(10.0, 0.0), (10.0, 1.0), (10.9, 0.0)];
    assert_eq!(
        selected(&triangle).unwrap(),
        [
            (317, 720),
            (318, 720),
            (318, 721),
            (319, 720),
            (319, 721),
            (319, 722)
        ]
    );
}

#[test]
fn a_polygon_across_the_antimeridian_is_as_wide_as_it_looks() {
    let across = [(0.0, 179.5), (0.0, -179.5), (0.5, -179.5), (0.5, 179.5)];
    let cells = selected(&across).unwrap();
    assert_eq!(cells.len(), 8);
    for row in [358, 359] {
        for col in [0, 1, 1438, 1439] {
            assert!(cells.contains(&(row, col)), "{row}, {col}");
        }
    }
}

#[test]
fn a_polygon_with_a_corner_on_the_north_pole_is_fine() {
    // Half the width at latitude L is 2 (90 - L): 0.25 on the top row, 0.75 on the next.
    let cap = [(90.0, 0.0), (89.5, -1.0), (89.5, 1.0)];
    let cells = selected(&cap).unwrap();
    let top: Vec<u32> = cells
        .iter()
        .filter(|(row, _)| *row == 0)
        .map(|(_, col)| *col)
        .collect();
    let next: Vec<u32> = cells
        .iter()
        .filter(|(row, _)| *row == 1)
        .map(|(_, col)| *col)
        .collect();
    assert_eq!(top, [719, 720]);
    assert_eq!(next, [717, 718, 719, 720, 721, 722]);
}

#[test]
fn a_polygon_with_a_corner_on_the_south_pole_is_fine() {
    let cap = [(-90.0, 0.0), (-89.5, -1.0), (-89.5, 1.0)];
    let cells = selected(&cap).unwrap();
    assert_eq!(cells.len(), 8);
    assert!(cells.iter().all(|(row, _)| *row >= 718));
}

#[test]
fn a_ring_around_the_pole_is_refused() {
    let ring = [(89.0, -180.0), (89.0, -90.0), (89.0, 0.0), (89.0, 90.0)];
    assert_eq!(selected(&ring), Err(QueryError::EnclosesPole));
}

#[test]
fn a_bow_tie_is_self_intersecting() {
    let bow_tie = [(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)];
    assert_eq!(selected(&bow_tie), Err(QueryError::SelfIntersecting));
}

#[test]
fn a_corner_touching_another_edge_is_self_intersecting() {
    // The last corner (0, 1) lies on the first edge from (0, 0) to (0, 2).
    let touching = [(0.0, 0.0), (0.0, 2.0), (2.0, 2.0), (2.0, 1.0), (0.0, 1.0)];
    assert_eq!(selected(&touching), Err(QueryError::SelfIntersecting));
}

#[test]
fn fewer_than_three_corners_or_no_area_is_empty() {
    assert_eq!(
        selected(&[(0.0, 0.0), (1.0, 1.0)]),
        Err(QueryError::EmptyArea)
    );
    assert_eq!(selected(&[]), Err(QueryError::EmptyArea));
    assert_eq!(
        selected(&[(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]),
        Err(QueryError::EmptyArea)
    );
}

#[test]
fn a_corner_off_the_globe_is_rejected() {
    let square = |latitude: f64, longitude: f64| [(0.0, 0.0), (0.0, 1.0), (latitude, longitude)];
    assert_eq!(
        selected(&square(91.0, 0.5)),
        Err(QueryError::LatitudeOutOfRange)
    );
    assert_eq!(
        selected(&square(1.0, 181.0)),
        Err(QueryError::LongitudeOutOfRange)
    );
    assert_eq!(
        selected(&square(f64::NAN, 0.5)),
        Err(QueryError::LatitudeOutOfRange)
    );
}

#[test]
fn a_scratch_buffer_that_is_too_short_is_an_error() {
    let vertices = corners(&[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0)]);
    let mut scratch = [corner(0.0, 0.0); 2];
    assert_eq!(
        PolygonArea::new(&vertices, &mut scratch).map(|_| ()),
        Err(QueryError::OutputLengthMismatch)
    );
}

fn tile_with_two_cells() -> Vec<u8> {
    // Position (39, 0) and (39, 1) of tile (7, 18) are grid row 319, columns 720 and 721.
    let mut count = vec![0_u16; TILE_CELLS];
    count[39 * 40] = 5;
    count[39 * 40 + 1] = 3;
    let zeros = vec![0_u16; TILE_CELLS];
    let mut bytes = vec![0; tile_len(1)];
    write_tile(
        MonthIndex::from_index(10),
        1,
        [&count, &zeros, &zeros],
        &mut bytes,
    )
    .unwrap();
    bytes
}

#[test]
fn a_series_over_a_polygon_sums_only_the_cells_it_selects() {
    let bytes = tile_with_two_cells();
    let tiles = [(
        TileId::new(7, 18).unwrap(),
        TileView::parse(&bytes).unwrap(),
    )];
    let sum = |points: &[(f64, f64)]| {
        let vertices = corners(points);
        let mut scratch = vec![corner(0.0, 0.0); vertices.len()];
        let area = PolygonArea::new(&vertices, &mut scratch).unwrap();
        let mut value = [-1.0];
        let out = SeriesOut {
            value: &mut value,
            sigma: None,
        };
        series(&tiles, &area, MonthIndex::from_index(10), Plane::Count, out).unwrap();
        value[0]
    };
    assert_eq!(
        sum(&[(10.0, 0.0), (10.0, 0.5), (10.25, 0.5), (10.25, 0.0)]),
        8.0
    );
    assert_eq!(
        sum(&[(10.0, 0.0), (10.0, 0.25), (10.25, 0.25), (10.25, 0.0)]),
        5.0
    );
}

#[test]
fn a_polygon_drawn_as_a_rectangle_selects_what_the_rectangle_does() {
    const SEED: u64 = 0x9019;
    let mut random = SeededRandom::new(SEED);
    let mut quarter =
        |bound: u64| 0.25 * f64::from(u32::try_from(random.next_below(bound)).unwrap());
    for case in 0..200 {
        let (south, west) = (quarter(40), quarter(40) - 20.0);
        let (north, east) = (south + 0.25 + quarter(20), west + 0.25 + quarter(20));
        let rectangle = Rectangle::new(south, north, west, east).unwrap();
        let polygon =
            selected(&[(south, west), (south, east), (north, east), (north, west)]).unwrap();
        let by_rectangle: Vec<(u32, u32)> = rectangle
            .rows()
            .flat_map(|row| rectangle.columns().map(move |col| (row, col)))
            .collect();
        assert_eq!(polygon, by_rectangle, "seed {SEED:#x}, case {case}");
    }
}
