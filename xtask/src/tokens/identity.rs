//! firecal's identity, worked out from six colours measured off the reference image Axel chose: the greys, the
//! four colours that mean something, the calendar ramp and the marks. Why each is what it is, and what moved:
//! `web/tokens/README.md`.

use super::color::{Rgb, bisect, contrast, mix, moved_to_contrast, moved_until};

/// The background of the reference image.
pub(crate) const PAPER: Rgb = Rgb([0xfa, 0xf9, 0xf5]);
/// The black arch.
pub(crate) const INK: Rgb = Rgb([0x14, 0x14, 0x13]);
/// The big circle: activity and heat.
pub(crate) const TERRACOTTA: Rgb = Rgb([0xd9, 0x77, 0x57]);
/// The circle on the terracotta: sigma and uncertainty.
pub(crate) const SKY: Rgb = Rgb([0x6a, 0x9b, 0xcc]);
/// The small circle: the provisional, and what needs attention.
pub(crate) const ROSE: Rgb = Rgb([0xc4, 0x66, 0x85]);
/// The half circle at the bottom: complete and healthy.
pub(crate) const OLIVE: Rgb = Rgb([0x78, 0x8c, 0x5d]);

/// WCAG 1.4.11: a shape that carries meaning, against what it sits on.
const GRAPHIC: f64 = 3.0;
/// WCAG 1.4.3: body text.
const TEXT: f64 = 4.5;
/// A calendar mark's edge against its own fill, so one of the two shows on any cell.
const MARK_OWN_EDGE: f64 = 7.0;
/// Steps of the ramp above an empty cell.
const RAMP_STEPS: u8 = 7;

/// The greys, each on the straight line in `OKLab` from paper to ink.
#[derive(Debug)]
pub(crate) struct Neutral {
    pub(crate) bg: Rgb,
    pub(crate) bg_deep: Rgb,
    pub(crate) surface: Rgb,
    pub(crate) raised: Rgb,
    pub(crate) line: Rgb,
    pub(crate) selection: Rgb,
    pub(crate) disabled: Rgb,
    pub(crate) text: Rgb,
    pub(crate) text_strong: Rgb,
    /// The lightest grey that still reaches 4.5:1 on `surface`, the darkest ground text sits on.
    pub(crate) text_secondary: Rgb,
}

impl Neutral {
    /// Every grey with its name, in the order the report prints them.
    pub(crate) const fn named(&self) -> [(&'static str, Rgb); 10] {
        [
            ("bg", self.bg),
            ("bg_deep", self.bg_deep),
            ("surface", self.surface),
            ("raised", self.raised),
            ("line", self.line),
            ("selection", self.selection),
            ("disabled", self.disabled),
            ("text", self.text),
            ("text_strong", self.text_strong),
            ("text_secondary", self.text_secondary),
        ]
    }
}

/// Every colour firecal draws with.
#[derive(Debug)]
pub(crate) struct Identity {
    pub(crate) neutral: Neutral,
    /// Terracotta, moved to 3:1 on paper.
    pub(crate) activity: Rgb,
    /// Sky, moved to 3:1 on paper.
    pub(crate) uncertainty: Rgb,
    /// Rose, moved to 3:1 on paper.
    pub(crate) attention: Rgb,
    /// Olive, moved to 3:1 on paper.
    pub(crate) healthy: Rgb,
    /// Rose as words: 4.5:1 on the surface.
    pub(crate) error: Rgb,
    /// Olive as words: 4.5:1 on the surface.
    pub(crate) added: Rgb,
    /// A calendar cell with no activity.
    pub(crate) empty_cell: Rgb,
    /// Seven steps from `activity` to ink, each the same contrast ratio darker than the one before.
    pub(crate) ramp: Vec<Rgb>,
    /// The provisional triangle: rose, lightened until it or its ink edge shows on every cell.
    pub(crate) provisional_fill: Rgb,
}

/// A neutral `t` of the way from paper to ink.
pub(crate) fn grey(t: f64) -> Rgb {
    mix(PAPER, INK, t)
}

impl Identity {
    /// Works the whole identity out of the six measured colours.
    pub(crate) fn derive() -> Result<Self, String> {
        let surface = grey(0.055);
        let neutral = Neutral {
            bg: grey(0.0),
            bg_deep: grey(0.035),
            surface,
            raised: grey(0.09),
            line: grey(0.13),
            selection: grey(0.17),
            disabled: grey(0.42),
            text: grey(0.92),
            text_strong: grey(1.0),
            text_secondary: grey(bisect(0.0, 1.0, |t| {
                contrast(grey(t), surface) >= TEXT + 0.05
            })),
        };
        let activity = moved_to_contrast(TERRACOTTA, PAPER, GRAPHIC)?;
        let empty_cell = grey(0.07);
        let ramp = ramp_from(activity);
        let provisional_fill = moved_until(
            ROSE,
            |fill| shows_on_every_cell(fill, empty_cell, &ramp),
            1.0,
        )?;
        Ok(Self {
            neutral,
            activity,
            uncertainty: moved_to_contrast(SKY, PAPER, GRAPHIC)?,
            attention: moved_to_contrast(ROSE, PAPER, GRAPHIC)?,
            healthy: moved_to_contrast(OLIVE, PAPER, GRAPHIC)?,
            error: moved_to_contrast(ROSE, surface, TEXT)?,
            added: moved_to_contrast(OLIVE, surface, TEXT)?,
            empty_cell,
            ramp,
            provisional_fill,
        })
    }
}

fn ramp_from(first: Rgb) -> Vec<Rgb> {
    let low = contrast(first, PAPER);
    let high = contrast(INK, PAPER);
    let ratio = (high / low).powf(1.0 / f64::from(RAMP_STEPS - 1));
    (0..RAMP_STEPS)
        .map(|step| {
            let target = low * ratio.powf(f64::from(step));
            let t = bisect(0.0, 1.0, |u| contrast(mix(first, INK, u), PAPER) >= target);
            mix(first, INK, t)
        })
        .collect()
}

fn shows_on_every_cell(fill: Rgb, empty_cell: Rgb, ramp: &[Rgb]) -> bool {
    contrast(fill, INK) >= MARK_OWN_EDGE
        && std::iter::once(&empty_cell)
            .chain(ramp)
            .all(|&cell| contrast(fill, cell).max(contrast(INK, cell)) >= GRAPHIC)
}
