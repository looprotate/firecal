use super::write_cells_csv;
use crate::{QueryError, Rectangle};
use firecal_types::{MonthIndex, Plane, TILE_CELLS, TileId, TileView, tile_len, write_tile};

// Fixture, by hand: tile (7, 18), two months from index 10 (November 2000). Position (39, 0) is grid row 319,
// column 720 (center 10.125 north, 0.125 east); position (39, 1) is column 721 (center 0.375 east).
//                 count     sigma    energy
//                 m0  m1    m0  m1   m0  m1
//   (39, 0)        5   7     3  12   40   0
//   (39, 1)        3   0     4   5    0   0
fn tile() -> Vec<u8> {
    let mut planes = [
        vec![0_u16; 2 * TILE_CELLS],
        vec![0; 2 * TILE_CELLS],
        vec![0; 2 * TILE_CELLS],
    ];
    for (plane, at, value) in [
        (0, 39 * 40, 5),
        (0, TILE_CELLS + 39 * 40, 7),
        (0, 39 * 40 + 1, 3),
        (1, 39 * 40, 3),
        (1, TILE_CELLS + 39 * 40, 12),
        (1, 39 * 40 + 1, 4),
        (1, TILE_CELLS + 39 * 40 + 1, 5),
        (2, 39 * 40, 40),
    ] {
        planes[plane][at] = value;
    }
    let mut bytes = vec![0; tile_len(2)];
    write_tile(
        MonthIndex::from_index(10),
        2,
        [&planes[0], &planes[1], &planes[2]],
        &mut bytes,
    )
    .unwrap();
    bytes
}

fn csv(plane: Plane, with_tile: bool, months: u16) -> Result<String, QueryError> {
    let bytes = tile();
    let tiles: Vec<_> = if with_tile {
        vec![(
            TileId::new(7, 18).unwrap(),
            TileView::parse(&bytes).unwrap(),
        )]
    } else {
        Vec::new()
    };
    let area = Rectangle::new(10.0, 10.25, 0.0, 0.5).unwrap();
    let mut out = vec![0_u8; 1024];
    let written = write_cells_csv(
        &tiles,
        &area,
        MonthIndex::from_index(10),
        months,
        plane,
        &mut out,
    )?;
    Ok(String::from_utf8(out[..written].to_vec()).unwrap())
}

#[test]
fn every_cell_of_every_month_is_a_row_with_the_count_and_its_sigma() {
    assert_eq!(
        csv(Plane::Count, true, 2).unwrap(),
        "year,month,cell_lat,cell_lon,value,sigma\n\
         2000,11,10.125,0.125,5,3\n\
         2000,11,10.125,0.375,3,4\n\
         2000,12,10.125,0.125,7,12\n\
         2000,12,10.125,0.375,0,5\n"
    );
}

#[test]
fn the_energy_has_no_sigma_so_the_field_is_empty() {
    assert_eq!(
        csv(Plane::Energy, true, 1).unwrap(),
        "year,month,cell_lat,cell_lon,value,sigma\n\
         2000,11,10.125,0.125,40,\n\
         2000,11,10.125,0.375,0,\n"
    );
}

#[test]
fn cells_in_a_missing_tile_are_rows_of_zeros() {
    assert_eq!(
        csv(Plane::Count, false, 1).unwrap(),
        "year,month,cell_lat,cell_lon,value,sigma\n\
         2000,11,10.125,0.125,0,0\n\
         2000,11,10.125,0.375,0,0\n"
    );
}

#[test]
fn a_month_the_tile_lacks_is_an_error() {
    assert!(matches!(
        csv(Plane::Count, true, 3),
        Err(QueryError::Types(_))
    ));
}

#[test]
fn no_months_is_an_error() {
    assert!(matches!(
        csv(Plane::Count, true, 0),
        Err(QueryError::Types(_))
    ));
}

#[test]
fn a_buffer_one_byte_short_is_an_error_and_the_exact_size_fits() {
    let needed = csv(Plane::Count, true, 2).unwrap().len();
    let bytes = tile();
    let tiles = [(
        TileId::new(7, 18).unwrap(),
        TileView::parse(&bytes).unwrap(),
    )];
    let area = Rectangle::new(10.0, 10.25, 0.0, 0.5).unwrap();
    let first = MonthIndex::from_index(10);
    let mut exact = vec![0_u8; needed];
    assert_eq!(
        write_cells_csv(&tiles, &area, first, 2, Plane::Count, &mut exact),
        Ok(needed)
    );
    let mut short = vec![0_u8; needed - 1];
    assert_eq!(
        write_cells_csv(&tiles, &area, first, 2, Plane::Count, &mut short),
        Err(QueryError::BufferTooSmall)
    );
}

#[test]
fn the_southern_and_western_hemispheres_have_negative_centers() {
    let area = Rectangle::new(-0.25, 0.0, -0.25, 0.0).unwrap();
    let mut out = vec![0_u8; 256];
    let written = write_cells_csv(
        &[],
        &area,
        MonthIndex::from_index(10),
        1,
        Plane::Count,
        &mut out,
    )
    .unwrap();
    // Row 360 is latitude -0.25 to 0 (center -0.125); column 719 is longitude -0.25 to 0 (center -0.125).
    assert_eq!(
        std::str::from_utf8(&out[..written]).unwrap(),
        "year,month,cell_lat,cell_lon,value,sigma\n2000,11,-0.125,-0.125,0,0\n"
    );
}
