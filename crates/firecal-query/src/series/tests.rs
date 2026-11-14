use super::{SeriesOut, series};
use crate::{QueryError, Rectangle};
use firecal_testkit::SeededRandom;
use firecal_types::{
    Error, MonthIndex, Plane, TILE_CELLS, TILE_SIDE, TileId, TileView, tile_len, write_tile,
};

// Fixture, worked out by hand. Grid cell row 319, column 720 (latitude 10.0 to 10.25, longitude 0.0 to 0.25)
// is tile (7, 18) at position (39, 0); its east neighbour (39, 1) is column 721; the cell south of it is
// tile (8, 18) at position (0, 0). Two months, the first is November 2000 (index 10).
//
//                  count            sigma            energy
//                  m0  m1           m0  m1           m0  m1
//  tile (7,18) (39,0)   5   7        3  12           40  50
//  tile (7,18) (39,1)   3   0        4   5            2   0
//  tile (8,18) (0,0)   10   1        4   9           30   0

fn month(index: u16) -> MonthIndex {
    MonthIndex::from_index(index)
}

struct Planes {
    count: Vec<u16>,
    sigma: Vec<u16>,
    energy: Vec<u16>,
}

fn planes_with(months: usize, cells: &[(usize, [u16; 3], [u16; 3])]) -> Planes {
    // Each cell: position in tile, then [count, sigma, energy] for month 0 and for month 1.
    let mut planes = Planes {
        count: vec![0; months * TILE_CELLS],
        sigma: vec![0; months * TILE_CELLS],
        energy: vec![0; months * TILE_CELLS],
    };
    for (position, first, second) in cells {
        for (at, values) in [(*position, first), (TILE_CELLS + *position, second)] {
            if at < months * TILE_CELLS {
                planes.count[at] = values[0];
                planes.sigma[at] = values[1];
                planes.energy[at] = values[2];
            }
        }
    }
    planes
}

fn encode(first: u16, months: u16, planes: &Planes) -> Vec<u8> {
    let mut bytes = vec![0; tile_len(months)];
    write_tile(
        month(first),
        months,
        [&planes.count, &planes.sigma, &planes.energy],
        &mut bytes,
    )
    .unwrap();
    bytes
}

fn tile_north() -> Vec<u8> {
    // position (39, 0) = 39 * 40, position (39, 1) = 39 * 40 + 1
    let planes = planes_with(
        2,
        &[
            (39 * 40, [5, 3, 40], [7, 12, 50]),
            (39 * 40 + 1, [3, 4, 2], [0, 5, 0]),
        ],
    );
    encode(10, 2, &planes)
}

fn tile_south() -> Vec<u8> {
    encode(10, 2, &planes_with(2, &[(0, [10, 4, 30], [1, 9, 0])]))
}

fn id(row: u32, col: u32) -> TileId {
    TileId::new(row, col).unwrap()
}

