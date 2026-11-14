//! The calls that must give the same bytes natively and in the browser build, and how they are encoded.

use super::series_cases::{SeriesCase, series_cases};
use firecal_testkit::SeededRandom;
use firecal_wasm::{fc_month_index, fc_month_span};

const EDGE_VALUES: [u32; 12] = [
    0,
    1,
    10,
    11,
    12,
    13,
    2000,
    2025,
    65_535,
    65_536,
    70_000,
    u32::MAX,
];
const RANDOM_CASES: u64 = 2000;
const RANDOM_BOUND: u64 = 8000;
const SEED: u64 = 0x5EED_F1AE;
const SERIES_ID: u32 = 2;

/// A month export of `firecal.wasm`; the id is its position in the table of `run.mjs`.
#[derive(Debug, Clone, Copy)]
pub(super) enum Function {
    MonthIndex,
    MonthSpan,
}

impl Function {
    const fn id(self) -> u32 {
        match self {
            Self::MonthIndex => 0,
            Self::MonthSpan => 1,
        }
    }

    fn call(self, first: u32, second: u32) -> u32 {
        match self {
            Self::MonthIndex => fc_month_index(first, second),
            Self::MonthSpan => fc_month_span(first, second),
        }
    }
}

/// One call to an export.
#[derive(Debug, Clone)]
pub(super) enum Case {
    /// A month export with its two numeric arguments.
    Months {
        function: Function,
        first: u32,
        second: u32,
    },
    /// The series export over tiles in memory.
    Series(SeriesCase),
}

/// Every pair of edge values and seeded random pairs for each month export, then the series cases.
pub(super) fn all_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut random = SeededRandom::new(SEED);
    for function in [Function::MonthIndex, Function::MonthSpan] {
        for first in EDGE_VALUES {
            for second in EDGE_VALUES {
                cases.push(Case::Months {
                    function,
                    first,
                    second,
                });
            }
        }
        for _ in 0..RANDOM_CASES {
            cases.push(Case::Months {
                function,
                first: random_below(&mut random),
                second: random_below(&mut random),
            });
        }
    }
    cases.extend(series_cases().into_iter().map(Case::Series));
    cases
}

fn random_below(random: &mut SeededRandom) -> u32 {
    u32::try_from(random.next_below(RANDOM_BOUND)).unwrap_or(0)
}

/// The cases as the records `run.mjs` reads: function id, payload length, payload.
pub(super) fn encode(cases: &[Case]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for case in cases {
        let (id, payload) = match case {
            Case::Months {
                function,
                first,
                second,
            } => {
                let mut payload = first.to_le_bytes().to_vec();
                payload.extend_from_slice(&second.to_le_bytes());
                (function.id(), payload)
            }
            Case::Series(series) => (SERIES_ID, series.encode_payload()),
        };
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&frame_length(&payload).to_le_bytes());
        bytes.extend_from_slice(&payload);
    }
    bytes
}

/// What the native build answers to each case: per case a `u32` answer length and the answer.
pub(super) fn native_results(cases: &[Case]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for case in cases {
        let answer = match case {
            Case::Months {
                function,
                first,
                second,
            } => function.call(*first, *second).to_le_bytes().to_vec(),
            Case::Series(series) => series.native_answer(),
        };
        bytes.extend_from_slice(&frame_length(&answer).to_le_bytes());
        bytes.extend_from_slice(&answer);
    }
    bytes
}

/// A length for a frame; a case never comes near `u32::MAX` bytes.
pub(super) fn frame_length(bytes: &[u8]) -> u32 {
    u32::try_from(bytes.len()).unwrap_or(u32::MAX)
}
