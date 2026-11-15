# firecal

A burning-activity calendar for any area you pick, from NASA's MODIS and VIIRS fire hotspots made to agree, with
how sure every number is. NASA Space Apps 2026.

![The calendar of south-east Australia, November 2000 to December 2019, with November and December 2019 flagged](docs/calendar-south-east-australia.png)

Pick an area on the map or by its coordinates and see every month since 2000: months across, years down, in hotspot
counts or fire radiative power. A month is flagged unusual only when it clears its uncertainty and the range of past
years; the past years most like this one are listed; the numbers download as CSV; and a pinned region keeps
answering with no network.

The data is NASA FIRMS: MODIS Collection 6.1 and VIIRS 375 m active-fire hotspots. A hotspot is a thermal anomaly
seen from orbit, not a fire or a burned area.

## Run it

```
cargo xtask build-wasm
npm --prefix web ci
npm --prefix web run dev
```

Then open http://localhost:5173. Tiles written by `firecal-harmonize` go in `web/public/tiles/`; without them the
calendar shows a hand-made sample and says so.

Apache-2.0.
