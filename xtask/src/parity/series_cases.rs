//! Series calls over seeded tiles: the same request laid out in a memory image for the native build, and as
//! a payload for the browser build, which the page's loader lays out in linear memory by itself.

use super::cases::frame_length;
use firecal_testkit::SeededRandom;
use firecal_types::{MonthIndex, TILE_CELLS, tile_len, write_tile};
use firecal_wasm::{SliceMemory, run_cells_csv, run_series};

const REQUEST_BYTES: u32 = 56;
const FIRST_MONTH: u16 = 10;
const TILE_MONTHS: u16 = 3;
const SEED: u64 = 0xF1AE_5EED;
const RANDOM_AREAS: u64 = 60;
const CSV_ROOM_BYTES: u32 = 4 << 20;
const WORLD: [f64; 4] = [-90.0, 90.0, -180.0, 180.0];
// Latitude 10 to 20 in tile rows 7 and 8 and longitude 0 to 10 in tile column 18: both tiles hold hotspots.
const TILE_ROWS: [u32; 2] = [7, 8];
const TILE_COLUMN: u32 = 18;

/// One series request: an area, months, a plane and the tiles it is given.
#[derive(Debug, Clone)]
pub(super) struct SeriesCase {
    pub(super) area: [f64; 4],
    pub(super) first_month: u32,
    pub(super) months: u32,
    pub(super) plane: u32,
    pub(super) tiles: Vec<(u32, u32, Vec<u8>)>,
}

impl SeriesCase {
    /// The payload `run.mjs` reads: the area, four `u32`, then each tile as row, column, length and bytes.
    pub(super) fn encode_payload(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for degrees in self.area {
            bytes.extend_from_slice(&degrees.to_le_bytes());
        }
        let count = u32::try_from(self.tiles.len()).unwrap_or(0);
        for word in [self.first_month, self.months, self.plane, count] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        for (row, col, tile) in &self.tiles {
            for word in [*row, *col, frame_length(tile)] {
                bytes.extend_from_slice(&word.to_le_bytes());
            }
            bytes.extend_from_slice(tile);
        }
        bytes
    }

    /// The native answer: a `u32` status, then, when it is zero, the values and the sigmas as `f64`.
    pub(super) fn native_answer(&self) -> Vec<u8> {
        let (mut image, request, output) = self.memory_image();
        let status = run_series(&mut SliceMemory::new(&mut image), request, REQUEST_BYTES);
        let mut answer = status.to_le_bytes().to_vec();
        if status == 0 {
            let start = usize::try_from(output - 1).unwrap_or(0);
            answer.extend_from_slice(image.get(start..).unwrap_or(&[]));
        }
        answer
    }

    /// The native answer of the `cells.csv` export: a `u32` status, then, when it is zero, the text.
    pub(super) fn native_csv_answer(&self) -> Vec<u8> {
        let (mut image, request, _) = self.memory_image();
        let buffer = address_of(&image);
        let capacity = 4 + CSV_ROOM_BYTES;
        image.resize(image.len() + usize::try_from(capacity).unwrap_or(0), 0);
        let status = run_cells_csv(
            &mut SliceMemory::new(&mut image),
            request,
            REQUEST_BYTES,
            buffer,
            capacity,
        );
        let mut answer = status.to_le_bytes().to_vec();
        if status == 0 {
            let start = usize::try_from(buffer - 1).unwrap_or(0);
            let length = image
                .get(start..start + 4)
                .and_then(|word| <[u8; 4]>::try_from(word).ok())
                .map_or(0, u32::from_le_bytes);
            let end = start + 4 + usize::try_from(length).unwrap_or(0);
            answer.extend_from_slice(image.get(start + 4..end).unwrap_or(&[]));
        }
        answer
    }

