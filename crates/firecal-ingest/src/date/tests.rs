use super::parse;
use crate::ParseError;

#[test]
fn valid_dates_have_expected_month_and_day_including_leap_centuries() {
    for (text, month, day) in [
        ("2000-01-01", 0, 1),
        ("2000-02-29", 1, 29),
        ("2024-02-29", 289, 29),
        ("2400-02-29", 4801, 29),
        ("2024-04-30", 291, 30),
    ] {
        let (actual_month, actual_day) = parse(text).unwrap();
        assert_eq!((actual_month.index(), actual_day), (month, day));
    }
}

#[test]
fn invalid_dates_and_month_index_overflow_are_rejected() {
    for text in [
        "1999-12-31",
        "2023-02-29",
        "2100-02-29",
        "2024-04-31",
        "2024-02-30",
        "2024-01-00",
        "2024-00-01",
        "2024-13-01",
        "2024-1-01",
        "2024/01/01",
        "2024-01-01x",
        "9999-01-01",
        "abcd-ef-gh",
        "２０２４-01-01",
    ] {
        assert_eq!(parse(text), Err(ParseError::InvalidDate), "{text}");
    }
}
