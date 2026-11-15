# The story: place and season

> **PROPOSAL, not a decision.** Written by Eminem's agent for Axel and Americo to choose from. Nothing in the
> demo, the script or the submission may treat a place as chosen until the **Decision** section at the bottom
> is filled in by a person. Issue #42 stays open until then.

## What the story has to do

The demo (`docs/idea.md`) needs one place and season that lets us show, in this order:

1. the calendar in one image, many years down and months across;
2. a **current** month flagged because it clears the uncertainty band;
3. the count to energy switch changing the story;
4. analog years: three past seasons that look like this one;
5. a new polygon answered offline.

So the place needs a **long history** (for the median, the band and the analogs), **both sensor families
overlapping** (for sigma), and a **current season** that is the one on screen. A place that is famous but
cannot give all three is the wrong place for this demo, however good the photographs.

## What data exists today

Read from `crates/firecal-ingest/archive-run.md`, `firms-2026-download.md` and Dre's numbers (now in `numbers.md`)
on 2026-11-14.

| Region | History | Current season | Status |
|---|---|---|---|
| **Uruguay** | Public FIRMS country archive, 2000 to 2025: MODIS, VIIRS S-NPP (from 2012), NOAA-20 (from 2018). Ingested; the full archive mixes MODIS product versions. | In the South America download below. | In hand |
| **South America box** (west -82, south -56, east -34, north 13) | None downloaded. | 2026-06-23 to 2026-11-14, all five satellites, standard where available and NRT after. S-NPP stops on 2026-11-02. | Current season only |
| Anywhere else | None. | None. | Needs a new download |

Two consequences. Only Uruguay has a full history today. And the South America box cannot show "now
against history" until someone downloads its past; it has the current season and nothing to compare it with.

### Checked by Biggie's agent on the real output, 2026-11-14

- **South America box:** Dre's release was downloaded and harmonized end to end: 24 complete tiles, July and August
  2026, query in the browser in 13 µs (`numbers.md`). The calendar cannot show it yet: the app needs at least one
  complete year before the season to compare with, and there is none. So B still needs a history download.
- **Uruguay:** the history exists only as Dre's per-satellite aggregates, and it cannot become tiles today.
  `firecal-harmonize build` accepts only tiles whose coverage is complete (`--complete-standard-tile`), and a country
  archive is cut at the border: every 10° tile that holds Uruguay also holds Argentina, Brazil or the sea, which the
  country files do not cover. Asserting the flag would turn the unobserved part into zeros, which the calibration
  spec forbids (`crates/firecal-harmonize/calibration.md`, "Calibration population"). A needs one of two things
  first: a regional (masked) mode in `firecal-harmonize` (Cube), or the full-tile archive of the Uruguay tiles from
  the FIRMS archive download (Dre). Either is work on Sunday, not a switch.
- **What runs today on real data:** the harmonization figure (July 2026, 24 tiles) and every number in
  `numbers.md`. The calendar on screen is the hand-made sample until one of the two above lands.

## Candidates

### A. Uruguay, a recent season (recommended)

- **For:** the only place with a history already ingested; every layer of the demo could run on it once it becomes
  tiles (see the check above: that is not today).
  Three sensors over a long history gives a real overlap for sigma and a real set of years for analogs. No
  new download, so no new risk on Saturday.
- **For:** a small place is a good place to show honesty. Its monthly counts are small, so the uncertainty band
  is wide, and the demo can say plainly that most months are not unusual. That is the pitch, not a flaw.
- **Against:** counts are small (the ingest report counts about 15 thousand kept detections in the 2023 to 2025
  window), so the calendar will look sparse next to a continental fire season. Energy mode matters more here.
- **Open:** which season. Do not pick it from memory. Pick the month that, in the real tiles, clears the band
  and has at least three analog years that differ in what happened next. Biggie checks that once tiles exist.
- **For:** the README names the event as "NASA Space Apps 2026 (Uruguay)", so the room and the judges are likely to
  know the place. A person should confirm that is how the team wants to be seen.
- **Needs from a person:** confirm the team can speak about Uruguay with authority on stage.

### B. A box in the South American spring burning season, 2026 (backup, only with a history download)

- **For:** the current season is already on disk and is large (about 3 million raw detections across the whole
  box), so the map and the calendar will look alive; and "this season, right now" is the strongest opening line.
- **Against:** no history. To flag anything or find analogs, someone must fetch the past years for a smaller
  box and ingest them; that is Dre's call and costs time on Saturday. Anything we say about it before then is
  a claim without data.
- **Switch rule:** move to B only if the Uruguay tiles show nothing that clears the band in any recent season, and
  only after Dre confirms in writing that a history for a named sub-box can be on disk by a stated hour.

### C. A globally famous season elsewhere (not recommended)

Examples from general knowledge, not checked against FIRMS: the Australian Black Summer of 2019 to 2020,
California in late summer 2020, the 2020 Pantanal season. Everyone recognizes them, which is the whole appeal.
**Against:** nothing is downloaded, the archive route takes hours to be prepared, and the whole region would have
to be ingested and harmonized from zero. A demo that never breaks beats a famous one; this one is the likeliest to break.
It is fine as a one-slide mention ("the same calendar works for ..."), only if the claim is true when said.

## Recommendation

1. **Primary:** A, Uruguay, with the season chosen from the data by the rule above.
2. **Backup:** B, with its switch rule. If neither works, show A honestly with a quiet season rather than
   changing the story on stage.
3. C only as a spoken aside.

## What we must not say about the place

- Not "fires" or "burned area": hotspots are thermal anomalies seen from orbit.
- Not a cause, not a forecast. Analog years are history.
- Nothing about a specific event's size, date or damage unless it is in `docs/references.md` with a source.
- The newest months come from near-real-time data and are labeled provisional.

## Questions for people

1. Axel and Americo: is Uruguay the place the team wants to stand behind on stage?
2. Cube or Dre: which of the two routes above gets Uruguay into tiles, and by what hour? Then Biggie: which Uruguay
   month clears the band, and which three years are its analogs?
3. Dre: could a history for a named sub-box of South America be on disk by an hour we can name? (Only if B is
   wanted; a request file goes in `requests/` when it is.)

## Decision

Chosen place and season: **south-east Australia, the 2019 season, shown as it stood on the first of January
2020** (the area west 147, south −38, east 151, north −34: East Gippsland and the New South Wales south coast). It is
option C, which this file advised against only because nothing was downloaded; the public country archive of
Australia turned out to cover its tiles completely, so it now runs end to end on real data: twenty years of history,
the core flags November and December 2019 (December: 7,112 hotspots against a median December of 7), the analog years
are 2009, 2001 and 2002, and 2019 was held out of the calibration, so the before and after figure tests the method on
a year it did not see. Everyone in the room knows the Black Summer, and the honest twist is on screen: January 2003,
the alpine fires, is still the largest January in this area, so the core does not call January 2020 unusual.

Backup: the same tiles and the same story with the hand-typed bounds instead of a drawn area; if the app fails, the
backup video (`video.md`), recorded on these tiles.

Chosen by and on: Decided on 2026-11-14 by Axel, the lead, who asked his agent to settle every open choice from the evidence; any teammate may reopen it before the freeze.

Uruguay stays the team's home and the opening line can say so; its own history needs a regional mode in
`firecal-harmonize` or the full-tile archive, which is after the hackathon.
