use super::fields;
use crate::ParseError;

#[test]
fn plain_quoted_and_escaped_extra_fields_preserve_column_boundaries() {
    assert_eq!(
        fields(b" a,\"b,c\",\"d\"\"e\",\"\",\r\n").unwrap(),
        ["a", "b,c", "d\"\"e", "", ""]
    );
    assert_eq!(fields(b"\xef\xbb\xbfa,b\n").unwrap(), ["a", "b"]);
}

#[test]
fn malformed_csv_encoding_and_empty_rows_are_distinct_errors() {
    assert_eq!(fields(&[0xff]), Err(ParseError::InvalidUtf8));
    for row in ["", "\r\n", "   "] {
        assert_eq!(fields(row.as_bytes()), Err(ParseError::EmptyRecord));
    }
    for row in [
        "\"open",
        "a\"b\"",
        "\"a\"text",
        "\"a\"b\"c\"",
        "a\nb",
        "a\rb",
    ] {
        assert_eq!(fields(row.as_bytes()), Err(ParseError::InvalidCsv), "{row}");
    }
}
