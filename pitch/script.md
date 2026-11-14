# The demo script

> **DRAFT.** Written from the product as `AGENTS.md` and `docs/idea.md` describe it, before it can be run end to
> end. It is finished only when someone who did not build the app has run the demo from this file (see the
> dry-run table at the bottom). Words in `[BRACKETS]` are placeholders: a place, a season or a **measured** number
> that does not exist yet. Never say a bracketed number aloud until it is in `pitch/numbers.md`.

Two cuts of the same story:

- **Live, three minutes.** Second by second below.
- **Submitted demo, 30 seconds.** The form allows a video of at most 30 seconds or at most seven slides; over the
  limit means no global judging. Last section.

## Before anyone speaks (pre-flight, two minutes)

1. Open the app from the **local copy of `tiles/`**, not from the network. Static site; nothing to fall over.
2. Pin the region for `[PLACE]` and confirm the app answers once with the network off, then turn it back on.
3. The browser is full screen, zoom 100%, the window large enough for the whole calendar. Notifications off.
4. Mode starts on **count**. The date range is set to the whole history. No area drawn yet.
5. The Wi-Fi switch is visible on screen (menu bar or system tray), because the judges must **see** it go off.
6. The backup video is open in another window, ready to play.

## The three minutes

Each row: when, what is on screen, what is said (in English, for the room), and what to do if it breaks.
Total spoken text is about 330 words, which is a calm pace for three minutes.

| Time | On screen | Say | If it breaks |
|---|---|---|---|
| 0:00 to 0:20 | The map, centered on `[PLACE]`, nothing drawn. | "Pin a region, lose your signal, and still ask how this fire season compares with the ones before it. Two families of satellites have been watching `[PLACE]` burn, one since 2000 and one since 2012, and nobody can put them on one calendar." | Skip the map; go straight to 0:20. |
| 0:20 to 0:40 | Draw a rectangle over `[PLACE]`. Pick the full date range. | "I draw the area. That is all the user does." | Type lat/lon bounds instead of drawing. |
| 0:40 to 1:05 | The **calendar** appears: months across, years down, cells on the sequential ramp. Hold on it. Point at one old cell, then at a recent one. | "This is the burning calendar. Months across, years down. Every cell is a count of hotspots, harmonized, so the older sensor and the newer one finally agree. And every cell carries a margin of error, because harmonizing is an estimate and we show how big." | If sigma marks are missing, do not mention them; say only the first two sentences. |
| 1:05 to 1:30 | The **current month** is marked. Click it: the comparison with the historical median and the band. | "Now the question an emergency manager asks: is this month unusual? We flag it only when it is outside the uncertainty band and outside what the baseline years have shown. So this is unusual, and it is not an artifact of switching sensors." | If nothing is flagged, say the honest version: "this month is inside the band, so we do not call it unusual." Do not force an alarm. |
| 1:30 to 1:50 | Switch the mode from **count** to **energy (FRP)**. The ramp redraws. | "Counts are not the whole picture. Switch to fire radiative energy and the story changes: many small burns look different from a few large ones." | Stay on count; cut this beat and add the time to the next. |
| 1:50 to 2:15 | The **analog years** panel: three past seasons that resemble this one, with what happened next in each. | "Which past years does this season look like? These three. We compare rank within the month, so a quiet region and a busy one are judged the same way. This is history, not a forecast." | If analogs are not ready, remove the beat and say nothing about analogs, anywhere. |
| 2:15 to 2:50 | **Turn the network off, on camera.** Then draw a **new** polygon somewhere else inside the pinned region. The calendar answers. | "Now the part I care about. I switch the network off. Airplane mode. I draw a new area I have never queried. It answers in `[MEASURED: query time]`. The same Rust function that harmonized the satellites is running here, in the browser, on this laptop." | If it does not answer: say "no signal and no answer, so this is the limit", play the backup video from 2:15, and tell the judges the offline layer is the part we cut. |
| 2:50 to 3:00 | The calendar with the network still off. Optionally the CSV download. | "The sensor stops mattering, and every number says how sure it is." | None. End on the sentence. |

## Things to never say

- "Fires" or "burned area". It is hotspots: thermal anomalies seen from orbit.
- "Forecast" or "prediction". Analog years are history.
- Any number that is not in `pitch/numbers.md`. Placeholders stay placeholders.
- "Accurate" without the validation: harmonized counts are MODIS-equivalent, validated only on the overlap years.
- If the newest months are on screen: say they are provisional, from near-real-time data.

## Questions to be ready for

| Question | Answer, short |
|---|---|
| How do you know the harmonization is right? | It is fitted on the years both sensors saw; the leftover error on those years is the sigma we show. It is validated only on that overlap. |
| Why no server? | A static site cannot fall over, and it works with no signal. |
| Is the anomaly a forecast? | No. It says this month is outside what the past shows, and not just a sensor change. |
| What does it not do? | No alerts, no accounts, no forecast, no raw per-hotspot history. |

## The 30-second submitted demo

The form's own beats, mapped to ours. Speak in English, or in another language with English subtitles. Nobody
under 18 may appear. The project name is `[NAME]` until the team chooses it (`firecal` is a working name).

| Time | Beat | On screen and what is said |
|---|---|---|
| 0:00 to 0:02 | Who | Title card: `[NAME]`, the team name, the challenge "Harmonization of MODIS and VIIRS Hot Spots". |
| 0:02 to 0:05 | The challenge | "Two satellite families, one fire record that cannot be compared." |
| 0:05 to 0:10 | The name and tagline | "`[NAME]`: every number says how sure it is, with or without signal." |
| 0:10 to 0:20 | How it works | The calendar, then the current month flagged, then the polygon drawn with the network off. "Built on NASA FIRMS MODIS and VIIRS hotspots." |
| 0:20 to 0:30 | The future | "A calendar for any area, in the truck, with no signal." |

If the team prefers slides: seven at most, including the title. Suggested order is the same five beats, one slide
each, plus the team members and the NASA data used.

## Dry run (what makes this file done)

Whoever runs this must not be the person who built the app or wrote the script.

| Run | Who | Date and time | Reached 3:00 | What did not match the script |
|---|---|---|---|---|
| 1 | _not run_ | | | |
