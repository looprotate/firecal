# Submission text

> **DRAFT for the team to review before the deadline: Sunday 2026-11-15, 23:59 local time.** One section per field
> of the Space Apps project page, in the form's order. The page and every linked material must be in English.
> `[BRACKETS]` are placeholders for things that do not exist yet (a name, a link, a measured number, the chosen
> place). Nothing bracketed may be pasted into the form. Field rules are from the official
> [Project Submission Guide](https://www.spaceappschallenge.org/resources/project-submission-guide/), read on 2026-11-14.

## Project Name

`[NAME]` (the working name is `firecal`; the team has not chosen the final one).

## High-Level Project Summary

> Answers: what did you develop, how does it address the challenge, why is it important.

Two generations of NASA fire satellites, MODIS since 2000 and VIIRS since 2012, record burning in ways that
cannot be compared directly, so nobody has had one consistent picture of when and where burning happens.
`[NAME]` harmonizes both records from NASA FIRMS into a single burning-activity calendar for any area the user
draws: months across, years down. Every harmonized number carries an estimated uncertainty, so a month is
called unusual only when it clears that band and is not just a change of sensor. The calendar can show hotspot
counts or fire radiative power, finds the past seasons that most resemble the current one, exports a CSV, and
keeps working with no signal in a region pinned beforehand. It matters because the people who decide where to
look for burning need to know how much to trust a number, not only the number.

## Link to Project Demo

`[PUBLIC LINK]` to the 30-second video or the seven-slide deck. At most 30 seconds or seven slides, or the
project is not eligible for global judging. Public, no login; test it in a private window. Beats: `script.md`.
Nobody under 18 may appear. A video in another language needs English subtitles.

## Link to Final Project

https://github.com/looprotate/firecal (public since 2026-11-14, Apache-2.0) and, if hosted, `[PUBLIC LINK]` to the
live app. If a new website hosts the
app, the guide recommends requesting a FortiGuard web filter classification so judges can open it.

## Detailed Project Description

> Answers: what does it do, how does it work, what benefits, what do you hope to achieve, what tools.

**What it does.** The user picks an area on a map, by drawing or by typing latitude and longitude bounds, and a date
range. The app shows the burning calendar for it: one cell per month and year on an accessible sequential
color ramp, in two modes, hotspot count or total fire radiative power. It compares the current month with the
historical median, shows the past seasons that look most like the one in progress, and downloads a CSV of what the
search covers. After a region is pinned, all of it works with the network off.

**How it works.** NASA FIRMS active-fire hotspots from MODIS (Terra, Aqua) and VIIRS (Suomi NPP, NOAA-20,
NOAA-21) are aggregated per cell and month. VIIRS counts are mapped to MODIS-equivalent counts per tile by quantile
mapping, fitted on the years both sensors observed; the residuals over those years are the uncertainty shipped
with every cell. The result is a set of static tile files. A single Rust core computes the series, the anomaly
flag, the analog years and the CSV, and is compiled twice: native for the data pipeline and to WebAssembly for the
browser. A test requires both builds to give identical bytes. The TypeScript interface only draws; every number
on screen comes from the core. There is no backend, so the app is a static site and works offline.

**What sets it apart.**
1. Uncertainty on every number, and an anomaly flagged only outside the band and the range of the baseline years.
2. Energy as well as counts: many small agricultural burns look different from a few large fires.
3. Analog years: a picture of the seasons most like this one, compared by rank within the month, so quiet and
   busy regions are judged the same way.
4. An offline field tool: the function that harmonized the archive runs in the browser, with no signal.

**What we do not claim.** A hotspot is a thermal anomaly seen from orbit, not a measured fire or a burned area.
Harmonized counts are MODIS-equivalent and validated only on the overlap years. Analog years are history, not a
forecast. The newest months come from near-real-time data and are provisional. Sigma adds cell errors as if
they were independent, which is optimistic where neighbors share a bias, so the app says it is estimated.

**Results we can state.** All from `pitch/numbers.md`, where each has who measured it and what it does not show.
On the South America season of July and August 2026, the pipeline harmonized 24 complete 10° tiles in 0.06 seconds
on a laptop, into 346 kilobytes of static files. In the fitted month, raw VIIRS saw 67,025 hotspots where MODIS saw
7,000; harmonized, VIIRS gives 7,000 (agreement by construction on the month it was fitted on; no held-out month
yet). In the browser, the same Rust core answers an area query in about 13 microseconds over those tiles, and gives
byte-identical results to the native build on 8,968 test calls. With the network off, a pinned region still
answers, checked in Chrome on macOS, Windows and Linux on every change. Ingest read the whole public Uruguay
archive, 97,392 rows, in 79 milliseconds.

**Tools.** Rust (a Cargo workspace, no third-party dependencies in the core crates), WebAssembly, TypeScript, Vite,
MapLibre GL JS with OpenFreeMap's OpenStreetMap tiles, TerraDraw for drawing; Node for the tests and headless Chrome
for the offline test.

**What we hope to achieve.** A calendar that any land or emergency manager can read, with the honesty to say when it is not sure.

## Use of Super Intelligence (SI)

> The form says this answer is for data collection and judges do not see it, but they see SI watermarks,
> descriptive text and metadata. Whoever submits must confirm this is true and complete.

Draft, to be confirmed by the team: the team used AI coding agents (Claude Code and Codex, per `AGENTS.md`) to
write and test code and documentation in this repository, working from a design and rules written by the team.
The agents did not choose the idea. `[Confirm: whether any image, video or audio in the demo is generated; if so,
it needs a visible watermark or a note in its description and metadata.]` Generated content may not use NASA
branding.

## NASA Data

NASA FIRMS (Fire Information for Resource Management System) active-fire hotspots: MODIS Collection 6.1 (Terra,
Aqua) and VIIRS 375 m (Suomi NPP, NOAA-20, NOAA-21), standard and near-real-time products, downloaded from the
FIRMS archive and area API `[and the exact regions and dates of the chosen place, once chosen]`. At least one NASA
data set is required for a global award; this is the whole scientific input.

## Space Agency Partner Data and Other References

Fill this field from `docs/references.md` on the freeze, line by line: that file is the list of record and the
guide requires every code, text and image resource to appear, open source included. Today it lists FIRMS, the
MODIS and VIIRS products, Rust and Cargo, MapLibre GL JS, OpenFreeMap with OpenStreetMap and OpenMapTiles,
TerraDraw, TypeScript, Vite, Node and the Chrome DevTools Protocol, the quantile-mapping method, and the sources of
the identity (the reference image, OKLab, a color-vision-deficiency model, WCAG 2.2). Missing as of today: the
**method paper for the quantile mapping** (Cube). The app uses no icon set; its only drawing is the logo, generated
by the team. A partner agency's data is encouraged, not required; the Argentine and Brazilian agencies are among this
year's partners and `[the team has not looked at their resources]`.

## Review checklist before pressing "Submit for Judging"

- [ ] Every required field filled, all in English.
- [ ] Demo link opens in a private window; at most 30 seconds or seven slides.
- [x] Repository public and licensed (Apache-2.0); the README says so. Whether it reads well for a judge is the
      request to Biggie, still open.
- [ ] Every number in the text is in `pitch/numbers.md`; none is bracketed.
- [ ] The two confirmation boxes read and ticked by a person.
- [ ] Nobody under 18 in any image or video.
- [ ] A teammate, who did not write this, read the page start to end.
- [ ] Submitted before Sunday 2026-11-15, 23:59 local time. Saved after the deadline does not count.
