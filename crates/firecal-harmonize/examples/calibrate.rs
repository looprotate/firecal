//! Run the hand-computed count calibration example: `cargo run -p firecal-harmonize --example calibrate`.

use firecal_harmonize::fit;
use firecal_types::TileId;

fn main() -> Result<(), String> {
    let tile = TileId::new(0, 0).map_err(|error| format!("{error:?}"))?;
    let calibration =
        fit(tile, &[0, 2, 4, 8], &[0, 4, 8, 16]).map_err(|error| format!("{error:?}"))?;
    println!("viirs_count,modis_equivalent_count");
    for count in [0, 2, 4, 6, 8, 12, 16, 20] {
        let mapped = calibration
            .map_count(count)
            .map_err(|error| format!("{error:?}"))?;
        println!("{count},{mapped}");
    }
    Ok(())
}
