//! Per-plane fitted parameters and explicit scientific-validation status.

use super::encoding::EnergyPlane;
use crate::SigmaModel;

pub(super) fn energy_json(plane: Option<&EnergyPlane>) -> String {
    plane.map_or_else(||"null".to_owned(),|plane|{
        let knots=plane.calibration.knots().iter().map(|k|format!("[{},{}]",k.viirs_megawatts(),k.modis_megawatts())).collect::<Vec<_>>().join(",");
        format!(concat!("{{\"version\":\"frp-quantile-v1\",\"energy_units\":\"megawatts_frp_sum\",",
            "\"sample_count\":{},\"knots\":[{}],\"clipped_cell_months\":{},\"tail\":\"clamp\",",
            "\"real_overlap_validation\":\"not_validated\"}}"),plane.calibration.sample_count(),knots,plane.clipped)
    })
}

pub(super) fn sigma_json(model: Option<&SigmaModel>) -> String {
    model.map_or_else(|| "null".to_owned(), |model| format!(
        concat!(
            "{{\"version\":\"count-residual-rms-v1\",\"residual_rms_counts\":{},",
            "\"measurement_floor_counts\":{},\"floor_applies_to\":\"active_cells\",\"first_training_month\":{},\"last_training_month\":{},",
            "\"base\":\"max(rms,sqrt(mapped_count),active_floor)\",",
            "\"temporal_factor\":\"1+distance/training_span\",",
            "\"range_factor\":\"1+excess/max_training_viirs\",",
            "\"real_holdout_coverage\":\"not_validated\"}}"
        ), model.residual_rms(), model.measurement_floor(), model.first_month().index(), model.last_month().index()
    ))
}
