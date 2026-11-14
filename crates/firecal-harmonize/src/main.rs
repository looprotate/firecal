//! Count, estimated-sigma and FRP development tile writer. All filesystem access stays in this binary.

use firecal_harmonize::{
    CountPlan, apply_with_energy, fit_energy_aggregates, fit_sigma_aggregates,
};
use firecal_types::AggregateView;
use std::io::Write;
use std::process::ExitCode;

mod cli;
use cli::{Action, Options};

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
    match &options.action {
        Action::Build { plan, date } => build(&options, plan, date, &aggregates),
        Action::Report { plans } => report(&options, plans, &aggregates),
    }
}

fn build(
    options: &Options,
    plan: &CountPlan,
    date: &str,
    aggregates: &[AggregateView<'_>],
) -> Result<(), String> {
    let model = fit_sigma_aggregates(plan, aggregates, options.cutoff, 1)
        .map_err(|error| format!("fit: {error:?}"))?;
    let energy = fit_energy_aggregates(plan, aggregates, options.cutoff)
        .map_err(|error| format!("fit FRP: {error:?}"))?;
    let tile = apply_with_energy(&model, &energy, plan, aggregates)
        .map_err(|error| format!("apply: {error:?}"))?;
    let index = tile
        .index_json(model.calibration(), options.cutoff, date)
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
        let name = format!("t_{}_{}.bin", plan.tile().row(), plan.tile().col());
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

fn report(
    options: &Options,
    plans: &[CountPlan],
    aggregates: &[AggregateView<'_>],
) -> Result<(), String> {
    let report = firecal_harmonize::overlap_report(plans, aggregates, options.cutoff)
        .map_err(|error| format!("report: {error:?}"))?;
    let csv = report.csv();
    let mut file = std::fs::File::create_new(&options.output).map_err(|error| {
        format!(
            "cannot create fresh CSV {}: {error}",
            options.output.display()
        )
    })?;
    file.write_all(csv.as_bytes())
        .map_err(|error| format!("write CSV: {error}"))?;
    println!(
        "Wrote {} paired tile-month rows to {}. Training rows are not independent validation.",
        report.row_count(),
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
