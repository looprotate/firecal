//! Analysis calls over a seeded history of three years: the same request laid out in a memory image for the
//! native build, and as a payload for the browser build, whose loader lays it out in linear memory.

use super::cases::frame_length;
use super::series_cases::{SeriesCase, address_of, quarter_degrees, random_tile};
use firecal_testkit::SeededRandom;
use firecal_wasm::{SliceMemory, run_analogs, run_area_series, run_comparison, run_series_csv};

const ANALYSIS_REQUEST_BYTES: u32 = 80;
const SEED: u64 = 0xA11A_5EED;
const HISTORY_FIRST_MONTH: u16 = 12;
const HISTORY_MONTHS: u16 = 36;
const SLOTS: u32 = 3;
const CSV_ROOM_BYTES: u32 = 1 << 20;
const ANALOG_COUNT_BYTES: usize = 4;
const ANALOG_BYTES: usize = 12;
// Latitude 10 to 20 in tile rows 7 and 8, longitude 0 to 10 in tile column 18: both tiles hold hotspots.
const TILE_ROWS: [u32; 2] = [7, 8];
const TILE_COLUMN: u32 = 18;

/// Which analysis export a case calls; the id is its position in the table of `run.mjs`.
#[derive(Debug, Clone, Copy)]
pub(super) enum Export {
    AreaSeries,
    Comparison,
    Analogs,
    SeriesCsv,
}

impl Export {
    pub(super) const ALL: [Self; 4] = [
        Self::AreaSeries,
        Self::Comparison,
        Self::Analogs,
        Self::SeriesCsv,
    ];

    pub(super) const fn id(self) -> u32 {
        match self {
            Self::AreaSeries => 4,
            Self::Comparison => 5,
            Self::Analogs => 6,
            Self::SeriesCsv => 7,
        }
    }
}

/// One analysis request: a series request, a baseline, a season and maybe a polygon.
#[derive(Debug, Clone)]
pub(super) struct AnalysisCase {
    series: SeriesCase,
    baseline: [u32; 2],
    season: [u32; 2],
    corners: Vec<[f64; 2]>,
}

impl AnalysisCase {
    /// The payload `run.mjs` reads: the series payload, then baseline, season, analog slots and corner count
    /// as `u32`, then the corners as `f64`.
    pub(super) fn encode_payload(&self) -> Vec<u8> {
        let mut bytes = self.series.encode_payload();
        let corner_count = u32::try_from(self.corners.len()).unwrap_or(0);
        for word in [
            self.baseline[0],
            self.baseline[1],
            self.season[0],
            self.season[1],
            SLOTS,
            corner_count,
        ] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        for degrees in self.corners.iter().flatten() {
            bytes.extend_from_slice(&degrees.to_le_bytes());
        }
        bytes
    }

    /// The native answer: a `u32` status, then, when it is zero, what the page reads back (see `run.mjs`).
    pub(super) fn native_answer(&self, export: Export) -> Vec<u8> {
        let room = match export {
            Export::AreaSeries => 16 * self.series.months,
            Export::Comparison => 17 * self.series.months,
            Export::Analogs => 4 + 12 * SLOTS,
            Export::SeriesCsv => 4 + CSV_ROOM_BYTES,
        };
        let (mut image, request) = self.memory_image();
        let buffer = address_of(&image);
        image.resize(image.len() + usize::try_from(room).unwrap_or(0), 0);
        let call = match export {
            Export::AreaSeries => run_area_series,
            Export::Comparison => run_comparison,
            Export::Analogs => run_analogs,
            Export::SeriesCsv => run_series_csv,
        };
        let status = call(
            &mut SliceMemory::new(&mut image),
            request,
            ANALYSIS_REQUEST_BYTES,
            buffer,
            room,
        );
        let mut answer = status.to_le_bytes().to_vec();
        if status == 0 {
            let start = usize::try_from(buffer - 1).unwrap_or(0);
            answer.extend_from_slice(read_back(export, image.get(start..).unwrap_or(&[])));
        }
        answer
    }

