use super::{Groups, batch, increment};
use firecal_ingest::{Aggregator, Hotspot};
use firecal_types::{MonthIndex, Satellite, aggregate_len};
use std::num::NonZeroUsize;

#[test]
fn batched_workers_match_a_simple_serial_oracle_with_sensitive_frp_sums() {
    let mut records = Vec::new();
    for day in 0..3000_u32 {
        for satellite in [Satellite::Terra, Satellite::Aqua, Satellite::SuomiNpp] {
            records.push(Hotspot {
                latitude: -33.5,
                longitude: -70.25,
                month: MonthIndex::from_index(288),
                day: 1,
                satellite,
                frp_megawatts: if day == 0 { 16_777_216.0 } else { 1.0 },
                daytime: day.is_multiple_of(2),
            });
        }
    }
    let mut serial = Groups::new();
    for record in &records {
        serial
            .entry((record.satellite, record.month))
            .or_insert_with(|| Aggregator::new(record.satellite, record.month))
            .add(record)
            .unwrap();
    }
    for workers in [1, 2, 8] {
        let mut parallel = Groups::new();
        for chunk in records.chunks(super::BATCH_ROWS) {
            batch::add(
                &mut parallel,
                chunk.to_vec(),
                NonZeroUsize::new(workers).unwrap(),
            )
            .unwrap();
        }
        assert_eq!(parallel.len(), serial.len());
        for (key, expected) in &serial {
            let size = aggregate_len(expected.records().unwrap().len()).unwrap();
            let mut left = vec![0; size];
            let mut right = vec![0; size];
            expected.write(&mut left).unwrap();
            parallel.get(key).unwrap().write(&mut right).unwrap();
            assert_eq!(left, right, "{key:?}, {workers} workers");
        }
    }
}

#[test]
fn empty_batch_is_a_noop_and_counter_overflow_is_an_error() {
    let mut groups = Groups::new();
    batch::add(&mut groups, Vec::new(), NonZeroUsize::MIN).unwrap();
    assert!(groups.is_empty());
    let mut counter = u64::MAX;
    assert!(increment(&mut counter).is_err());
    assert_eq!(counter, u64::MAX);
}
