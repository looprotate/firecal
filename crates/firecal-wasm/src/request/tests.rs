use super::{REQUEST_BYTES, Request, TILE_ENTRY_BYTES, TileEntry};

fn block() -> Vec<u8> {
    let mut bytes = Vec::new();
    for degrees in [-20.5_f64, -16.0, -62.25, -58.0] {
        bytes.extend_from_slice(&degrees.to_le_bytes());
    }
    for word in [276_u32, 12, 1, 2, 4096, 8192] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}

#[test]
fn a_request_block_is_read_field_by_field() {
    let request = Request::parse(&block()).unwrap();
    assert_eq!(
        request,
        Request {
            south: -20.5,
            north: -16.0,
            west: -62.25,
            east: -58.0,
            first_month: 276,
            month_count: 12,
            plane: 1,
            tile_count: 2,
            tile_table: 4096,
            output: 8192,
        }
    );
    assert_eq!(block().len(), usize::try_from(REQUEST_BYTES).unwrap());
}

#[test]
fn a_request_block_of_the_wrong_length_is_refused() {
    let mut bytes = block();
    bytes.push(0);
    assert_eq!(Request::parse(&bytes), None);
    assert_eq!(Request::parse(&block()[..55]), None);
}

#[test]
fn a_tile_table_is_read_entry_by_entry() {
    let mut bytes = Vec::new();
    for word in [7_u32, 18, 100, 3216, 8, 18, 4000, 3216] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    let table = TileEntry::parse_table(&bytes).unwrap();
    assert_eq!(table.len(), 2);
    assert_eq!(
        table[1],
        TileEntry {
            row: 8,
            col: 18,
            address: 4000,
            len: 3216
        }
    );
    assert_eq!(bytes.len(), 2 * usize::try_from(TILE_ENTRY_BYTES).unwrap());
}

#[test]
fn a_tile_table_that_is_not_whole_entries_is_refused() {
    assert_eq!(TileEntry::parse_table(&[0; 17]), None);
}

#[test]
fn an_empty_tile_table_has_no_entries() {
    assert_eq!(TileEntry::parse_table(&[]), Some(Vec::new()));
}
