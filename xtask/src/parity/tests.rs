use super::cases::{Case, Function, all_cases, encode, native_results};
use super::first_difference;

const INVALID_BYTES: [u8; 4] = u32::MAX.to_le_bytes();

const fn months(function: Function, first: u32, second: u32) -> Case {
    Case::Months {
        function,
        first,
        second,
    }
}

#[test]
fn encoding_a_month_call_is_id_length_and_two_arguments_little_endian() {
    let bytes = encode(&[months(Function::MonthSpan, 10, 21)]);
    assert_eq!(bytes, [1, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 21, 0, 0, 0]);
}

#[test]
fn native_answers_are_framed_and_match_the_documented_exports() {
    let cases = [
        months(Function::MonthIndex, 2000, 11),
        months(Function::MonthIndex, 2020, 13),
        months(Function::MonthSpan, 10, 21),
        months(Function::MonthSpan, 21, 10),
    ];
    let mut expected = Vec::new();
    for answer in [
        10_u32.to_le_bytes(),
        INVALID_BYTES,
        12_u32.to_le_bytes(),
        INVALID_BYTES,
    ] {
        expected.extend_from_slice(&4_u32.to_le_bytes());
        expected.extend_from_slice(&answer);
    }
    assert_eq!(native_results(&cases), expected);
}

#[test]
fn the_case_list_has_month_calls_and_series_calls() {
    let cases = all_cases();
    let month_calls = 2 * (12 * 12 + 2000);
    assert!(
        cases[..month_calls]
            .iter()
            .all(|case| matches!(case, Case::Months { .. }))
    );
    assert!(
        cases[month_calls..]
            .iter()
            .all(|case| matches!(case, Case::Series(_) | Case::CellsCsv(_)))
    );
    assert!(cases.len() > month_calls);
}

#[test]
fn a_series_that_succeeds_answers_status_zero_then_two_f64_per_month() {
    let cases = all_cases();
    let ok = cases
        .iter()
        .filter_map(|case| match case {
            Case::Series(series) => Some(series.native_answer()),
            Case::CellsCsv(_) | Case::Months { .. } => None,
        })
        .find(|answer| answer.get(..4) == Some(&[0, 0, 0, 0]))
        .unwrap();
    assert_eq!((ok.len() - 4) % 16, 0, "two f64 per month after the status");
}

#[test]
fn identical_bytes_have_no_difference() {
    assert_eq!(
        first_difference(&[4, 0, 0, 0, 1, 2, 3, 4], &[4, 0, 0, 0, 1, 2, 3, 4]),
        None
    );
}

#[test]
fn one_different_byte_names_its_call() {
    let native = [4, 0, 0, 0, 1, 2, 3, 4, 4, 0, 0, 0, 5, 6, 7, 8];
    let mut wasm = native;
    wasm[14] = 0;
    let message = first_difference(&native, &wasm).unwrap();
    assert!(
        message.starts_with("call 1 differs at byte 14"),
        "{message}"
    );
}

#[test]
fn different_lengths_are_a_difference() {
    assert!(first_difference(&[4, 0, 0, 0, 1, 2, 3, 4], &[4, 0, 0, 0, 1, 2, 3]).is_some());
}
