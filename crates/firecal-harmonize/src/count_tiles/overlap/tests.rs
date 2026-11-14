//! Independent monthly agreement and honest reporting of mismatches/missing coverage.

use super::*;
use firecal_types::{AggregateRecord, CellIndex, aggregate_len, write_aggregate};

fn plan(tile: TileId) -> CountPlan {
    CountPlan::new(
        tile,
        MonthIndex::from_index(276),
        3,
        Satellite::Terra,
        Satellite::SuomiNpp,
    )
    .unwrap()
}
fn file(satellite: Satellite, month: u16, counts: &[u32]) -> Vec<u8> {
    let records: Vec<_> = counts
        .iter()
        .zip(0_u32..)
        .map(|(&count, col)| AggregateRecord {
            cell: CellIndex::new(0, col).unwrap(),
            count,
            frp_megawatts: 0.0,
            day_count: 0,
        })
        .collect();
    let mut bytes = vec![0; aggregate_len(records.len()).unwrap()];
    write_aggregate(
        satellite,
        MonthIndex::from_index(month),
        &records,
        &mut bytes,
    )
    .unwrap();
    bytes
}

#[test]
fn report_golden_training_and_held_out_months_match_with_explicit_tolerance() {
    let tile = TileId::new(0, 0).unwrap();
    let p = plan(tile);
    let files = [
        file(Satellite::Terra, 276, &[2, 4]),
        file(Satellite::SuomiNpp, 276, &[4, 8]),
        file(Satellite::Terra, 277, &[1, 3]),
        file(Satellite::SuomiNpp, 277, &[2, 6]),
        file(Satellite::SuomiNpp, 278, &[4]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|bytes| AggregateView::parse(bytes).unwrap())
        .collect();
    let report = overlap_report(&[p], &views, p.first_month()).unwrap();
    assert_eq!(report.row_count(), 2);
    // Every mapped count is an exact half integer in this hand fixture. 1e-12
    // bounds arithmetic roundoff; it is not an empirical sensor-error allowance.
    for (row, reference) in report.rows.iter().zip([6.0, 4.0]) {
        assert!((row.mapped - reference).abs() <= 1e-12);
    }
    assert_eq!(
        report.csv(),
        concat!(
            "tile_row,tile_col,year,month,month_index,modis_satellite,viirs_satellite,split,raw_modis_count,raw_viirs_count,harmonized_viirs_count\n",
            "0,0,2023,1,276,0,2,training,6,12,6.000000\n",
            "0,0,2023,2,277,0,2,held_out,4,8,4.000000\n"
        )
    );
    // Input and tile-plan ordering cannot alter the report bytes.
    let mut reversed = views.clone();
    reversed.reverse();
    let extra = plan(TileId::new(1, 0).unwrap());
    assert_eq!(
        overlap_report(&[extra, p], &reversed, p.first_month())
            .unwrap()
            .csv(),
        overlap_report(&[p, extra], &views, p.first_month())
            .unwrap()
            .csv()
    );
}

#[test]
fn report_maps_viirs_even_when_modis_exists_and_preserves_disagreement() {
    let p = plan(TileId::new(0, 0).unwrap());
    let files = [
        file(Satellite::Terra, 276, &[2, 4]),
        file(Satellite::SuomiNpp, 276, &[4, 8]),
        file(Satellite::Terra, 277, &[]),
        file(Satellite::SuomiNpp, 277, &[8]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|bytes| AggregateView::parse(bytes).unwrap())
        .collect();
    let report = overlap_report(&[p], &views, p.first_month()).unwrap();
    assert!(report.csv().contains("held_out,0,8,4.000000\n"));
    // This must not accidentally use apply(), which would return the known MODIS zero.
    assert!((report.rows[1].mapped - 4.0).abs() < 1e-12);
}

#[test]
fn report_rejects_empty_duplicate_regions_and_unknown_training() {
    let p = plan(TileId::new(0, 0).unwrap());
    assert!(matches!(
        overlap_report(&[], &[], p.first_month()),
        Err(ReportError::EmptyRegion)
    ));
    assert!(matches!(
        overlap_report(&[p, p], &[], p.first_month()),
        Err(ReportError::DuplicateTile)
    ));
    assert!(matches!(
        overlap_report(&[p], &[], p.first_month()),
        Err(ReportError::Tile(TileBuildError::NoTrainingOverlap))
    ));
}

#[test]
fn report_keeps_raw_u32_counts_above_tile_encoding_limit() {
    let p = plan(TileId::new(0, 0).unwrap());
    let files = [
        file(Satellite::Terra, 276, &[u32::MAX, u32::MAX]),
        file(Satellite::SuomiNpp, 276, &[u32::MAX, u32::MAX]),
    ];
    let views: Vec<_> = files
        .iter()
        .map(|bytes| AggregateView::parse(bytes).unwrap())
        .collect();
    let report = overlap_report(&[p], &views, p.first_month()).unwrap();
    assert!(
        report
            .csv()
            .contains("training,8589934590,8589934590,8589934590.000000\n")
    );
}
