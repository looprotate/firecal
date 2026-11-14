use super::baseline_of;
use crate::STATUS_BASELINE_INVALID;
use crate::analysis_request::AnalysisRequest;

fn request(first: u32, last: u32) -> AnalysisRequest {
    let mut bytes = vec![0; 80];
    bytes[56..60].copy_from_slice(&first.to_le_bytes());
    bytes[60..64].copy_from_slice(&last.to_le_bytes());
    AnalysisRequest::parse(&bytes).unwrap()
}

#[test]
fn the_baseline_is_the_years_of_the_request() {
    assert!(baseline_of(&request(2001, 2024)).is_ok());
    assert!(baseline_of(&request(2024, 2024)).is_ok());
}

#[test]
fn a_baseline_backwards_or_wider_than_sixteen_bits_is_refused() {
    assert_eq!(
        baseline_of(&request(2024, 2001)).err(),
        Some(STATUS_BASELINE_INVALID)
    );
    assert_eq!(
        baseline_of(&request(2001, 65_536)).err(),
        Some(STATUS_BASELINE_INVALID)
    );
}
