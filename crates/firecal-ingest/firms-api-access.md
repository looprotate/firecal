# FIRMS API links for issue #23

Official area CSV API: <https://firms.modaps.eosdis.nasa.gov/api/area/>

Product/date availability: <https://firms.modaps.eosdis.nasa.gov/api/data_availability/>

Free MAP_KEY registration: <https://firms.modaps.eosdis.nasa.gov/api/map_key/>

The successful NOAA-20 query, with the credential deliberately omitted:

```text
https://firms.modaps.eosdis.nasa.gov/api/area/csv/[MAP_KEY]/VIIRS_NOAA20_NRT/-82,-56,-34,13/1/2026-11-13
```

Read the key from the `FIRMS_MAP_KEY` environment variable at the download edge.
Never commit the real key or a generated URL containing it. Requests may cover
one to five days. With a date, the API returns that date through date + days - 1.
Check availability before selecting SP or NRT, and do not overlap the products
for the same sensor/day.

The requested run covers bounds west -82, south -56, east -34, north 13 from
2026-06-23 through 2026-11-14. Download results and provenance are recorded
separately after validation; raw CSVs remain under ignored `data/`.
