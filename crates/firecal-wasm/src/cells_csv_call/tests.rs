use super::run_cells_csv;
use crate::memory::SliceMemory;
use crate::series_call::tests::{COUNT, Call, ENERGY, REQUEST_LEN};
use crate::{INVALID, STATUS_EMPTY_AREA, STATUS_OK, STATUS_OUTPUT_TOO_SMALL};

// The series fixture: tile (7, 18), two months from November 2000; cell (39, 0) is row 319, column 720 and
// (39, 1) is column 721. Area latitude 10.0 to 10.25, longitude 0.0 to 0.5 holds both.
const COUNT_CSV: &str = "year,month,cell_lat,cell_lon,value,sigma\n\
    2000,11,10.125,0.125,5,3\n\
    2000,11,10.125,0.375,3,4\n\
    2000,12,10.125,0.125,7,12\n\
    2000,12,10.125,0.375,0,5\n";

fn call(plane: u32) -> Call {
    Call {
        area: [10.0, 10.25, 0.0, 0.5],
        ..Call::one_cell(plane)
    }
}

/// Runs the call with a text buffer of `room` bytes after the length, returning the status and the text.
fn run(call: &Call, room: u32) -> (u32, Option<String>) {
    let (mut image, request, _) = call.image();
    let buffer = u32::try_from(image.len() + 1).unwrap();
    image.resize(image.len() + 4 + usize::try_from(room).unwrap(), 0xAA);
    let status = run_cells_csv(
        &mut SliceMemory::new(&mut image),
        request,
        REQUEST_LEN,
        buffer,
        4 + room,
    );
    let start = usize::try_from(buffer - 1).unwrap();
    let length = u32::from_le_bytes(image[start..start + 4].try_into().unwrap());
    let text = (status == STATUS_OK).then(|| {
        String::from_utf8(image[start + 4..start + 4 + usize::try_from(length).unwrap()].to_vec())
            .unwrap()
    });
    (status, text)
}

#[test]
fn the_text_follows_its_length() {
    assert_eq!(
        run(&call(COUNT), 512),
        (STATUS_OK, Some(COUNT_CSV.to_owned()))
    );
}

#[test]
fn energy_leaves_the_sigma_empty() {
    let (status, text) = run(&call(ENERGY), 512);
    assert_eq!(status, STATUS_OK);
    assert!(text.unwrap().contains("2000,11,10.125,0.125,40,\n"));
}

#[test]
fn a_buffer_exactly_the_size_of_the_text_fits_and_one_byte_less_does_not() {
    let size = u32::try_from(COUNT_CSV.len()).unwrap();
    assert_eq!(run(&call(COUNT), size).0, STATUS_OK);
    assert_eq!(run(&call(COUNT), size - 1).0, STATUS_OUTPUT_TOO_SMALL);
}

#[test]
fn nothing_is_written_when_the_status_is_not_ok() {
    let call = Call {
        area: [10.0, 10.0, 0.0, 0.5],
        ..call(COUNT)
    };
    assert_eq!(run(&call, 64).0, STATUS_EMPTY_AREA);
}

#[test]
fn a_buffer_that_cannot_hold_the_length_or_is_past_the_memory_is_invalid() {
    let (mut image, request, _) = call(COUNT).image();
    let mut memory = SliceMemory::new(&mut image);
    assert_eq!(
        run_cells_csv(&mut memory, request, REQUEST_LEN, 1, 3),
        INVALID
    );
    assert_eq!(
        run_cells_csv(&mut memory, request, REQUEST_LEN, 1 << 30, 64),
        INVALID
    );
    assert_eq!(
        run_cells_csv(&mut memory, request, REQUEST_LEN - 1, 1, 64),
        INVALID
    );
}
