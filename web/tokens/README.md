# Tokens

`tokens.css` is generated from the Carbón palette by `theme/palette.py` (`firecal()`), then committed. Colors
are not edited here: change the generator and write the file again. The neutrals and the red and green are
measured off an interface whose restraint we liked; there is no accent color, and color shows only when it
means something. The calendar ramp is the one place color carries a value, because heat is what it says.

## The ramp

Seven steps for activity, plus `--ramp-0` for a cell with none.

- Built in OKLab. Lightness rises evenly from 0.52 to 0.96, so the order survives grayscale and every kind of
  colour blindness; hue (ember at 30 to glow at 90) and chroma only add warmth.
- The first step is the darkest that still reaches 3:1 against the page (WCAG 1.4.11), so any activity stands
  out from an empty cell. It measures 3.16:1.
- Checked by `ramp.test.mjs`: each step is at least 1.2 times the luminance of the one before (the smallest is
  1.25); neighbouring steps stay 0.04 apart in OKLab, twice the smallest noticeable difference, under protanopia,
  deuteranopia and tritanopia (Machado 2009, severity 1); text colors meet 4.5:1.
- A cell carries no text. The value appears on focus, hover and in the side panel.

## Marks that do not rely on color

- Unusual month: a diamond, dark fill with a light edge (`--mark-fill`, `--mark-edge`), so it shows on every step.
- Provisional month: a triangle in the bottom left corner, 10px along each side (`--provisional-size`), the same
  dark fill and light edge as the diamond (`--provisional-fill`, `--provisional-edge`), so the shape tells it apart
  and the diamond keeps the top right. A dashed edge was tried first: inside the cell it was lost on both the dark and
  the light steps, and outside it was too thin to see at calendar size.
- Focus and selection: a `--focus-ring` outline.

## Type, size, motion

SF Pro (the system face; Segoe UI on Windows) for the interface, Monaspace Neon for code. Sizes 17, 14, 12 and 11.
Corners 10, 6 and 5; rows 38, fields 52, icons 24, margins 8. Motion is a short spring (280 ms) and turns off
under `prefers-reduced-motion`.

## Run the tests

```
node --test web/tokens
```

`web/package.json` (issue 17) runs them with its `test` script.
