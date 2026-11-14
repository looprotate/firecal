use super::{ANALYSIS_REQUEST_BYTES, AnalysisRequest, VERTEX_BYTES, parse_vertices};
use firecal_query::Vertex;

fn block() -> Vec<u8> {
    let mut bytes = Vec::new();
    for degrees in [-20.5_f64, -16.0, -62.25, -58.0] {
        bytes.extend_from_slice(&degrees.to_le_bytes());
    }
    for word in [276_u32, 12, 1, 2, 4096, 8192, 2001, 2024, 2025, 6, 3, 9000] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}

#[test]
fn a_block_is_read_field_by_field() {
    let request = AnalysisRequest::parse(&block()).unwrap();
    assert_eq!(request.series.first_month, 276);
    assert_eq!(request.series.tile_table, 4096);
    assert_eq!(
        (
            request.baseline_first,
            request.baseline_last,
            request.season_year,
            request.season_months,
            request.vertex_count,
            request.vertices
        ),
        (2001, 2024, 2025, 6, 3, 9000)
    );
}

#[test]
fn a_block_of_another_length_is_refused() {
    assert_eq!(
        block().len(),
        usize::try_from(ANALYSIS_REQUEST_BYTES).unwrap()
    );
    assert_eq!(AnalysisRequest::parse(&block()[..79]), None);
    assert_eq!(AnalysisRequest::parse(&[block(), vec![0]].concat()), None);
}

#[test]
fn corners_are_latitude_then_longitude() {
    let mut bytes = Vec::new();
    for degrees in [10.5_f64, -20.0, -3.0, 4.0] {
        bytes.extend_from_slice(&degrees.to_le_bytes());
    }
    assert_eq!(bytes.len(), 2 * usize::try_from(VERTEX_BYTES).unwrap());
    assert_eq!(
        parse_vertices(&bytes),
        Some(vec![
            Vertex {
                latitude: 10.5,
                longitude: -20.0
            },
            Vertex {
                latitude: -3.0,
                longitude: 4.0
            },
        ])
    );
    assert_eq!(parse_vertices(&bytes[..31]), None);
}
