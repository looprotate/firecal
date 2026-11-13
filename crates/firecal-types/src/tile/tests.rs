use super::{Plane, TILE_CELLS, TILE_MAGIC, TileId, TileView, tile_len, write_tile};
use crate::{CellIndex, Error, MonthIndex};
use firecal_testkit::SeededRandom;

fn month(index: u16) -> MonthIndex {
    MonthIndex::from_index(index)
}

fn plane_of(months: usize, seed: u64) -> Vec<u16> {
    let mut random = SeededRandom::new(seed);
    (0..months * TILE_CELLS)
        .map(|_| u16::try_from(random.next_below(65_536)).unwrap())
        .collect()
}

fn encode(first: u16, months: u16, planes: [&[u16]; 3]) -> Vec<u8> {
    let mut out = vec![0; tile_len(months)];
    let written = write_tile(month(first), months, planes, &mut out).unwrap();
    assert_eq!(written, out.len());
    out
}

#[test]
fn the_header_has_the_bytes_written_by_hand() {
    let zeros = vec![0_u16; TILE_CELLS];
    let bytes = encode(10, 1, [&zeros, &zeros, &zeros]);
    let header: [u8; 16] = [
        0x46, 0x43, 0x54, 0x31, // FCT1
        0x01, 0x00, // one month
        0x0a, 0x00, // first month 10
        0x03, // three planes
        0, 0, 0, 0, 0, 0, 0, // reserved
    ];
    assert_eq!(&bytes[..16], &header);
    assert_eq!(&bytes[..4], &TILE_MAGIC);
    assert_eq!(bytes.len(), 16 + 3 * 1600 * 2);
}

#[test]
fn the_size_is_sixteen_plus_three_planes_of_two_bytes_per_cell_and_month() {
    assert_eq!(tile_len(0), 16);
    assert_eq!(tile_len(1), 16 + 9_600);
    assert_eq!(tile_len(2), 16 + 19_200);
    assert_eq!(tile_len(u16::MAX), 16 + 3 * 65_535 * 1_600 * 2);
}

#[test]
fn every_value_is_found_by_plane_month_row_and_column() {
    let counts = plane_of(3, 1);
    let sigmas = plane_of(3, 2);
    let energies = plane_of(3, 3);
    let bytes = encode(100, 3, [&counts, &sigmas, &energies]);
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!((view.first_month(), view.month_count()), (month(100), 3));
    for (plane, values) in [
        (Plane::Count, &counts),
        (Plane::Sigma, &sigmas),
        (Plane::Energy, &energies),
    ] {
        for offset in 0..3_u16 {
            for row in 0..40_u32 {
                for col in 0..40_u32 {
                    let at = usize::from(offset) * 1600 + usize::try_from(row * 40 + col).unwrap();
                    assert_eq!(
                        view.value(plane, month(100 + offset), row, col),
                        Ok(values[at])
                    );
                }
            }
        }
    }
}

#[test]
fn the_corners_of_the_first_and_last_month_land_where_the_layout_says() {
    let mut counts = vec![0_u16; 2 * TILE_CELLS];
    counts[0] = 7;
    counts[TILE_CELLS - 1] = 8;
    counts[TILE_CELLS] = 9;
    counts[2 * TILE_CELLS - 1] = u16::MAX;
    let zeros = vec![0_u16; 2 * TILE_CELLS];
    let bytes = encode(0, 2, [&counts, &zeros, &zeros]);
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(view.value(Plane::Count, month(0), 0, 0), Ok(7));
    assert_eq!(view.value(Plane::Count, month(0), 39, 39), Ok(8));
    assert_eq!(view.value(Plane::Count, month(1), 0, 0), Ok(9));
    assert_eq!(view.value(Plane::Count, month(1), 39, 39), Ok(65_535));
    assert_eq!(view.value(Plane::Sigma, month(1), 39, 39), Ok(0));
}

#[test]
fn a_tile_of_zero_months_is_valid_and_holds_no_month() {
    let bytes = encode(0, 0, [&[], &[], &[]]);
    assert_eq!(bytes.len(), 16);
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(view.month_count(), 0);
    assert_eq!(
        view.value(Plane::Count, month(0), 0, 0),
        Err(Error::MonthOutsideTile)
    );
}

