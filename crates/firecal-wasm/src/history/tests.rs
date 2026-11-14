use super::read_history;
use crate::memory::SliceMemory;
use crate::{INVALID, STATUS_POLYGON_INVALID};
use firecal_types::{MonthIndex, TILE_CELLS, tile_len, write_tile};

// Fixture, worked out by hand: tile (7, 18), 36 months from January 2001 (index 12). Cell (39, 0) is grid row 319,
// column 720 (latitude 10.0 to 10.25, longitude 0.0 to 0.25, center 10.125, 0.125); its east neighbour (39, 1)
// has center 10.125, 0.375. Counts of cell (39, 0), the same sigma 2 every month, no energy:
//
//          Jan  Feb  other months
//   2001    10   20   100
//   2002    30   40   100
//   2003     5   22   100
//
// Cell (39, 1) holds 1 every month with sigma 0.

pub(crate) const MONTHS: u32 = 36;
pub(crate) const ONE_CELL: [f64; 4] = [10.0, 10.25, 0.0, 0.25];
const JANUARY_FEBRUARY: [[u16; 2]; 3] = [[10, 20], [30, 40], [5, 22]];

pub(crate) fn fixture_tile() -> Vec<u8> {
    let mut count = vec![0_u16; 36 * TILE_CELLS];
    let mut sigma = vec![0_u16; 36 * TILE_CELLS];
    for month in 0..36 {
        let (year, calendar) = (month / 12, month % 12);
        count[month * TILE_CELLS + 39 * 40] = JANUARY_FEBRUARY
            .get(year)
            .and_then(|months| months.get(calendar))
            .copied()
            .unwrap_or(100);
        count[month * TILE_CELLS + 39 * 40 + 1] = 1;
        sigma[month * TILE_CELLS + 39 * 40] = 2;
    }
    let energy = vec![0_u16; 36 * TILE_CELLS];
    let mut bytes = vec![0; tile_len(36)];
    write_tile(
        MonthIndex::from_index(12),
        36,
        [&count, &sigma, &energy],
        &mut bytes,
    )
    .unwrap();
    bytes
}

/// One analysis request, laid out in a memory image whose address 1 is its first byte.
pub(crate) struct Call {
    pub(crate) area: [f64; 4],
    pub(crate) baseline: (u32, u32),
    pub(crate) season: (u32, u32),
    pub(crate) corners: Vec<[f64; 2]>,
    pub(crate) months: u32,
    pub(crate) tiles: Vec<(u32, u32, Vec<u8>)>,
}

impl Call {
    pub(crate) fn fixture() -> Self {
        Self {
            area: ONE_CELL,
            baseline: (2001, 2002),
            season: (2003, 2),
            corners: Vec::new(),
            months: MONTHS,
            tiles: vec![(7, 18, fixture_tile())],
        }
    }

    pub(crate) fn polygon(corners: &[[f64; 2]]) -> Self {
        Self {
            corners: corners.to_vec(),
            ..Self::fixture()
        }
    }