    /// The tiles, the table, the request block and the output, in one image whose address 1 is its first byte.
    fn memory_image(&self) -> (Vec<u8>, u32, u32) {
        let mut image = Vec::new();
        let mut table = Vec::new();
        for (row, col, tile) in &self.tiles {
            table.extend([*row, *col, address_of(&image), frame_length(tile)]);
            image.extend_from_slice(tile);
        }
        let table_address = address_of(&image);
        for word in table {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let request = address_of(&image);
        let output = request + REQUEST_BYTES;
        for degrees in self.area {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let count = u32::try_from(self.tiles.len()).unwrap_or(0);
        for word in [
            self.first_month,
            self.months,
            self.plane,
            count,
            table_address,
            output,
        ] {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let output_bytes = usize::try_from(self.months * 16).unwrap_or(0);
        image.resize(image.len() + output_bytes, 0);
        (image, request, output)
    }
}

pub(super) fn address_of(image: &[u8]) -> u32 {
    u32::try_from(image.len() + 1).unwrap_or(u32::MAX)
}

/// A tile of seeded random counts, sigmas and energies for `months` months from `first_month`.
pub(super) fn random_tile(random: &mut SeededRandom, first_month: u16, months: u16) -> Vec<u8> {
    let mut plane = |bound: u64| -> Vec<u16> {
        (0..usize::from(months) * TILE_CELLS)
            .map(|_| u16::try_from(random.next_below(bound)).unwrap_or(0))
            .collect()
    };
    let (count, sigma, energy) = (plane(1000), plane(100), plane(5000));
    let mut bytes = vec![0; tile_len(months)];
    let written = write_tile(
        MonthIndex::from_index(first_month),
        months,
        [&count, &sigma, &energy],
        &mut bytes,
    );
    if written.is_err() {
        // Cannot happen: the planes have the length the format demands. An empty tile fails every case loudly.
        bytes.clear();
    }
    bytes
}

pub(super) fn quarter_degrees(random: &mut SeededRandom, low: f64, steps: u64) -> f64 {
    0.25_f64.mul_add(
        f64::from(u32::try_from(random.next_below(steps)).unwrap_or(0)),
        low,
    )
}

/// Fixed areas that touch every kind of edge, then seeded random rectangles, each for both planes and for
/// both tiles, one tile and none.
pub(super) fn series_cases() -> Vec<SeriesCase> {
    let mut random = SeededRandom::new(SEED);
    let tiles: Vec<(u32, u32, Vec<u8>)> = TILE_ROWS
        .iter()
        .map(|row| {
            (
                *row,
                TILE_COLUMN,
                random_tile(&mut random, FIRST_MONTH, TILE_MONTHS),
            )
        })
        .collect();
    let mut areas = vec![
        [10.0, 10.25, 0.0, 0.25],
        [10.1, 10.2, 0.1, 0.2],
        [9.9, 10.1, 0.1, 0.2],
        [5.0, 15.0, -5.0, 5.0],
        [0.0, 20.0, 0.0, 10.0],
        [-90.0, 90.0, -180.0, 180.0],
        [10.0, 10.0, 0.0, 1.0],
        [10.0, 95.0, 0.0, 1.0],
    ];
    for _ in 0..RANDOM_AREAS {
        let south = quarter_degrees(&mut random, 0.0, 80);
        let west = quarter_degrees(&mut random, -2.0, 50);
        areas.push([
            south,
            south + 0.25 + quarter_degrees(&mut random, 0.0, 40),
            west,
            west + 0.25 + quarter_degrees(&mut random, 0.0, 40),
        ]);
    }
    let mut cases = Vec::new();
    for area in areas {
        for plane in [0, 1] {
            for tile_count in [2, 1, 0] {
                for months in [3, 1, 4, 0] {
                    cases.push(SeriesCase {
                        area,
                        first_month: u32::from(FIRST_MONTH),
                        months,
                        plane,
                        tiles: tiles.iter().take(tile_count).cloned().collect(),
                    });
                }
            }
        }
    }
    cases
}

/// The series cases, except the whole world, whose text would be too long to compare.
pub(super) fn csv_cases() -> Vec<SeriesCase> {
    series_cases()
        .into_iter()
        .filter(|case| case.area.map(f64::to_bits) != WORLD.map(f64::to_bits))
        .collect()
}
