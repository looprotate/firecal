# pitch

Eminem's directory: the story, the demo script, the numbers we measured, the backup video and the submission
text. Plain Markdown; no code.

Files to create as the weekend goes: `story.md` (the place and the season we tell), `criteria.md` (the
challenge's judging points, ticked), `script.md` (the three-minute demo), `numbers.md` (only measured numbers,
each with where it came from), `submission.md`.

## The look of slides, figures and the video

The pitch looks like the app, because they are one thing (`docs/style.md`, "The look"; the numbers are in
`web/tokens/README.md`).

- **Warm paper and ink.** Slides are paper with ink text; no photographs behind words, no gradients, no shadows.
- **Color means one thing.** Terracotta is activity and heat, sky is uncertainty, rose is the provisional and what
  needs attention, olive is complete and healthy. A color with no meaning on a slide is taken off.
- **Shapes, not icons.** Arches, circles, half circles and rectangles, as in the logo and the calendar's marks.
- **Type:** the system face; one size for the title and one for the rest, hierarchy by size and grey.

Images in `figures/` are generated, never drawn by hand:

| File | Made by | Again with |
|---|---|---|
| `title-card.svg` | `title_card()` in Axel's `theme/firecal.py`, the same generator as the tokens | `python3 theme/palette.py --write` in `~/dev/me` |
| `harmonization-before-after.svg` | the app's own figure (`web/calendar/figure.ts`) on the overlap report of south-east Australia, 2012 to 2019, fitted on 2012 to 2018 with 2019 held out (squares) | `firecal-harmonize report` on the Australia aggregates (see `numbers.md`), then the panel's "Save as SVG" |

The figure says in its own text which months were fitted (agreement by construction) and which were held out
(the test). Do not crop those lines off.
