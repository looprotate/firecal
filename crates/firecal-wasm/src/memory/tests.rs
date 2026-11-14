use super::{Memory, SliceMemory};

#[test]
fn address_one_is_the_first_byte() {
    let mut bytes = [7, 8, 9];
    let memory = SliceMemory::new(&mut bytes);
    assert_eq!(memory.read(1, 2), Some(&[7, 8][..]));
    assert_eq!(memory.read(3, 1), Some(&[9][..]));
}

#[test]
fn address_zero_is_refused() {
    let mut bytes = [7, 8, 9];
    assert_eq!(SliceMemory::new(&mut bytes).read(0, 1), None);
}

#[test]
fn reading_past_the_end_is_refused() {
    let mut bytes = [7, 8, 9];
    let memory = SliceMemory::new(&mut bytes);
    assert_eq!(memory.read(3, 2), None);
    assert_eq!(memory.read(4, 1), None);
}

#[test]
fn an_address_and_length_that_overflow_are_refused() {
    let mut bytes = [7, 8, 9];
    let memory = SliceMemory::new(&mut bytes);
    assert_eq!(memory.read(u32::MAX, u32::MAX), None);
    assert_eq!(memory.read(2, u32::MAX), None);
}

#[test]
fn an_empty_read_is_fine_anywhere() {
    let mut bytes = [7];
    assert_eq!(SliceMemory::new(&mut bytes).read(0, 0), Some(&[][..]));
}

#[test]
fn writing_changes_the_bytes_and_past_the_end_is_refused() {
    let mut bytes = [0; 4];
    let mut memory = SliceMemory::new(&mut bytes);
    memory.write(2, 2).unwrap().copy_from_slice(&[5, 6]);
    assert!(memory.write(4, 2).is_none());
    assert_eq!(bytes, [0, 5, 6, 0]);
}
