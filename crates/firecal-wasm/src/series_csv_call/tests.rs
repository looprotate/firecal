use super::run_series_csv;
use crate::history::tests::Call;
use crate::{INVALID, STATUS_OK, STATUS_OUTPUT_TOO_SMALL};

fn text_of(buffer: &[u8]) -> String {
    let length = usize::try_from(u32::from_le_bytes(buffer[..4].try_into().unwrap())).unwrap();
    String::from_utf8(buffer[4..4 + length].to_vec()).unwrap()
}

#[test]
fn the_series_is_written_with_its_median_anomaly_and_flag() {
    let (status, buffer) = Call::fixture().run(4096, |m, r, l, b, n| run_series_csv(m, r, l, b, n));
    assert_eq!(status, STATUS_OK);
    let text = text_of(&buffer);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 37);
    assert_eq!(lines[0], "year,month,value,sigma,median,anomaly,unusual");
    assert_eq!(lines[1], "2001,1,10,2.000,20.0,-0.500,true");
    assert_eq!(lines[25], "2003,1,5,2.000,20.0,-0.750,true");
    assert_eq!(lines[27], "2003,3,100,2.000,100.0,0.000,false");
}

#[test]
fn a_month_without_a_baseline_leaves_the_median_and_anomaly_empty() {
    let call = Call {
        baseline: (2010, 2011),
        ..Call::fixture()
    };
    let text = text_of(
        &call
            .run(4096, |m, r, l, b, n| run_series_csv(m, r, l, b, n))
            .1,
    );
    assert_eq!(text.lines().nth(1), Some("2001,1,10,2.000,,,false"));
}

#[test]
fn text_that_does_not_fit_is_reported_and_nothing_is_written() {
    let (status, buffer) =
        Call::fixture().run(4 + 10, |m, r, l, b, n| run_series_csv(m, r, l, b, n));
    assert_eq!(status, STATUS_OUTPUT_TOO_SMALL);
    assert!(buffer.iter().all(|byte| *byte == 0xAA));
}

#[test]
fn a_buffer_without_room_for_the_length_is_invalid() {
    assert_eq!(
        Call::fixture()
            .run(3, |m, r, l, b, n| run_series_csv(m, r, l, b, n))
            .0,
        INVALID
    );
}
