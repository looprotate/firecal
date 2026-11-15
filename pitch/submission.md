# Submission text

> **Ready to paste, before the deadline: Sunday 2026-11-15, 23:59 local time.** One section per field of the Space
> Apps project page, in the form's order; paste what is under each heading, not these notes. The page and every
> linked material must be in English. Field rules are from the official
> [Project Submission Guide](https://www.spaceappschallenge.org/resources/project-submission-guide/), read on
> 2026-11-14. The checklist at the end is for whoever presses "Submit for Judging".

## Project Name

firecal.

## High-Level Project Summary

> Answers: what did you develop, how does it address the challenge, why is it important.

Two generations of NASA fire satellites, MODIS since 2000 and VIIRS since 2012, record burning in ways that
cannot be compared directly, so nobody has had one consistent picture of when and where burning happens.
firecal harmonizes both records from NASA FIRMS into a single burning-activity calendar for any area the user
draws: months across, years down. Every harmonized number carries an estimated uncertainty, so a month is
called unusual only when it clears that band and is not just a change of sensor. The calendar can show hotspot
counts or fire radiative power, finds the past seasons that most resemble the current one, exports a CSV, and
keeps working with no signal in a region pinned beforehand. It runs in the browser at https://firecal.pages.dev/,
over twenty years of all of Australia. It matters because the people who decide where to look for burning need to
know how much to trust a number, not only the number.

## Link to Project Demo

https://github.com/looprotate/firecal/releases/download/space-apps-2026/firecal-demo.mp4

The animated film (29.9 s, 1920 × 1080, `video.md`), attached to the `space-apps-2026` release of the repository:
public, no login, checked by downloading it signed out. The backup is the 26-second screen cut,
`firecal-demo-30s.webm`, not uploaded. At most 30 seconds or seven slides, or the project is not eligible for global
judging. Nobody under 18 appears. The film is silent, with all its words on screen in English.

## Link to Final Project

https://firecal.pages.dev/

The project's site, with the film, and the app itself at https://firecal.pages.dev/app/: a static site on Cloudflare
Pages, no login, with harmonized tiles for all of Australia, November 2000 to December 2019. The code is at
https://github.com/looprotate/firecal (public since 2026-11-14, Apache-2.0; the README says how to run it). A
FortiGuard web filter classification for the site was requested on 2026-11-15, as the guide recommends, so judges
behind that filter can open it.

## Detailed Project Description

> Answers: what does it do, how does it work, what benefits, what do you hope to achieve, what tools.

**What it does.** The user picks an area on a map, by drawing a rectangle or a polygon or by typing latitude and
longitude bounds. The app shows the burning calendar for it, every month from November 2000 to December 2019: one cell per month and year on an accessible sequential
color ramp, in two modes, hotspot count or total fire radiative power. It compares the current month with the
historical median, shows the past seasons that look most like the one in progress, and downloads a CSV of what the
search covers. After a region is pinned, all of it works with the network off. The hosted app has tiles for all of
Australia; the map outlines that region, and an area outside it says it has no data yet and offers a way there,
rather than showing numbers it does not have.

**How it works.** NASA FIRMS active-fire hotspots from MODIS (Terra, Aqua) and VIIRS (Suomi NPP, NOAA-20,
NOAA-21) are aggregated per cell and month. VIIRS counts are mapped to MODIS-equivalent counts per tile by quantile
mapping, fitted on the years both sensors observed; the residuals over those years are the uncertainty shipped
with every cell. The result is a set of static tile files. A single Rust core computes the series, the anomaly
flag, the analog years and the CSV, and is compiled twice: native for the data pipeline and to WebAssembly for the
browser. A test requires both builds to give identical bytes. The TypeScript interface only draws; every number
on screen comes from the core. There is no backend, so the app is a static site and works offline. The interface
is drawn by hand in code, in the same style and with the same drawing code as the film and the project's site, so
the three read as one thing.

**What sets it apart.**
1. Uncertainty on every number, and a month flagged only when it rises past the band and above every baseline year.
2. Energy as well as counts: many small agricultural burns look different from a few large fires.
3. Analog years: a picture of the seasons most like this one, compared by rank within the month, so quiet and
   busy regions are judged the same way.
4. An offline field tool: the function that harmonized the archive runs in the browser, with no signal.

**What we do not claim.** A hotspot is a thermal anomaly seen from orbit, not a measured fire or a burned area.
Harmonized counts are MODIS-equivalent and validated only on the overlap years. Analog years are history, not a
forecast. The newest months come from near-real-time data and are provisional. Sigma adds cell errors as if
they were independent, which is optimistic where neighbors share a bias, so the app says it is estimated.

**Results we can state.** All from `pitch/numbers.md`, where each has who measured it and what it does not show.
Over south-east Australia, twenty years of public FIRMS records (16.6 million rows) were ingested in 7.3 seconds and
harmonized into 230 months of tiles in a quarter of a second, on a laptop. Holding 2019 out of the calibration,
raw VIIRS counted 12.5 times what MODIS counted, and harmonized VIIRS 0.77 times: close to MODIS from a gap of an
order of magnitude, and low because 2019 went past anything the calibration had seen. The core flags November and
December 2019 as unusual (December: 7,112 hotspots against a median December of 7) and finds 2009, 2001 and 2002 as
the most similar past years; in the browser it answers that in under a millisecond, with no network.
The same build over all of Australia writes 15 tiles of 230 months in under half a second; in the hosted app,
December 2019 is flagged unusual in south-west Western Australia (384 hotspots) and in the Top End (505) as well.
On the South America season of July and August 2026, the pipeline harmonized 24 complete 10° tiles in 0.06 seconds
on a laptop, into 346 kilobytes of static files. In the fitted month, raw VIIRS saw 67,025 hotspots where MODIS saw
7,000; harmonized, VIIRS gives 7,000 (agreement by construction on the month it was fitted on; no held-out month
yet). In the browser, the same Rust core answers an area query in about 13 microseconds over those tiles, and gives
byte-identical results to the native build on 8,968 test calls. With the network off, a pinned region still
answers, checked in Chrome on macOS, Windows and Linux on every change. Ingest read the whole public Uruguay
archive, 97,392 rows, in 79 milliseconds.

**Tools.** Rust (a Cargo workspace, no third-party dependencies in the core crates), WebAssembly, TypeScript, Vite,
MapLibre GL JS with OpenFreeMap's OpenStreetMap tiles, TerraDraw for drawing; Node for the tests and headless Chrome
for the offline test; Cloudflare Pages to host the static site. The demo film was drawn as SVG in headless Chrome and
encoded with Apple's AVFoundation.

**What we hope to achieve.** A calendar that any land or emergency manager can read, with the honesty to say when it is not sure.

## Use of Super Intelligence (SI)

> The form says this answer is for data collection and judges do not see it, but they see SI watermarks,
> descriptive text and metadata. Whoever submits must confirm this is true and complete.

The team used AI coding agents (Claude Code and Codex, per `AGENTS.md`) to
write and test code and documentation in this repository, working from a design and rules written by the team.
The agents did not choose the idea. The demo video is an animation drawn by code that an AI agent wrote, in a style
the team chose: every frame is an SVG drawing computed from the app's real numbers, with no image-generation model.
The interface of the app and the project's site were drawn the same way. The backup video was recorded by an agent
driving the real app. This answer is where the team declares the AI's
part; the videos carry no notice of their own. Generated content may not use NASA branding, and the film uses none.

## NASA Data

NASA FIRMS (Fire Information for Resource Management System) active-fire hotspots: MODIS Collection 6.1 (Terra,
Aqua) and VIIRS 375 m (Suomi NPP, NOAA-20, NOAA-21), standard and near-real-time products, downloaded from the
FIRMS public country archive (Australia: MODIS 2000 to 2020 and VIIRS S-NPP 2012 to 2020) and the FIRMS area API
(South America, June to November 2026). At least one NASA
data set is required for a global award; this is the whole scientific input.

## Space Agency Partner Data and Other References

Fill this field from `docs/references.md` on the freeze, line by line: that file is the list of record and the
guide requires every code, text and image resource to appear, open source included. Today it lists FIRMS, the
MODIS and VIIRS products, Rust and Cargo, MapLibre GL JS, OpenFreeMap with OpenStreetMap and OpenMapTiles,
TerraDraw, TypeScript, Vite, Node and the Chrome DevTools Protocol, the quantile-mapping method, and the sources of
the identity (the reference image, OKLab, a color-vision-deficiency model, WCAG 2.2), and the quantile-mapping method
(Gudmundsson et al. 2012, Hydrology and Earth System Sciences). The app uses no icon set; its only drawing is the logo, generated
by the team. A partner agency's data is encouraged, not required; the Argentine and Brazilian agencies are among this
year's partners and the team did not use their data.

## Review checklist before pressing "Submit for Judging"

- [ ] Every required field filled, all in English.
- [x] Demo link opens without signing in (downloaded signed out on 2026-11-15); 29.9 seconds.
- [x] Repository public and licensed (Apache-2.0); the README is a short description of the project, links the
      hosted app, and says how to run it.
- [x] https://firecal.pages.dev/ and /app/ open without signing in; the app answers on Australia (checked
      2026-11-15: December 2019 in south-east Australia reads 7,112, unusual).
- [x] Every number in the text is in `pitch/numbers.md`.
- [ ] The two confirmation boxes read and ticked by a person.
- [ ] Nobody under 18 in any image or video.
- [ ] A teammate, who did not write this, read the page start to end.
- [ ] Submitted before Sunday 2026-11-15, 23:59 local time. Saved after the deadline does not count.
