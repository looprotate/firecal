//! Colour arithmetic, in the order the identity needs it: WCAG relative luminance and contrast, `OKLab`
//! (Ottosson 2020) there and back, straight lines in `OKLab`, and lightness moved until a check passes.
//!
//! The arithmetic is written out exactly as the first generator wrote it (`pow`, no fused multiply-add), because
//! a colour that sits on a threshold must land on the same side every time, on every machine.
#![expect(
    clippy::suboptimal_flops,
    clippy::imprecise_flops,
    reason = "plain multiply-add and pow keep every colour identical to the files already committed"
)]

use std::fmt;

/// A colour in `OKLab`: lightness, then the green-red and blue-yellow axes.
pub(crate) type Lab = [f64; 3];

/// An sRGB colour, one byte per channel; it prints as `#rrggbb`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rgb(pub(crate) [u8; 3]);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b] = self.0;
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

/// How far lightness moves per try when a colour must pass a check: small enough that the first pass is the
/// nearest one once written as hex.
const LIGHTNESS_STEP: f64 = 0.0005;
/// Halvings in [`bisect`]: far past the precision of a byte per channel.
const BISECTIONS: usize = 60;

fn linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn encoded(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "`encoded` clamps to 0..=1, so the rounded value is a byte"
)]
fn byte(linear_channel: f64) -> u8 {
    (encoded(linear_channel) * 255.0).round_ties_even() as u8
}

fn linear_channels(colour: Rgb) -> [f64; 3] {
    colour.0.map(|channel| linear(f64::from(channel) / 255.0))
}

/// WCAG relative luminance.
pub(crate) fn luminance(colour: Rgb) -> f64 {
    let [red, green, blue] = linear_channels(colour);
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

/// WCAG contrast ratio of two colours, at least 1.
pub(crate) fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (high, low) = if la >= lb { (la, lb) } else { (lb, la) };
    (high + 0.05) / (low + 0.05)
}

/// The `OKLab` coordinates of a colour.
#[expect(
    clippy::many_single_char_names,
    reason = "the names of the published formula, so it can be checked against it"
)]
pub(crate) fn oklab(colour: Rgb) -> Lab {
    let [r, g, b] = linear_channels(colour);
    let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).powf(1.0 / 3.0);
    let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).powf(1.0 / 3.0);
    let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).powf(1.0 / 3.0);
    [
        0.210_454_255_3 * l + 0.793_617_785_0 * m - 0.004_072_046_8 * s,
        1.977_998_495_1 * l - 2.428_592_205_0 * m + 0.450_593_709_9 * s,
        0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766_0 * s,
    ]
}

/// The sRGB colour of `OKLab` coordinates, each channel clipped to the gamut.
#[expect(
    clippy::many_single_char_names,
    reason = "the names of the published formula, so it can be checked against it"
)]
pub(crate) fn rgb_of_oklab([lightness, a, b]: Lab) -> Rgb {
    let l = (lightness + 0.396_337_777_4 * a + 0.215_803_757_3 * b).powf(3.0);
    let m = (lightness - 0.105_561_345_8 * a - 0.063_854_172_8 * b).powf(3.0);
    let s = (lightness - 0.089_484_177_5 * a - 1.291_485_548_0 * b).powf(3.0);
    Rgb([
        byte(4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s),
        byte(-1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s),
        byte(-0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s),
    ])
}

/// The colour a fraction `t` of the way from `a` to `b`, on a straight line in `OKLab`.
pub(crate) fn mix(a: Rgb, b: Rgb, t: f64) -> Rgb {
    let [l1, a1, b1] = oklab(a);
    let [l2, a2, b2] = oklab(b);
    rgb_of_oklab([l1 + (l2 - l1) * t, a1 + (a2 - a1) * t, b1 + (b2 - b1) * t])
}

fn with_lightness(colour: Rgb, lightness: f64) -> Rgb {
    let [_, a, b] = oklab(colour);
    rgb_of_oklab([lightness, a, b])
}

/// `colour` with its `OKLab` lightness moved, lighter for a positive `direction` and darker for a negative one,
/// until `passes` holds. Hue and chroma stay those that were measured.
pub(crate) fn moved_until(
    colour: Rgb,
    passes: impl Fn(Rgb) -> bool,
    direction: f64,
) -> Result<Rgb, String> {
    let [mut lightness, _, _] = oklab(colour);
    let mut moved = colour;
    while !passes(moved) {
        lightness += direction * LIGHTNESS_STEP;
        if !(0.0 < lightness && lightness < 1.0) {
            return Err(format!("{colour} cannot be moved far enough"));
        }
        moved = with_lightness(colour, lightness);
    }
    Ok(moved)
}

/// `colour` if it reaches `target` against `ground`; otherwise the nearest lightness, away from `ground`, that
/// does.
pub(crate) fn moved_to_contrast(colour: Rgb, ground: Rgb, target: f64) -> Result<Rgb, String> {
    let away = if luminance(colour) < luminance(ground) {
        -1.0
    } else {
        1.0
    };
    moved_until(colour, |c| contrast(c, ground) >= target, away)
}

/// The smallest `t` in `low..=high` for which `reaches(t)` holds, given that it holds from some point on.
pub(crate) fn bisect(mut low: f64, mut high: f64, reaches: impl Fn(f64) -> bool) -> f64 {
    for _ in 0..BISECTIONS {
        let middle = f64::midpoint(low, high);
        if reaches(middle) {
            high = middle;
        } else {
            low = middle;
        }
    }
    high
}