fn run(
    tiles: &[(TileId, TileView<'_>)],
    area: &Rectangle,
    plane: Plane,
    months: usize,
    with_sigma: bool,
) -> Result<(Vec<f64>, Vec<f64>), QueryError> {
    let mut value = vec![-1.0; months];
    let mut sigma = vec![-1.0; months];
    let out = SeriesOut {
        value: &mut value,
        sigma: with_sigma.then_some(sigma.as_mut_slice()),
    };
    series(tiles, area, month(10), plane, out)?;
    Ok((value, sigma))
}

fn rectangle(south: f64, north: f64, west: f64, east: f64) -> Rectangle {
    Rectangle::new(south, north, west, east).unwrap()
}

#[test]
fn one_cell_gives_its_count_and_sigma_each_month() {
    let north = tile_north();
    let tiles = [(id(7, 18), TileView::parse(&north).unwrap())];
    let (value, sigma) = run(
        &tiles,
        &rectangle(10.0, 10.25, 0.0, 0.25),
        Plane::Count,
        2,
        true,
    )
    .unwrap();
    assert_eq!(value, [5.0, 7.0]);
    assert_eq!(sigma, [3.0, 12.0]);
}

#[test]
fn two_cells_add_counts_and_add_sigmas_in_quadrature() {
    let north = tile_north();
    let tiles = [(id(7, 18), TileView::parse(&north).unwrap())];
    let (value, sigma) = run(
        &tiles,
        &rectangle(10.1, 10.2, 0.1, 0.3),
        Plane::Count,
        2,
        true,
    )
    .unwrap();
    assert_eq!(value, [8.0, 7.0]);
    // month 0: sqrt(3^2 + 4^2) = 5; month 1: sqrt(12^2 + 5^2) = 13
    assert_eq!(sigma, [5.0, 13.0]);
}

#[test]
fn the_energy_plane_sums_the_energy_values() {
    let north = tile_north();
    let tiles = [(id(7, 18), TileView::parse(&north).unwrap())];
    let (value, _) = run(
        &tiles,
        &rectangle(10.1, 10.2, 0.1, 0.3),
        Plane::Energy,
        2,
        false,
    )
    .unwrap();
    assert_eq!(value, [42.0, 50.0]);
}

#[test]
fn an_area_across_two_tiles_adds_both() {
    let (north, south) = (tile_north(), tile_south());
    let tiles = [
        (id(8, 18), TileView::parse(&south).unwrap()),
        (id(7, 18), TileView::parse(&north).unwrap()),
    ];
    let (value, sigma) = run(
        &tiles,
        &rectangle(9.9, 10.1, 0.1, 0.2),
        Plane::Count,
        2,
        true,
    )
    .unwrap();
    // cells (39,0) of tile (7,18) and (0,0) of tile (8,18): 5 + 10 and 7 + 1
    assert_eq!(value, [15.0, 8.0]);
    // month 0: sqrt(3^2 + 4^2) = 5; month 1: sqrt(12^2 + 9^2) = 15
    assert_eq!(sigma, [5.0, 15.0]);
}

#[test]
fn the_order_of_the_tiles_does_not_change_the_answer() {
    let (north, south) = (tile_north(), tile_south());
    let a = [
        (id(7, 18), TileView::parse(&north).unwrap()),
        (id(8, 18), TileView::parse(&south).unwrap()),
    ];
    let b = [a[1], a[0]];
    let area = rectangle(9.9, 10.1, 0.1, 0.2);
    assert_eq!(
        run(&a, &area, Plane::Count, 2, true),
        run(&b, &area, Plane::Count, 2, true)
    );
}

#[test]
fn a_missing_tile_is_zeros_not_an_error() {
    let (value, sigma) = run(
        &[],
        &rectangle(10.0, 10.25, 0.0, 0.25),
        Plane::Count,
        3,
        true,
    )
    .unwrap();
    assert_eq!((value, sigma), (vec![0.0; 3], vec![0.0; 3]));
}

#[test]
fn the_whole_world_with_no_tiles_is_zeros() {
    let world = rectangle(-90.0, 90.0, -180.0, 180.0);
    let (value, _) = run(&[], &world, Plane::Count, 1, true).unwrap();
    assert_eq!(value, [0.0]);
}

#[test]
fn the_whole_world_sums_every_tile_present() {
    let (north, south) = (tile_north(), tile_south());
    let tiles = [
        (id(7, 18), TileView::parse(&north).unwrap()),
        (id(8, 18), TileView::parse(&south).unwrap()),
    ];
    let world = rectangle(-90.0, 90.0, -180.0, 180.0);
    let (value, sigma) = run(&tiles, &world, Plane::Count, 2, true).unwrap();
    // month 0: 5 + 3 + 10; month 1: 7 + 0 + 1; sigma month 0: sqrt(9 + 16 + 16) = sqrt(41)
    assert_eq!(value, [18.0, 8.0]);
    assert!(
        (sigma[0] - 41.0_f64.sqrt()).abs() < 1e-12,
        "sqrt(41) is exact to rounding"
    );
}

#[test]
fn counts_of_65535_add_without_wrapping() {
    let planes = planes_with(
        1,
        &[
            (0, [65_535, 65_535, 0], [0; 3]),
            (1, [65_535, 65_535, 0], [0; 3]),
        ],
    );
    let bytes = encode(10, 1, &planes);
    let tiles = [(id(0, 0), TileView::parse(&bytes).unwrap())];
    // tile (0,0) positions (0,0) and (0,1) are cells row 0, columns 0 and 1
    let (value, sigma) = run(
        &tiles,
        &rectangle(89.75, 90.0, -180.0, -179.5),
        Plane::Count,
        1,
        true,
    )
    .unwrap();
    assert_eq!(value, [131_070.0]);
    // sqrt(2 * 65535^2) = 65535 * sqrt(2): equal to sqrt of the exact sum
    let expected = (2.0 * 65_535.0 * 65_535.0_f64).sqrt();
    assert!((sigma[0] - expected).abs() < 1e-9, "sigma {}", sigma[0]);
}

#[test]
fn a_month_the_tile_lacks_is_an_error() {
    let north = tile_north();
    let tiles = [(id(7, 18), TileView::parse(&north).unwrap())];
    let error = run(
        &tiles,
        &rectangle(10.0, 10.25, 0.0, 0.25),
        Plane::Count,
        3,
        false,
    )
    .unwrap_err();
    assert_eq!(error, QueryError::Types(Error::MonthOutsideTile));
}

#[test]
fn no_months_is_an_error() {
    let error = run(
        &[],
        &rectangle(10.0, 10.25, 0.0, 0.25),
        Plane::Count,
        0,
        false,
    )
    .unwrap_err();
    assert_eq!(error, QueryError::Types(Error::EmptyMonthRange));
}

#[test]
fn months_running_past_the_index_are_an_error() {
    let mut value = [0.0; 2];
    let out = SeriesOut {
        value: &mut value,
        sigma: None,
    };
    let error = series(
        &[],
        &rectangle(0.0, 1.0, 0.0, 1.0),
        month(u16::MAX),
        Plane::Count,
        out,
    )
    .unwrap_err();
    assert_eq!(error, QueryError::Types(Error::MonthRangeTooLong));
}

#[test]
fn sigma_and_value_buffers_of_different_length_are_an_error() {
    let mut value = [0.0; 2];
    let mut sigma = [0.0; 3];
    let out = SeriesOut {
        value: &mut value,
        sigma: Some(&mut sigma),
    };
    let error = series(
        &[],
        &rectangle(0.0, 1.0, 0.0, 1.0),
        month(10),
        Plane::Count,
        out,
    )
    .unwrap_err();
    assert_eq!(error, QueryError::OutputLengthMismatch);
}

#[test]
fn old_contents_of_the_output_are_overwritten() {
    let north = tile_north();
    let tiles = [(id(7, 18), TileView::parse(&north).unwrap())];
    let (value, _) = run(
        &tiles,
        &rectangle(10.0, 10.25, 0.0, 0.25),
        Plane::Count,
        2,
        false,
    )
    .unwrap();
    assert_eq!(value, [5.0, 7.0], "the buffer started at -1");
}

#[test]
fn an_area_that_contains_another_has_at_least_its_total() {
    const SEED: u64 = 0xA2EA;
    let mut random = SeededRandom::new(SEED);
    let count: Vec<u16> = (0..TILE_CELLS)
        .map(|_| u16::try_from(random.next_below(1000)).unwrap())
        .collect();
    let zeros = vec![0_u16; TILE_CELLS];
    let bytes = encode(
        10,
        1,
        &Planes {
            count,
            sigma: zeros.clone(),
            energy: zeros,
        },
    );
    let tiles = [(id(0, 0), TileView::parse(&bytes).unwrap())];
    // A block of cells of tile (0,0), as grid rows and columns [from, to), drawn as degrees.
    let area = |rows: (u32, u32), columns: (u32, u32)| {
        let degrees = |cells: u32| 0.25 * f64::from(cells);
        rectangle(
            90.0 - degrees(rows.1),
            90.0 - degrees(rows.0),
            -180.0 + degrees(columns.0),
            -180.0 + degrees(columns.1),
        )
    };
    let total = |area: &Rectangle| run(&tiles, area, Plane::Count, 1, false).unwrap().0[0];
    let mut draw = |bound: u32| u32::try_from(random.next_below(u64::from(bound))).unwrap();
    for case in 0..300 {
        let (row, col) = (draw(39), draw(39));
        let (row_end, col_end) = (
            row + 1 + draw(40 - row - 1 + 1),
            col + 1 + draw(40 - col - 1 + 1),
        );
        let inner = area((row, row_end), (col, col_end));
        let outer = area(
            (row - draw(row + 1), row_end + draw(41 - row_end)),
            (col - draw(col + 1), col_end + draw(41 - col_end)),
        );
        assert!(
            total(&outer) >= total(&inner),
            "seed {SEED:#x}, case {case}"
        );
    }
}

#[test]
fn the_series_sum_equals_the_sum_of_the_cells_it_covers() {
    let count: Vec<u16> = (0..TILE_CELLS)
        .map(|cell| u16::try_from(cell % 7).unwrap())
        .collect();
    let zeros = vec![0_u16; TILE_CELLS];
    let bytes = encode(
        10,
        1,
        &Planes {
            count: count.clone(),
            sigma: zeros.clone(),
            energy: zeros,
        },
    );
    let tiles = [(id(0, 0), TileView::parse(&bytes).unwrap())];
    // Rows 0..3 and columns 0..2 of the grid: latitude 89.25 to 90, longitude -180 to -179.5.
    let by_hand: u32 = [0_usize, 1, 40, 41, 80, 81]
        .iter()
        .map(|at| u32::from(count[*at]))
        .sum();
    let (value, _) = run(
        &tiles,
        &rectangle(89.25, 90.0, -180.0, -179.5),
        Plane::Count,
        1,
        false,
    )
    .unwrap();
    assert_eq!(value, [f64::from(by_hand)]);
    assert_eq!(TILE_SIDE, 40);
}
