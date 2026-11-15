//! `cargo xtask tokens`: writes firecal's identity files from the six colours measured off the reference image,
//! or, with `--report`, prints every derived colour and the check it was moved for.
//!
//! The files are committed and never edited by hand; a test fails when they differ from what this writes.

mod color;
mod files;
mod identity;
mod report;
#[cfg(test)]
mod tests;

use identity::Identity;
use std::fs;

/// Writes the text of one file of the identity.
type Writer = fn(&Identity) -> String;

/// Every file the identity is written as, with what writes it.
const TARGETS: [(&str, Writer); 3] = [
    ("web/tokens/tokens.css", files::tokens_css),
    ("web/tokens/logo.svg", files::logo_svg),
    ("pitch/figures/title-card.svg", files::title_card_svg),
];

/// Writes every file of the identity.
pub(crate) fn write() -> Result<(), String> {
    let id = Identity::derive()?;
    for (path, make) in TARGETS {
        fs::write(path, make(&id)).map_err(|error| format!("cannot write {path}: {error}"))?;
        println!("tokens: {path}");
    }
    Ok(())
}

/// Prints the report of every derived colour.
pub(crate) fn print_report() -> Result<(), String> {
    println!("{}", report::report(&Identity::derive()?));
    Ok(())
}
