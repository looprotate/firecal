use super::{Writer, read_f32, read_u16, read_u32};
use crate::Error;

#[test]
fn integers_are_read_little_endian_at_an_offset() {
    let bytes = [0xff, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
    assert_eq!(read_u16(&bytes, 1), Some(0x1234));
    assert_eq!(read_u32(&bytes, 3), Some(0x1234_5678));
}

#[test]
fn reading_past_the_end_is_none_not_a_panic() {
    let bytes = [1, 2, 3];
    assert_eq!(read_u16(&bytes, 2), None);
    assert_eq!(read_u32(&bytes, 0), None);
    assert_eq!(read_u32(&bytes, usize::MAX), None);
}

#[test]
fn a_float_is_read_from_its_bits() {
    assert_eq!(
        read_f32(&[0x00, 0x00, 0x48, 0x41], 0).map(f32::to_bits),
        Some(12.5_f32.to_bits())
    );
}

#[test]
fn the_writer_fills_the_buffer_in_order_and_counts() {
    let mut buffer = [0_u8; 5];
    let mut writer = Writer::new(&mut buffer);
    assert_eq!(writer.put(&[1, 2]), Ok(()));
    assert_eq!(writer.put(&[3, 4, 5]), Ok(()));
    assert_eq!(writer.written(), 5);
    assert_eq!(buffer, [1, 2, 3, 4, 5]);
}

#[test]
fn the_writer_refuses_to_overrun() {
    let mut buffer = [0_u8; 3];
    let mut writer = Writer::new(&mut buffer);
    assert_eq!(writer.put(&[1, 2]), Ok(()));
    assert_eq!(writer.put(&[3, 4]), Err(Error::BufferTooSmall));
    assert_eq!(writer.written(), 2);
}
