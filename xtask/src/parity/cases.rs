//! The calls that must give the same bytes natively and in the browser build.

use firecal_testkit::SeededRandom;
use firecal_wasm::{fc_month_index, fc_month_span};

/// Bytes of one encoded call: function id, first argument, second argument, each a little-endian `u32`.
pub(super) const CASE_BYTES: usize = 12;

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

/// An export of `firecal.wasm`; the id is its position in the table of `run.mjs`.
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

/// One call to an export, with its two numeric arguments.
#[derive(Debug, Clone, Copy)]
pub(super) struct Case {
    pub(super) function: Function,
    pub(super) first: u32,
    pub(super) second: u32,
}

/// Every pair of edge values, then seeded random pairs, for each export.
pub(super) fn all_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut random = SeededRandom::new(SEED);
    for function in [Function::MonthIndex, Function::MonthSpan] {
        for first in EDGE_VALUES {
            for second in EDGE_VALUES {
                cases.push(Case {
                    function,
                    first,
                    second,
                });
            }
        }
        for _ in 0..RANDOM_CASES {
            cases.push(Case {
                function,
                first: random_below(&mut random),
                second: random_below(&mut random),
            });
        }
    }
    cases
}

fn random_below(random: &mut SeededRandom) -> u32 {
    u32::try_from(random.next_below(RANDOM_BOUND)).unwrap_or(0)
}

/// The cases as the bytes `run.mjs` reads.
pub(super) fn encode(cases: &[Case]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(cases.len() * CASE_BYTES);
    for case in cases {
        bytes.extend_from_slice(&case.function.id().to_le_bytes());
        bytes.extend_from_slice(&case.first.to_le_bytes());
        bytes.extend_from_slice(&case.second.to_le_bytes());
    }
    bytes
}

/// What the native build answers to each case, one little-endian `u32` per case.
pub(super) fn native_results(cases: &[Case]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(cases.len() * 4);
    for case in cases {
        bytes.extend_from_slice(&case.function.call(case.first, case.second).to_le_bytes());
    }
    bytes
}