#[test]
fn a_month_or_cell_outside_the_tile_is_refused() {
    let zeros = vec![0_u16; 2 * TILE_CELLS];
    let bytes = encode(10, 2, [&zeros, &zeros, &zeros]);
    let view = TileView::parse(&bytes).unwrap();
    assert_eq!(
        view.value(Plane::Count, month(9), 0, 0),
        Err(Error::MonthOutsideTile)
    );
    assert_eq!(
        view.value(Plane::Count, month(12), 0, 0),
        Err(Error::MonthOutsideTile)
    );
    assert_eq!(
        view.value(Plane::Count, month(10), 40, 0),
        Err(Error::CellOutsideTile)
    );
    assert_eq!(
        view.value(Plane::Count, month(10), 0, 40),
        Err(Error::CellOutsideTile)
    );
    assert_eq!(view.value(Plane::Count, month(11), 39, 39), Ok(0));
}

#[test]
fn a_plane_of_the_wrong_length_is_refused_on_write() {
    let right = vec![0_u16; TILE_CELLS];
    let wrong = vec![0_u16; TILE_CELLS - 1];
    let mut out = vec![0; tile_len(1)];
    let result = write_tile(month(0), 1, [&right, &wrong, &right], &mut out);
    assert_eq!(result, Err(Error::PlaneLengthMismatch));
}

#[test]
fn a_buffer_that_is_too_small_is_refused() {
    let zeros = vec![0_u16; TILE_CELLS];
    let mut out = vec![0; tile_len(1) - 1];
    assert_eq!(
        write_tile(month(0), 1, [&zeros, &zeros, &zeros], &mut out),
        Err(Error::BufferTooSmall)
    );
}

#[test]
fn a_damaged_file_is_refused() {
    let zeros = vec![0_u16; TILE_CELLS];
    let good = encode(0, 1, [&zeros, &zeros, &zeros]);
    let mut bytes = good.clone();
    bytes[0] = b'X';
    assert_eq!(TileView::parse(&bytes).unwrap_err(), Error::BadMagic);
    assert_eq!(TileView::parse(&good[..15]).unwrap_err(), Error::Truncated);
    assert_eq!(
        TileView::parse(&good[..good.len() - 1]).unwrap_err(),
        Error::Truncated
    );
    let mut bytes = good.clone();
    bytes.push(0);
    assert_eq!(TileView::parse(&bytes).unwrap_err(), Error::TrailingBytes);
    let mut bytes = good.clone();
    bytes[8] = 2;
    assert_eq!(
        TileView::parse(&bytes).unwrap_err(),
        Error::UnsupportedPlaneCount
    );
    let mut bytes = good;
    bytes[12] = 1;
    assert_eq!(TileView::parse(&bytes).unwrap_err(), Error::ReservedNotZero);
}

#[test]
fn a_month_count_that_promises_more_than_the_file_has_is_truncated() {
    let zeros = vec![0_u16; TILE_CELLS];
    let mut bytes = encode(0, 1, [&zeros, &zeros, &zeros]);
    bytes[4..6].copy_from_slice(&u16::MAX.to_le_bytes());
    assert_eq!(TileView::parse(&bytes).unwrap_err(), Error::Truncated);
}

#[test]
fn the_tile_of_a_cell_is_its_row_and_column_divided_by_forty() {
    let tile = |row, col| TileId::of_cell(CellIndex::new(row, col).unwrap());
    assert_eq!((tile(0, 0).row(), tile(0, 0).col()), (0, 0));
    assert_eq!((tile(39, 39).row(), tile(39, 39).col()), (0, 0));
    assert_eq!((tile(40, 0).row(), tile(40, 0).col()), (1, 0));
    assert_eq!((tile(719, 1439).row(), tile(719, 1439).col()), (17, 35));
}

#[test]
fn tile_ids_are_bounded_by_the_eighteen_by_thirty_six_grid() {
    assert!(TileId::new(17, 35).is_ok());
    assert_eq!(TileId::new(18, 0), Err(Error::CellOutsideTile));
    assert_eq!(TileId::new(0, 36), Err(Error::CellOutsideTile));
}
