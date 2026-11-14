use super::cases::{CASE_BYTES, Case, Function, all_cases, encode, native_results};
use super::first_difference;

const INVALID_BYTES: [u8; 4] = u32::MAX.to_le_bytes();

const fn case(function: Function, first: u32, second: u32) -> Case {
    Case {
        function,
        first,
        second,
    }
}

#[test]
fn encoding_a_case_is_function_then_arguments_little_endian() {
    let bytes = encode(&[case(Function::MonthSpan, 10, 21)]);
    assert_eq!(bytes, [1, 0, 0, 0, 10, 0, 0, 0, 21, 0, 0, 0]);
}

#[test]
fn native_answers_match_the_documented_exports() {
    let cases = [
        case(Function::MonthIndex, 2000, 11),
        case(Function::MonthIndex, 2020, 13),
        case(Function::MonthSpan, 10, 21),
        case(Function::MonthSpan, 21, 10),
    ];
    let mut expected = Vec::new();
    expected.extend_from_slice(&10_u32.to_le_bytes());
    expected.extend_from_slice(&INVALID_BYTES);
    expected.extend_from_slice(&12_u32.to_le_bytes());
    expected.extend_from_slice(&INVALID_BYTES);
    assert_eq!(native_results(&cases), expected);
}

#[test]
fn the_case_list_covers_both_exports_and_every_edge_pair() {
    let cases = all_cases();
    assert_eq!(cases.len(), 2 * (12 * 12 + 2000));
    assert_eq!(encode(&cases).len(), cases.len() * CASE_BYTES);
}

#[test]
fn identical_bytes_have_no_difference() {
    assert_eq!(first_difference(&[1, 2, 3, 4], &[1, 2, 3, 4]), None);
}

#[test]
fn one_different_byte_names_its_call() {
    let message = first_difference(&[0; 12], &[0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0]).unwrap();
    assert!(message.starts_with("call 2 differs"), "{message}");
}

#[test]
fn different_lengths_are_a_difference() {
    assert!(first_difference(&[0; 8], &[0; 4]).is_some());
}
