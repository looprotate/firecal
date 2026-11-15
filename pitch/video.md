# The demo videos

Three videos, all outside git:

- **The submitted demo: an animated film**, 29.9 seconds (the form's limit is 30; over it, no global judging). A
  hand-drawn animation of the story, not a screen recording; every number in it comes from the app on the real
  tiles or from `numbers.md`. Described in the next section.
- **The full run**, about three minutes, following `script.md` beat by beat. Played in the room if the app breaks.
- **The 26-second screen cut** of the full run: the backup demo if the film cannot be used.

## The submitted demo: the animated film

Made on 2026-11-15 by Biggie's agent, at Axel's request, in the hand-drawn style of `~/dev/me/claude/CLAUDE.md`
("Ilustración y animación"): paper and ink, pastel fills, lines that draw themselves on and off. Silent; everything
it says is written on screen in English. 1920 × 1080, 24 fps, H.264 MP4.

How it was made: one deterministic function draws each instant as SVG; headless Chrome captured the 718 frames and
Apple's AVFoundation encoded them (MediaRecorder dropped frames at 1080p). The source is in
`data/video/animated-source/`, ignored by git; the calendar data it embeds is the core's answer for the demo area
and for the offline area, read from the app on the pinned tiles.

| Seconds | Beat of the guide | On screen |
|---|---|---|
| 0 to 8 | Who, and the challenge | The team line and the challenge's name. Two satellites over the Earth, MODIS since 2000 and VIIRS since 2012, drop their 2019 hotspots: 50,497 against 631,751, "12.5× apart". The VIIRS dots thin to the harmonized 38,910: "From 12.5× to 0.77×, on 2019, a year the fit never saw." |
| 8 to 12 | The name and tagline | The logo draws itself; "firecal. One burning calendar from two generations of satellites. Every number says how sure it is." |
| 12 to 18 | How it works | The real calendar of the demo area, 2000 to 2019, cell by cell. December 2019 circled: 7,112 ± 88 hotspots, a median December of 7, "Unusual, and not a change of sensor." |
| 18 to 24 | How it works, offline | A map of the area; the network goes off; a new area is drawn; the calendar redraws with that area's real answer. "In under a millisecond, by the same Rust core." NASA FIRMS named as the data. |
| 24 to 30 | The future | "Any area you draw. Any season. In the field, with no signal." The logo, the name and the repository's address. |

On screen at all times: "Data: NASA FIRMS". The film carries no AI notice: which AI tools the team used, and what
for, is declared when the project is submitted (`submission.md`, "Use of Super Intelligence"), as Axel decided on
2026-11-15.

## The screen recordings

Status on 2026-11-14: **recorded**, silent with captions, on the real south-east Australia tiles, by Biggie's agent:
headless Chrome driven through the shots below, each frame captured and encoded to WebM in the same browser. They
are backups, not the pitch: a person may record the full run again with a voice, from this table.

## Set up once

1. Serve the app as in `script.md` ("Before anyone speaks"), with the demo's tiles and `overlap.csv` in
   `web/public/tiles/`, and pin the demo region.
2. Browser at 1920 × 1080, zoom 100%, no other tabs, no bookmarks bar, notifications off, the system clock hidden or
   at a true time. Light mode is the only mode.
3. Recorder: QuickTime (macOS, File, New Screen Recording) or the Xbox Game Bar (Windows, Win + Alt + R), 1080p,
   30 frames a second, the microphone on for the full run.
4. The first frame is `figures/title-card.svg` opened full screen for two seconds, then the app.

## The shots, with the state of the app in each

| # | Seconds (full run) | The app shows | Action | Done when |
|---|---|---|---|---|
| 1 | 0:00 to 0:03 | `title-card.svg` | Nothing | The card has been still for two seconds |
| 2 | 0:03 to 0:20 | The page, area panel on the left, calendar on the right, the sample or the region | Hold | The status under the calendar says "From the core, on N tiles (N pinned)" |
| 3 | 0:20 to 0:40 | Coordinates of the place in West, South, East, North | Type them, press **Use this area** | The map frames the ink outline |
| 4 | 0:40 to 1:05 | The calendar of the place | Hover an old cell, then a recent one | The line under the grid reads each cell |
| 5 | 1:05 to 1:30 | A month with a diamond; the strip under the calendar | Point at the diamond, then the bar and its blue band | The sentence under the strip ends in "Unusual." |
| 6 | 1:30 to 1:50 | **Energy** pressed | Press it | Cells redraw in megawatts |
| 7 | 1:50 to 2:15 | **Years most like 2019** | Press the first year | Its row is outlined in the calendar |
| 8 | 2:15 to 2:50 | The network switch visible on screen | Turn it off; polygon tool; draw a new area in the region; close it on its first corner | The calendar redraws and the status says "From the core" |
| 9 | 2:50 to 3:00 | The calendar, network still off | Hold; say the last line | Three seconds of stillness to cut on |

Alternative to shot 6 if the room wants proof of the harmonization: scroll to **How the sensors were made to agree**
and hold on the figure, as in `script.md`.

## The 26-second screen cut

The backup to the animated film. Cut from the full run, no new takes: shots 1 (2 s), 3 (3 s), 4 (5 s), 5 (6 s), 8 (10 s), 9 (4 s). Spoken lines from
the end of `script.md`, or English subtitles if the voice is in Spanish. Check the length in the player before
uploading: 30.0 seconds at most.

## Rehearse before recording

- [ ] The region answers with the network off (shot 8) twice in a row on the recording machine.
- [ ] Every number to be said is in `numbers.md` and was measured on the demo's tiles.
- [ ] Nobody under 18 on screen, no faces needed at all.
- [ ] Nothing on screen says "fires" or "burned area"; provisional months, if visible, are called provisional.
- [ ] One full dry run with a timer, by the person who records; note in `script.md` what did not match.

## After recording

- [x] Save every video **outside git** in two places, and write here where they are and their length. Today both
      places are on Axel's laptop; a copy on the team's drive or a USB stick is still to do.
- [ ] Upload the animated film where the form can link it, public, no login, and open the link in a private window.
- [ ] Declare the AI's part in the submission form (`submission.md`, "Use of Super Intelligence").

| File | Length | Saved in | Saved in | Link |
|---|---|---|---|---|
| `firecal-demo-animated.mp4` (1920 × 1080, H.264), **the submitted demo** | 29.9 s | `~/dev/firecal/data/video/` on Axel's laptop (ignored by git) | `~/Movies/firecal/` on the same laptop | not uploaded: uploading publishes it, so a person does |
| `firecal-demo-full.webm` (1600 × 900, VP9) | 51 s | `~/dev/firecal/data/video/` on Axel's laptop (ignored by git) | `~/Movies/firecal/` on the same laptop | not uploaded |
| `firecal-demo-30s.webm` | 26 s | the same | the same | not uploaded: uploading publishes it, so a person does |
