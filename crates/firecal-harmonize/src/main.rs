//! Count, estimated-sigma and FRP development tile writer. All filesystem access stays in this binary.

use firecal_harmonize::{
    CountPlan, apply_with_energy, fit_energy_aggregates, fit_sigma_aggregates,
};
use firecal_types::{AggregateView, MonthIndex, Satellite, TileId};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: firecal-harmonize build --complete-standard-tile OUTPUT ROW COL FIRST_MONTH MONTH_COUNT MODIS_CODE VIIRS_CODE TRAIN_THROUGH YYYY-MM-DD FILE.fca...\nThe flag asserts complete science-quality coverage of the entire tile in every supplied satellite-month; do not use country-clipped data. Output is development-only: count, estimated count sigma and calibrated FRP are available.";

struct Options {
    output: PathBuf,
    plan: CountPlan,
    cutoff: MonthIndex,
    date: String,
    files: Vec<PathBuf>,
}

fn number<T: std::str::FromStr>(
    arguments: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<T, String> {
    arguments
        .next()
        .ok_or_else(|| format!("missing {name}\n{USAGE}"))?
        .into_string()
        .map_err(|_| format!("{name} is not text"))?
        .parse()
        .map_err(|_| format!("invalid {name}"))
}

impl Options {
    fn parse() -> Result<Self, String> {
        let mut arguments = std::env::args_os().skip(1);
        if arguments.next().as_deref() != Some(std::ffi::OsStr::new("build"))
            || arguments.next().as_deref() != Some(std::ffi::OsStr::new("--complete-standard-tile"))
        {
            return Err(USAGE.to_owned());
        }
        let output = PathBuf::from(arguments.next().ok_or(USAGE)?);
        let row = number(&mut arguments, "tile row")?;
        let col = number(&mut arguments, "tile column")?;
        let first = MonthIndex::from_index(number(&mut arguments, "first month")?);
        let months = number(&mut arguments, "month count")?;
        let modis = Satellite::from_code(number(&mut arguments, "MODIS satellite code")?)
            .map_err(|error| format!("{error:?}"))?;
        let viirs = Satellite::from_code(number(&mut arguments, "VIIRS satellite code")?)
            .map_err(|error| format!("{error:?}"))?;
        let cutoff = MonthIndex::from_index(number(&mut arguments, "last training month")?);
        let date = arguments
            .next()
            .ok_or("missing build date")?
            .into_string()
            .map_err(|_| "build date is not text")?;
        let tile = TileId::new(row, col).map_err(|error| format!("{error:?}"))?;
        let plan = CountPlan::new(tile, first, months, modis, viirs)
            .map_err(|error| format!("{error:?}"))?;
        let files = arguments.map(PathBuf::from).collect::<Vec<_>>();
        if files.is_empty() {
            return Err("no FCA1 files supplied".to_owned());
        }
        Ok(Self {
            output,
            plan,
            cutoff,
            date,
            files,
        })
    }
}

fn run() -> Result<(), String> {
    let options = Options::parse()?;
    let bytes = options
        .files
        .iter()
        .map(|path| std::fs::read(path).map_err(|error| format!("{}: {error}", path.display())))
        .collect::<Result<Vec<_>, _>>()?;
    let aggregates = bytes
        .iter()
        .map(|bytes| {
            AggregateView::parse(bytes).map_err(|error| format!("invalid FCA1 input: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let model = fit_sigma_aggregates(&options.plan, &aggregates, options.cutoff, 1)
        .map_err(|error| format!("fit: {error:?}"))?;
    let energy = fit_energy_aggregates(&options.plan, &aggregates, options.cutoff)
        .map_err(|error| format!("fit FRP: {error:?}"))?;
    let tile = apply_with_energy(&model, &energy, &options.plan, &aggregates)
        .map_err(|error| format!("apply: {error:?}"))?;
    let index = tile
        .index_json(model.calibration(), options.cutoff, &options.date)
        .map_err(|error| format!("index: {error:?}"))?;
    let encoded = if tile.is_empty() {
        None
    } else {
        Some(
            tile.encode()
                .map_err(|error| format!("encode: {error:?}"))?,
        )
    };
    // A fresh directory protects previous outputs and all source files from replacement.
    std::fs::create_dir(&options.output).map_err(|error| {
        format!(
            "cannot create fresh output {}: {error}",
            options.output.display()
        )
    })?;
    if let Some(encoded) = encoded {
        let name = format!(
            "t_{}_{}.bin",
            options.plan.tile().row(),
            options.plan.tile().col()
        );
        std::fs::write(options.output.join(name), encoded)
            .map_err(|error| format!("write tile: {error}"))?;
    }
    // An incomplete write has no index and cannot be mistaken for a finished build.
    std::fs::write(options.output.join("index.json"), index)
        .map_err(|error| format!("write index: {error}"))?;
    println!(
        "Count, estimated count sigma and FRP development build written to {}. Real overlap accuracy and sigma coverage remain unvalidated.",
        options.output.display()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
