use super::run_series;
use crate::memory::SliceMemory;
use crate::{
    INVALID, STATUS_AREA_OUT_OF_RANGE, STATUS_EMPTY_AREA, STATUS_MONTH_NOT_IN_TILE,
    STATUS_MONTH_RANGE_INVALID, STATUS_OK, STATUS_TILE_INVALID,
};
use firecal_types::{MonthIndex, TILE_CELLS, tile_len, write_tile};

// Fixture, worked out by hand. Grid cell row 319, column 720 (latitude 10.0 to 10.25, longitude 0.0 to 0.25)
// is tile (7, 18) at position (39, 0); its east neighbour (39, 1) is column 721. Two months from index 10.
//
//                      count       sigma       energy
//                      m0  m1      m0  m1      m0  m1
//  tile (7,18) (39,0)   5   7       3  12      40  50
//  tile (7,18) (39,1)   3   0       4   5       2   0

pub(crate) const COUNT: u32 = 0;
pub(crate) const ENERGY: u32 = 1;
pub(crate) const REQUEST_LEN: u32 = 56;

pub(crate) fn fixture_tile() -> Vec<u8> {
    let mut planes = [
        vec![0_u16; 2 * TILE_CELLS],
        vec![0; 2 * TILE_CELLS],
        vec![0; 2 * TILE_CELLS],
    ];
    let cells: [(usize, [[u16; 3]; 2]); 2] = [
        (39 * 40, [[5, 3, 40], [7, 12, 50]]),
        (39 * 40 + 1, [[3, 4, 2], [0, 5, 0]]),
    ];
    for (position, months) in cells {
        for (month, values) in months.iter().enumerate() {
            for (plane, value) in planes.iter_mut().zip(values) {
                plane[month * TILE_CELLS + position] = *value;
            }
        }
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

/// One request, laid out in a memory image whose address 1 is its first byte.
pub(crate) struct Call {
    pub(crate) area: [f64; 4],
    pub(crate) first_month: u32,
    pub(crate) months: u32,
    pub(crate) plane: u32,
    pub(crate) tiles: Vec<(u32, u32, Vec<u8>)>,
}

impl Call {
    pub(crate) fn one_cell(plane: u32) -> Self {
        Self {
            area: [10.0, 10.25, 0.0, 0.25],
            first_month: 10,
            months: 2,
            plane,
            tiles: vec![(7, 18, fixture_tile())],
        }
    }

    /// The image, the address of the request and the address of the output.
    pub(crate) fn image(&self) -> (Vec<u8>, u32, u32) {
        let mut image = Vec::new();
        let mut entries = Vec::new();
        for (row, col, bytes) in &self.tiles {
            let address = u32::try_from(image.len() + 1).unwrap();
            image.extend_from_slice(bytes);
            entries.extend([*row, *col, address, u32::try_from(bytes.len()).unwrap()]);
        }
        let table = u32::try_from(image.len() + 1).unwrap();
        for word in entries {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let request = u32::try_from(image.len() + 1).unwrap();
        let output = request + REQUEST_LEN;
        for degrees in self.area {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let tile_count = u32::try_from(self.tiles.len()).unwrap();
        for word in [
            self.first_month,
            self.months,
            self.plane,
            tile_count,
            table,
            output,
        ] {
            image.extend_from_slice(&word.to_le_bytes());
        }
        image.resize(
            image.len() + 16 * usize::try_from(self.months).unwrap(),
            0xAA,
        );
        (image, request, output)
    }

    /// The status and the output as numbers: values then sigmas.
    fn run(&self) -> (u32, Vec<f64>) {
        let (mut image, request, output) = self.image();
        let status = run_series(&mut SliceMemory::new(&mut image), request, REQUEST_LEN);
        let start = usize::try_from(output - 1).unwrap();
        let numbers = image[start..]
            .as_chunks::<8>()
            .0
            .iter()
            .map(|chunk| f64::from_le_bytes(*chunk))
            .collect();
        (status, numbers)
    }
}

#[test]
fn one_cell_count_writes_values_then_sigmas() {
    assert_eq!(
        Call::one_cell(COUNT).run(),
        (STATUS_OK, vec![5.0, 7.0, 3.0, 12.0])
    );
}

#[test]
fn two_cells_add_counts_and_sigmas_in_quadrature() {
    let call = Call {
        area: [10.1, 10.2, 0.1, 0.3],
        ..Call::one_cell(COUNT)
    };
    // values 5 + 3 and 7 + 0; sigmas sqrt(3^2 + 4^2) = 5 and sqrt(12^2 + 5^2) = 13
    assert_eq!(call.run(), (STATUS_OK, vec![8.0, 7.0, 5.0, 13.0]));
}

#[test]
fn energy_writes_the_energy_and_nan_for_sigma() {
    let call = Call {
        area: [10.1, 10.2, 0.1, 0.3],
        ..Call::one_cell(ENERGY)
    };
    let (status, numbers) = call.run();
    assert_eq!(status, STATUS_OK);
    assert_eq!(numbers[..2], [42.0, 50.0]);
    assert!(numbers[2].is_nan() && numbers[3].is_nan());
}

#[test]
fn no_tiles_is_zeros() {
    let call = Call {
        tiles: Vec::new(),
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run(), (STATUS_OK, vec![0.0; 4]));
}

#[test]
fn an_area_with_no_extent_is_status_empty_area() {
    let call = Call {
        area: [10.0, 10.0, 0.0, 0.25],
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_EMPTY_AREA);
}

#[test]
fn a_latitude_beyond_the_pole_is_status_area_out_of_range() {
    let call = Call {
        area: [10.0, 95.0, 0.0, 0.25],
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_AREA_OUT_OF_RANGE);
}

#[test]
fn no_months_is_status_month_range_invalid() {
    let call = Call {
        months: 0,
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_MONTH_RANGE_INVALID);
}

#[test]
fn a_first_month_wider_than_sixteen_bits_is_status_month_range_invalid() {
    let call = Call {
        first_month: 70_000,
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_MONTH_RANGE_INVALID);
}

#[test]
fn a_month_the_tile_lacks_is_status_month_not_in_tile() {
    let call = Call {
        months: 3,
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_MONTH_NOT_IN_TILE);
}

#[test]
fn a_tile_that_is_not_a_tile_is_status_tile_invalid() {
    let call = Call {
        tiles: vec![(7, 18, vec![1, 2, 3])],
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_TILE_INVALID);
}

#[test]
fn a_tile_outside_the_tile_grid_is_status_tile_invalid() {
    let call = Call {
        tiles: vec![(18, 0, fixture_tile())],
        ..Call::one_cell(COUNT)
    };
    assert_eq!(call.run().0, STATUS_TILE_INVALID);
}

#[test]
fn an_unknown_plane_is_invalid() {
    assert_eq!(Call::one_cell(2).run().0, INVALID);
}

#[test]
fn a_status_other_than_ok_leaves_the_output_untouched() {
    let call = Call {
        area: [10.0, 10.0, 0.0, 0.25],
        ..Call::one_cell(COUNT)
    };
    let (status, numbers) = call.run();
    assert_eq!(status, STATUS_EMPTY_AREA);
    assert!(
        numbers
            .iter()
            .all(|number| number.to_bits() == 0xAAAA_AAAA_AAAA_AAAA)
    );
}

#[test]
fn a_request_of_the_wrong_length_is_invalid() {
    let (mut image, request, _) = Call::one_cell(COUNT).image();
    assert_eq!(
        run_series(&mut SliceMemory::new(&mut image), request, REQUEST_LEN - 1),
        INVALID
    );
}

#[test]
fn a_request_at_address_zero_or_past_the_memory_is_invalid() {
    let (mut image, _, _) = Call::one_cell(COUNT).image();
    let mut memory = SliceMemory::new(&mut image);
    assert_eq!(run_series(&mut memory, 0, REQUEST_LEN), INVALID);
    assert_eq!(run_series(&mut memory, u32::MAX - 10, REQUEST_LEN), INVALID);
}

/// The image with one u32 of the request block replaced, at byte `field` of the block.
fn with_word(field: usize, word: u32) -> u32 {
    let (mut image, request, _) = Call::one_cell(COUNT).image();
    let at = usize::try_from(request - 1).unwrap() + field;
    image[at..at + 4].copy_from_slice(&word.to_le_bytes());
    run_series(&mut SliceMemory::new(&mut image), request, REQUEST_LEN)
}

#[test]
fn a_tile_table_past_the_memory_is_invalid() {
    assert_eq!(with_word(48, 1 << 30), INVALID);
}

#[test]
fn a_tile_count_that_overflows_the_table_size_is_invalid() {
    assert_eq!(with_word(44, u32::MAX), INVALID);
}

#[test]
fn an_output_past_the_memory_is_invalid() {
    assert_eq!(with_word(52, 1 << 30), INVALID);
}

#[test]
fn a_month_count_that_overflows_the_output_size_is_invalid() {
    assert_eq!(with_word(36, u32::MAX), INVALID);
}

#[test]
fn a_tile_that_points_past_the_memory_is_invalid() {
    let (mut image, request, _) = Call::one_cell(COUNT).image();
    // The table follows the tile; its first entry's address field is at byte 8 of the table.
    let table = fixture_tile().len() + 8;
    image[table..table + 4].copy_from_slice(&(1_u32 << 30).to_le_bytes());
    assert_eq!(
        run_series(&mut SliceMemory::new(&mut image), request, REQUEST_LEN),
        INVALID
    );
}
