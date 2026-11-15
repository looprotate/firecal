# Judging criteria and the one-line pitch

Sources: the official [2026 Judging & Awards Guide](https://www.spaceappschallenge.org/resources/judging-awards-guide)
and [Project Submission Guide](https://www.spaceappschallenge.org/resources/project-submission-guide/), read on
2026-11-14. The challenge is "Harmonization of MODIS and VIIRS Hot Spots": a web app that turns the two
records into one burning-activity calendar over an area the user picks. The challenge page itself was not
readable by the writer; the repo's reading of it is `docs/idea.md`, and a person should compare the two.

What we claim is limited by `docs/idea.md` ("What we do not claim"). A tick means the claim is true **today in
`main`**; a box left open means it depends on work still in flight.

## How we are judged

Every level of judging scores five criteria from 1 to 5. Presentation counts the same as the other four, and the
guide says teams under-invest in it.

| Criterion | What the guide asks | Our evidence | Status |
|---|---|---|---|
| **Impact** | A real problem, a reach beyond the team | Emergency managers and land managers cannot compare burning across the MODIS and VIIRS eras, and cannot tell a real anomaly from a sensor change. We say how sure each number is. | argue it |
| **Creativity** | Original, or a meaningful improvement | Four layers that are one argument: uncertainty on every cell, energy and not only counts, analog years, an offline field tool. Most teams will stop at a heatmap. | argue it |
| **Validity** | Scientifically sound, feasible, usable | Calibration fitted per tile on the overlap years, its residuals shipped as sigma; native equals wasm byte for byte (8,968 calls); tests are the review (393 Rust, 102 web). Accessibility checked by hand (#41). Usability by a person on Windows is open (#6); held-out accuracy is unmeasured. | partly open |
| **Relevance** | Answers the challenge; NASA data is the core | NASA FIRMS MODIS and VIIRS hotspots are every number on screen. Nothing else is used for the science. | yes |
| **Presentation** | Clear story, engaging, well organized | A place everyone knows, a three-minute script (`script.md`), a 30-second demo, a backup video, and the airplane-mode moment. | in progress |

### Evidence checklist

Tick when it can be shown to a judge, not when it is planned. Checked against `main` on 2026-11-14.

- [ ] A calendar of one place and season (months across, years down) from real harmonized tiles. **Open:** real
  tiles exist only for July and August 2026 in South America, with no history to compare; see `story.md` for what
  each place needs. Today the calendar shows the hand-made sample, and says so.
- [x] Sigma visible on screen, and an anomaly flagged only when it clears the band: the estimated uncertainty in
  every cell's words, the band at two sigma under the calendar, a diamond on flagged months (#36). The core decides.
- [x] Count and energy (FRP) modes both working, from the core (#38). Energy has no sigma, and the page says so.
- [x] Analog years shown for the season in progress, and the chosen one outlined in the calendar (#39).
- [x] CSV of what the search covers: `series.csv` and `cells.csv` from the core (#37).
- [x] A new polygon answered with the network off: tested in Chrome on macOS, Windows and Linux on every pull
  request (#22), and by hand with the host stopped (#20).
- [x] Before and after of harmonization, measured, in `numbers.md` and in the app: raw VIIRS 67,025 against MODIS
  7,000, harmonized 7,000. **On the fit month only**, so it shows the method working, not its accuracy.
- [x] Every external resource in `docs/references.md`, software included. Still missing: the quantile mapping paper
  (Cube).

A layer that is half done on Sunday morning comes out of the demo and out of this table.

## Rules of the submission that shape the pitch

- **The demo is at most 30 seconds of video or at most seven slides.** Over the limit means not eligible for
  global judging. The three-minute script is for the live pitch; the submission needs the short cut.
- Everything on the project page is in English. A video may be in another language with English subtitles.
- At least one NASA data set must be named. Ours: FIRMS MODIS C6.1 and VIIRS 375 m active fire.
- The page and the demo may not show anyone under 18.
- Deadline: Sunday 2026-11-15, 23:59 local time of the team's event. Submit before it; saved after it does not count.

## The one-line pitch

Three versions. The team picks one and writes it at the end of this file.

1. **The literal one (the manifesto's line).**
   The same Rust function that harmonized the satellites runs in your browser, with no signal, on the area you
   just drew.
2. **The problem first.**
   Two generations of fire satellites, one burning calendar, and every number tells you how sure it is.
3. **The field first.**
   Pin a region, lose your signal, and still ask how this fire season compares with the ones before it.

### Which line for which moment

| Moment | Line | Why |
|---|---|---|
| Opening of the live pitch | 3 | A person in a place, not a pipeline. |
| The close, after airplane mode | 1 | It is literal at that point; the judges just watched it. |
| The first sentence of the form summary | 2 | Names the challenge in its own words. |

### Checks before a line is used

- No number in it that is not in `numbers.md`. The lines above have none on purpose.
- "Harmonized" means MODIS-equivalent counts, validated on the overlap years only.
- It does not say forecast, burned area or detected fires: a hotspot is a thermal anomaly seen from orbit.

## Decision

Chosen line: _not chosen yet._ Chosen by: _the team, in the call or in the thread of #43._
