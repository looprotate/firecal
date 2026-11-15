//! What `cargo xtask tokens --report` prints: every derived colour, what it came from, and the check it was moved
//! for. `web/tokens/README.md` records it.

use super::color::{Rgb, contrast, oklab};
use super::identity::{INK, Identity, OLIVE, PAPER, ROSE, SKY, TERRACOTTA};

/// The report, one line per colour.
pub(crate) fn report(id: &Identity) -> String {
    let surface = id.neutral.surface;
    let moved: [(&str, Rgb, Rgb, Rgb); 7] = [
        ("activity", TERRACOTTA, id.activity, PAPER),
        ("uncertainty", SKY, id.uncertainty, PAPER),
        ("attention", ROSE, id.attention, PAPER),
        ("healthy", OLIVE, id.healthy, PAPER),
        ("error", ROSE, id.error, surface),
        ("added", OLIVE, id.added, surface),
        ("provisional-fill", ROSE, id.provisional_fill, INK),
    ];
    let mut rows: Vec<String> = moved
        .iter()
        .map(|&(name, measured, used, ground)| {
            let [measured_l, _, _] = oklab(measured);
            let [used_l, _, _] = oklab(used);
            format!(
                "{name:17} {measured} -> {used}  {:.2} -> {:.2}:1  OKLab L {:+.3}",
                contrast(measured, ground),
                contrast(used, ground),
                used_l - measured_l
            )
        })
        .collect();
    rows.push(String::new());
    rows.extend(id.neutral.named().iter().map(|&(name, value)| {
        format!(
            "{name:17} {value}  {:.2}:1 on paper",
            contrast(value, PAPER)
        )
    }));
    rows.push(String::new());
    rows.extend(
        std::iter::once(&id.empty_cell)
            .chain(&id.ramp)
            .enumerate()
            .map(|(step, &colour)| {
                let [lightness, _, _] = oklab(colour);
                format!(
                    "ramp-{step:<12} {colour}  {:.2}:1 on paper  L {lightness:.3}",
                    contrast(colour, PAPER)
                )
            }),
    );
    rows.join("\n")
}
