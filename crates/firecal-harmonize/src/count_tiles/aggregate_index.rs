//! Validated satellite-month lookup and tile-local sparse-to-dense conversion.

use super::{CountPlan, TileBuildError};
use firecal_types::{AggregateView, MonthIndex, Satellite, TILE_CELLS, TILE_SIDE, TileId};
use std::collections::BTreeMap;

pub(super) struct AggregateIndex<'a> {
    files: BTreeMap<(MonthIndex, Satellite), AggregateView<'a>>,
}

impl<'a> AggregateIndex<'a> {
    pub(super) fn new(
        plan: &CountPlan,
        aggregates: &[AggregateView<'a>],
    ) -> Result<Self, TileBuildError> {
        let mut files = BTreeMap::new();
        for &view in aggregates {
            if view.satellite() != plan.modis && view.satellite() != plan.viirs {
                return Err(TileBuildError::UnexpectedSatellite);
            }
            if view
                .month()
                .index()
                .checked_sub(plan.first.index())
                .is_none_or(|offset| offset >= plan.months)
            {
                return Err(TileBuildError::MonthOutsideRange);
            }
            if files
                .insert((view.month(), view.satellite()), view)
                .is_some()
            {
                return Err(TileBuildError::DuplicateAggregate);
            }
        }
        Ok(Self { files })
    }
    pub(super) fn paired_months(&self, plan: &CountPlan) -> Vec<MonthIndex> {
        self.files
            .keys()
            .filter_map(|&(month, satellite)| {
                (satellite == plan.modis
                    && month.index() >= 144
                    && self.files.contains_key(&(month, plan.viirs)))
                .then_some(month)
            })
            .collect()
    }

    pub(super) fn counts(
        &self,
        plan: &CountPlan,
        month: MonthIndex,
        satellite: Satellite,
    ) -> Option<Vec<u32>> {
        let view = self.files.get(&(month, satellite))?;
        let mut counts = vec![0; TILE_CELLS];
        for record in view
            .records()
            .filter(|record| TileId::of_cell(record.cell) == plan.tile)
        {
            let offset =
                (record.cell.row() % TILE_SIDE) * TILE_SIDE + record.cell.col() % TILE_SIDE;
            if let Some(slot) = usize::try_from(offset)
                .ok()
                .and_then(|offset| counts.get_mut(offset))
            {
                *slot = record.count;
            }
        }
        Some(counts)
    }
}
