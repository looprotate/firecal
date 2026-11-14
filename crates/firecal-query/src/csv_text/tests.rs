use super::TextBuffer;
use crate::QueryError;

#[test]
fn text_is_appended_in_order() {
    let mut out = [0_u8; 16];
    let mut buffer = TextBuffer::new(&mut out);
    buffer.put(format_args!("{},{}", 2000, 11)).unwrap();
    buffer.put(format_args!("\n")).unwrap();
    let written = buffer.written();
    assert_eq!(&out[..written], b"2000,11\n");
}

#[test]
fn text_that_exactly_fills_the_buffer_fits() {
    let mut out = [0_u8; 4];
    assert_eq!(TextBuffer::new(&mut out).put(format_args!("abcd")), Ok(()));
}

#[test]
fn text_one_byte_too_long_is_buffer_too_small() {
    let mut out = [0_u8; 4];
    assert_eq!(
        TextBuffer::new(&mut out).put(format_args!("abcde")),
        Err(QueryError::BufferTooSmall)
    );
}

#[test]
fn a_number_has_the_decimals_asked_for_and_a_missing_number_is_empty() {
    let mut out = [0_u8; 32];
    let mut buffer = TextBuffer::new(&mut out);
    buffer.put_number(10.125, 3).unwrap();
    buffer.put(format_args!("|")).unwrap();
    buffer.put_number(5.0, 0).unwrap();
    buffer.put(format_args!("|")).unwrap();
    buffer.put_number(f64::NAN, 3).unwrap();
    buffer.put(format_args!("|")).unwrap();
    buffer.put_number(f64::INFINITY, 3).unwrap();
    let written = buffer.written();
    assert_eq!(&out[..written], b"10.125|5||");
}

#[test]
fn a_negative_number_keeps_its_sign() {
    let mut out = [0_u8; 8];
    let mut buffer = TextBuffer::new(&mut out);
    buffer.put_number(-0.75, 3).unwrap();
    let written = buffer.written();
    assert_eq!(&out[..written], b"-0.750");
}
