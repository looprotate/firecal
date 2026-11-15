use super::TARGETS;
use super::color::{Rgb, contrast, mix, oklab, rgb_of_oklab};
use super::identity::{INK, Identity, OLIVE, PAPER, ROSE, SKY, TERRACOTTA};

const MEASURED: [Rgb; 6] = [PAPER, INK, TERRACOTTA, SKY, ROSE, OLIVE];

fn hex(colour: Rgb) -> String {
    colour.to_string()
}

fn derived() -> Identity {
    Identity::derive().unwrap()
}

#[test]
fn a_colour_prints_as_lowercase_hex() {
    assert_eq!(hex(Rgb([0xd9, 0x77, 0x05])), "#d97705");
}

#[test]
fn every_measured_colour_survives_a_trip_through_oklab() {
    for colour in MEASURED {
        assert_eq!(rgb_of_oklab(oklab(colour)), colour, "{colour}");
    }
}

#[test]
fn a_mix_starts_at_one_end_and_stops_at_the_other() {
    assert_eq!(mix(PAPER, INK, 0.0), PAPER);
    assert_eq!(mix(PAPER, INK, 1.0), INK);
}

#[test]
fn contrast_is_one_against_itself_and_the_same_either_way() {
    assert!((contrast(ROSE, ROSE) - 1.0).abs() < 1e-12);
    assert!((contrast(PAPER, INK) - contrast(INK, PAPER)).abs() < 1e-12);
}

#[test]
fn white_on_black_is_twenty_one_to_one() {
    let ratio = contrast(Rgb([0xff, 0xff, 0xff]), Rgb([0, 0, 0]));
    assert!((ratio - 21.0).abs() < 1e-9, "{ratio}");
}

/// The values `web/tokens/README.md` gives for every colour that moved, and for the greys and the ramp.
#[test]
fn the_derived_colours_are_the_ones_the_readme_documents() {
    let id = derived();
    let moved = [
        id.activity,
        id.uncertainty,
        id.attention,
        id.healthy,
        id.error,
        id.added,
        id.provisional_fill,
    ];
    assert_eq!(
        moved.map(hex),
        [
            "#d77656", "#6494c5", "#c46685", "#788c5d", "#a74c6c", "#5d7043", "#e685a4"
        ]
    );
    assert_eq!(hex(id.neutral.text_secondary), "#6a6967");
    assert_eq!(hex(id.empty_cell), "#e8e7e3");
    assert_eq!(
        id.ramp.iter().copied().map(hex).collect::<Vec<_>>(),
        [
            "#d77656", "#b5654b", "#945641", "#774737", "#5a382d", "#3b2922", "#141413"
        ]
    );
}

#[test]
fn every_ramp_step_stands_three_to_one_off_the_page_and_darkens() {
    let id = derived();
    let ratios: Vec<f64> = id.ramp.iter().map(|&step| contrast(step, PAPER)).collect();
    assert!(ratios.iter().all(|&ratio| ratio >= 3.0), "{ratios:?}");
    assert!(
        ratios
            .windows(2)
            .all(|pair| matches!(pair, [a, b] if b > a)),
        "{ratios:?}"
    );
}

#[test]
fn the_committed_files_are_what_the_generator_writes() {
    let id = derived();
    let committed = [
        include_str!("../../../web/tokens/tokens.css"),
        include_str!("../../../web/tokens/logo.svg"),
        include_str!("../../../pitch/figures/title-card.svg"),
    ];
    for ((path, make), file) in TARGETS.iter().zip(committed) {
        assert_eq!(make(&id), file, "{path} differs: run `cargo xtask tokens`");
    }
}