    /// The image, the address of the request block and the address of a buffer of `room` bytes after it.
    pub(crate) fn image(&self, room: usize) -> (Vec<u8>, u32, u32) {
        let at = |image: &Vec<u8>| u32::try_from(image.len() + 1).unwrap();
        let mut image = Vec::new();
        let mut entries = Vec::new();
        for (row, col, bytes) in &self.tiles {
            entries.extend([*row, *col, at(&image), u32::try_from(bytes.len()).unwrap()]);
            image.extend_from_slice(bytes);
        }
        let table = at(&image);
        for word in entries {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let vertices = at(&image);
        for degrees in self.corners.iter().flatten() {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let request = at(&image);
        for degrees in self.area {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let words = [
            12,
            self.months,
            0,
            u32::try_from(self.tiles.len()).unwrap(),
            table,
            0,
            self.baseline.0,
            self.baseline.1,
            self.season.0,
            self.season.1,
            u32::try_from(self.corners.len()).unwrap(),
            vertices,
        ];
        for word in words {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let buffer = at(&image);
        image.resize(image.len() + room, 0xAA);
        (image, request, buffer)
    }

    /// Runs `export` with a buffer of `room` bytes and returns its status and the buffer.
    pub(crate) fn run(
        &self,
        room: usize,
        export: impl FnOnce(&mut SliceMemory<'_>, u32, u32, u32, u32) -> u32,
    ) -> (u32, Vec<u8>) {
        let (mut image, request, buffer) = self.image(room);
        let status = export(
            &mut SliceMemory::new(&mut image),
            request,
            80,
            buffer,
            u32::try_from(room).unwrap(),
        );
        let start = usize::try_from(buffer - 1).unwrap();
        (status, image[start..].to_vec())
    }
}

pub(crate) fn doubles(bytes: &[u8]) -> Vec<f64> {
    bytes
        .as_chunks::<8>()
        .0
        .iter()
        .map(|chunk| f64::from_le_bytes(*chunk))
        .collect()
}

fn history_of(call: &Call) -> Result<Vec<f64>, u32> {
    let (mut image, request, _) = call.image(0);
    read_history(&SliceMemory::new(&mut image), request, 80).map(|(_, history)| history.value)
}

#[test]
fn a_rectangle_gives_the_series_of_its_cell() {
    let value = history_of(&Call::fixture()).unwrap();
    assert_eq!(value.len(), 36);
    assert_eq!(value[..4], [10.0, 20.0, 100.0, 100.0]);
    assert_eq!(value[24..26], [5.0, 22.0]);
}

#[test]
fn a_rectangle_over_two_cells_adds_the_neighbour() {
    let call = Call {
        area: [10.0, 10.25, 0.0, 0.5],
        ..Call::fixture()
    };
    assert_eq!(history_of(&call).unwrap()[..2], [11.0, 21.0]);
}

#[test]
fn a_polygon_selects_the_cells_whose_center_it_contains() {
    let around_east_cell = [[10.0, 0.25], [10.25, 0.25], [10.25, 0.5], [10.0, 0.5]];
    let value = history_of(&Call::polygon(&around_east_cell)).unwrap();
    assert!(
        value
            .iter()
            .all(|number| number.to_bits() == 1.0_f64.to_bits())
    );
}

#[test]
fn a_polygon_ignores_the_rectangle_fields() {
    let around_west_cell = [[10.0, 0.0], [10.25, 0.0], [10.25, 0.25], [10.0, 0.25]];
    let call = Call {
        area: [0.0, 0.0, 0.0, 0.0],
        ..Call::polygon(&around_west_cell)
    };
    assert_eq!(history_of(&call).unwrap()[..2], [10.0, 20.0]);
}

#[test]
fn a_bow_tie_polygon_is_refused() {
    let bow_tie = [[10.0, 0.0], [10.25, 0.25], [10.25, 0.0], [10.0, 0.25]];
    assert_eq!(
        history_of(&Call::polygon(&bow_tie)).err(),
        Some(STATUS_POLYGON_INVALID)
    );
}

#[test]
fn corners_outside_memory_or_a_wrong_block_length_are_invalid() {
    let (mut image, request, _) = Call::polygon(&[[0.0, 0.0]; 3]).image(0);
    let mut memory = SliceMemory::new(&mut image);
    assert_eq!(read_history(&memory, request, 79).err(), Some(INVALID));
    assert_eq!(read_history(&memory, request, 81).err(), Some(INVALID));
    assert_eq!(read_history(&memory, 0, 80).err(), Some(INVALID));
    let (mut image, request, _) = Call::polygon(&[[0.0, 0.0]; 3]).image(0);
    // The corner count field is at offset 72 of the block: make it larger than the memory.
    let at = usize::try_from(request - 1).unwrap() + 72;
    image[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    memory = SliceMemory::new(&mut image);
    assert_eq!(read_history(&memory, request, 80).err(), Some(INVALID));
}