    /// The tiles, the table, the corners and the request block, in one image whose address 1 is its first byte.
    fn memory_image(&self) -> (Vec<u8>, u32) {
        let mut image = Vec::new();
        let mut table = Vec::new();
        for (row, col, tile) in &self.series.tiles {
            table.extend([*row, *col, address_of(&image), frame_length(tile)]);
            image.extend_from_slice(tile);
        }
        let table_address = address_of(&image);
        for word in table {
            image.extend_from_slice(&word.to_le_bytes());
        }
        let corners = address_of(&image);
        for degrees in self.corners.iter().flatten() {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let request = address_of(&image);
        for degrees in self.series.area {
            image.extend_from_slice(&degrees.to_le_bytes());
        }
        let tile_count = u32::try_from(self.series.tiles.len()).unwrap_or(0);
        let corner_count = u32::try_from(self.corners.len()).unwrap_or(0);
        for word in [
            self.series.first_month,
            self.series.months,
            self.series.plane,
            tile_count,
            table_address,
            0,
            self.baseline[0],
            self.baseline[1],
            self.season[0],
            self.season[1],
            corner_count,
            corners,
        ] {
            image.extend_from_slice(&word.to_le_bytes());
        }
        (image, request)
    }
}

/// What the page reads out of the buffer: the whole buffer for fixed blocks, the text for the CSV and the slots
/// found for the analogs.
fn read_back(export: Export, buffer: &[u8]) -> &[u8] {
    let length = |at: usize| {
        buffer
            .get(at..at + 4)
            .and_then(|word| <[u8; 4]>::try_from(word).ok())
            .map_or(0, |word| {
                usize::try_from(u32::from_le_bytes(word)).unwrap_or(0)
            })
    };
    let end = match export {
        Export::AreaSeries | Export::Comparison => buffer.len(),
        Export::Analogs => ANALOG_COUNT_BYTES + length(0) * ANALOG_BYTES,
        Export::SeriesCsv => 4 + length(0),
    };
    let start = match export {
        Export::AreaSeries | Export::Comparison => 0,
        Export::Analogs | Export::SeriesCsv => 4,
    };
    buffer.get(start..end).unwrap_or(&[])
}

/// The areas, polygons and variants that touch every kind of edge, each for both planes and for each pairing of
/// tiles and months, over a seeded history.
pub(super) fn analysis_cases() -> Vec<AnalysisCase> {
    let mut random = SeededRandom::new(SEED);
    let tiles: Vec<(u32, u32, Vec<u8>)> = TILE_ROWS
        .iter()
        .map(|row| {
            let tile = random_tile(&mut random, HISTORY_FIRST_MONTH, HISTORY_MONTHS);
            (*row, TILE_COLUMN, tile)
        })
        .collect();
    let west = quarter_degrees(&mut random, 0.0, 8);
    let rectangles = [
        [10.0, 10.25, 0.0, 0.25],
        [5.0, 15.0, -5.0, 5.0],
        [west, west + 3.0, 0.0, 4.0],
        [10.0, 10.0, 0.0, 1.0],
    ];
    let polygons: [&[[f64; 2]]; 5] = [
        &[[10.0, 0.0], [10.25, 0.0], [10.25, 0.25], [10.0, 0.25]],
        &[[9.0, -1.0], [19.0, 1.0], [14.0, 9.0]],
        &[[10.0, 0.0], [10.25, 0.25], [10.25, 0.0], [10.0, 0.25]],
        &[[10.0, 179.0], [11.0, -179.0], [12.0, 179.0]],
        &[[10.0, 0.0], [11.0, 1.0]],
    ];
    let areas: Vec<([f64; 4], Vec<[f64; 2]>)> = rectangles
        .iter()
        .map(|area| (*area, Vec::new()))
        .chain(polygons.iter().map(|corners| ([0.0; 4], corners.to_vec())))
        .collect();
    let variants = [
        ([2001, 2002], [2003, 3]),
        ([2001, 2002], [2003, 13]),
        ([2002, 2001], [2003, 3]),
        ([2010, 2011], [2003, 3]),
        ([2001, 2002], [2004, 2]),
    ];
    let mut cases = Vec::new();
    for (area, corners) in &areas {
        for plane in [0, 1] {
            for (tile_count, months) in [(2, 36), (0, 36), (1, 0), (1, 40)] {
                for (baseline, season) in variants {
                    cases.push(AnalysisCase {
                        series: SeriesCase {
                            area: *area,
                            first_month: u32::from(HISTORY_FIRST_MONTH),
                            months,
                            plane,
                            tiles: tiles.iter().take(tile_count).cloned().collect(),
                        },
                        baseline,
                        season,
                        corners: corners.clone(),
                    });
                }
            }
        }
    }
    cases
}
