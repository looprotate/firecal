//! Each satellite-month belongs to one worker per batch, preserving row addition order.

use super::{Groups, Key};
use firecal_ingest::{Aggregator, Hotspot};
use std::{collections::BTreeMap, num::NonZeroUsize, thread};

type Job = (Key, Aggregator, Vec<Hotspot>);

pub(super) fn add(
    groups: &mut Groups,
    records: Vec<Hotspot>,
    workers: NonZeroUsize,
) -> Result<(), String> {
    let mut by_group: BTreeMap<Key, Vec<Hotspot>> = BTreeMap::new();
    for record in records {
        by_group
            .entry((record.satellite, record.month))
            .or_default()
            .push(record);
    }
    let worker_count = workers.get().min(by_group.len());
    if worker_count == 0 {
        return Ok(());
    }
    let mut buckets: Vec<Vec<Job>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (index, (key, records)) in by_group.into_iter().enumerate() {
        let aggregator = groups
            .remove(&key)
            .unwrap_or_else(|| Aggregator::new(key.0, key.1));
        buckets
            .get_mut(index % worker_count)
            .ok_or("invalid worker bucket")?
            .push((key, aggregator, records));
    }
    let finished = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(buckets.len());
        for bucket in buckets {
            handles.push(scope.spawn(move || process(bucket)));
        }
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .map_err(|_| String::from("ingest worker panicked"))?
            })
            .collect::<Result<Vec<_>, String>>()
    })?;
    for bucket in finished {
        for (key, aggregator) in bucket {
            groups.insert(key, aggregator);
        }
    }
    Ok(())
}

fn process(jobs: Vec<Job>) -> Result<Vec<(Key, Aggregator)>, String> {
    let mut finished = Vec::with_capacity(jobs.len());
    for (key, mut aggregator, records) in jobs {
        for record in records {
            aggregator
                .add(&record)
                .map_err(|error| format!("{key:?}: {error:?}"))?;
        }
        finished.push((key, aggregator));
    }
    Ok(finished)
}
