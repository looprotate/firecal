# Tokens

`tokens.css` and `logo.svg`, and the pitch's `figures/title-card.svg`, are written by `cargo xtask tokens` (the
generator is `xtask/src/tokens/`), then committed. Nothing here is edited by hand: change the generator and run it
again; a test in `xtask` fails when a committed file differs from what it writes. `cargo xtask tokens --report`
prints every derived color and the check it was moved for, the numbers in the tables below.

## Where the colors come from

firecal's identity is a flat composition of geometric shapes on warm paper. Six colors were measured off the
reference image Axel chose, as pixel samples:

| Measured | Value | In the reference | Means |
|---|---|---|---|
| paper | `#faf9f5` | the background | the page |
| ink | `#141413` | the black arch | text, and the most of anything |
| terracotta | `#d97757` | the big circle | activity and heat |
| sky | `#6a9bcc` | the circle on the terracotta | sigma and uncertainty |
| rose | `#c46685` | the small circle | the provisional, and what needs attention |
| olive | `#788c5d` | the half circle at the bottom | complete and healthy |

Everything else is worked out from these six in OKLab. A measured color that fails a check below is not used
as is: the check stays and the color moves along OKLab lightness only, keeping its hue and chroma, by the
smallest step (0.0005 in L) that passes once written as hex.

| Token | Measured | Used | Why it moved |
|---|---|---|---|
| `--activity` | `#d97757` | `#d77656` | 2.96:1 on paper, under 3:1 (WCAG 1.4.11); L −0.004 reaches 3.01:1 |
| `--uncertainty` | `#6a9bcc` | `#6494c5` | 2.78:1 on paper; L −0.022 reaches 3.03:1 |
| `--attention` | `#c46685` | `#c46685` | 3.57:1, kept |
| `--healthy` | `#788c5d` | `#788c5d` | 3.49:1, kept |
| `--error` | `#c46685` | `#a74c6c` | text in rose: 3.15:1 on `--surface`, under 4.5:1 (WCAG 1.4.3); L −0.087 |
| `--added` | `#788c5d` | `#5d7043` | text in olive: 3.08:1 on `--surface`; L −0.094 |
| `--provisional-fill` | `#c46685` | `#e685a4` | the triangle's fill must be 7:1 against its ink edge and, on every cell, it or the edge 3:1; L +0.101 |

## The greys

Each grey lies on the straight line in OKLab from paper to ink, at a fixed fraction of the way: `--bg-deep`
0.035, `--surface` 0.055, `--raised` 0.09, `--line` 0.13, `--selection` 0.17, `--disabled` 0.42, `--text` 0.92,
`--text-strong` 1 (ink). `--text-secondary` is the lightest grey on the line that still reaches 4.5:1 on
`--surface`, the darkest ground text sits on; it measures 5.21:1 on paper. The page stays one sheet: grounds are
within 0.06 of paper, and the text is ink.

## The ramp

Seven steps for activity, plus `--ramp-0` for a cell with none.

- The ramp lies on the path from paper through terracotta to ink, in OKLab. Every step must stand 3:1 off the page
  (WCAG 1.4.11), which leaves the stretch from paper to terracotta to the empty cell: step 1 is `--activity`
  itself and step 7 is ink. On paper, more activity is darker.
- Each step is the same contrast ratio darker than the one before, 1.34, from 3.01:1 against the page to 17.5:1, so
  the order holds in grayscale and for every kind of color blindness. The hue stays terracotta's, fading into ink.
- `--ramp-0` is the grey at 0.07: a cell, but quiet.
- Checked by `ramp.test.mjs`: each step at least 1.2 times as far in contrast from the page as the one before;
  every step 3:1 against the page; neighbouring steps 0.04 apart in OKLab, twice the smallest noticeable difference,
  under protanopia, deuteranopia and tritanopia (Machado 2009, severity 1); text colors at 4.5:1; the four meaning
  colors at 3:1; step 1 is `--activity` and step 7 the ink of the text.
- A cell carries no text. The value appears on focus, hover and in the side panel.

## Marks that do not rely on color

- Unusual month: a diamond in the top right, ink with a paper edge (`--mark-fill`, `--mark-edge`): the strongest pair
  there is, so it shows on every step.
- Provisional month: a triangle in the bottom left corner, 10px along each side (`--provisional-size`), rose with an
  ink edge (`--provisional-fill`, `--provisional-edge`). The test proves it on every step and on an empty cell.
- Focus and selection: a `--focus-ring` outline in ink.

## The logo

An ink arch with a terracotta circle under its round top, on a 32 grid: the shapes of the reference, and the one
color that means fire. `logo.svg` takes its colors from the same generator, and the test checks they are tokens.

## Shape, type, size, motion

Flat and opaque: no gradient, blur, glow or translucency. One soft neutral shadow (`--shadow-float`, ink at 12%)
for what floats. Panels, fields and buttons are outlined in ink by hand (`web/app/frames.ts`, with the site's
`site/js/draw.js`); a rule between parts is one physical pixel: `--hairline` is `1 / scale` (1px, 0.5px at 2x,
0.34px at 3x). The app's pastels (land, sea, fire) are these tokens mixed with paper, as on the site.
Corners 10 on panels and fields, 6 on rows, buttons and thumbnails, 5 on tags; no pills, and a circle only where the
thing is a point. The geometry is the identity: arches, circles, half circles and rectangles.

SF Pro (the system face; Segoe UI on Windows) for the interface, Monaspace Neon for code and for numbers that must
line up. Sizes 17, 14, 12 and 11; hierarchy by size and by grey, never by color. Rows 38, fields 52, icons 24,
margins 8. Motion is a short spring (280 ms); drawings trace and pop in as `docs/style.md` says; all of it turns
off under `prefers-reduced-motion`.

Light only. A dark mode, if one is ever made, is derived from these same tokens, never picked by hand.

## Run the tests

```
node --test web/tokens
```

`web/package.json` runs them with its `test` script.
