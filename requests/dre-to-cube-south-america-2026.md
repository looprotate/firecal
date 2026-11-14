# South America API data handoff (#23)

The requested bounding box (-82,-56,-34,13) is downloaded for 2026-06-23 through
2026-11-14: 116 original CSVs, 3,006,747 rows, 251,108,012 bytes. All available
requested sensor-days are covered without SP/NRT overlap. S-NPP is unavailable
after November 2 and is not filled with zeros.

In the shared canonical checkout, use:

```text
data/raw/firms-south-america-20260623-20261114/
data/firms-2026/download-manifest.json
data/firms-2026/download-plan.json
data/firms-2026/availability.json
data/firms-2026/verified-summary.json
data/firms-2026/aggregate-manifest.json
data/aggregates-south-america-20260623-20261114/
```

The 30 FCA1 files contain 2,735,338 kept detections, match between one/four
workers, and have satellite/month count/day totals independently checked
against the CSVs. Keep this region/date set separate from the earlier Uruguay
handoff and never ingest all of `data/` recursively. Manifests preserve hashes,
source versions, request windows and retrieval timestamps without credentials.

Important interpretation requirements:

- MODIS is SP through August 31 and provisional from September 1. VIIRS
  NOAA-20/21 and S-NPP are SP through July 31 and provisional from August 1.
  A single global provisional cutoff, if required by the existing index, must
  be conservative; do not label August VIIRS as standard. No contract change is
  made by Dre; source-specific provenance remains in the download manifest.
- Provisional NOAA-20 includes some URT records from the combined NRT endpoint.
  SP includes `type`; provisional CSVs omit it. The vegetation/static-source
  filter therefore cannot be applied uniformly. Do not silently treat the
  provisional input as equivalent to standard processing during calibration.
- June 23–30 and November 1–14 are partial months; November 14 is also a live,
  incomplete day at the retrieval timestamp. S-NPP has November 1–2 only.
  Missing S-NPP coverage after that is not a measured zero series.
- The input region is a bounding box, not a political-country mask. These are
  raw detections aggregated per satellite, not calibrated counts or tiles.

Raw bytes are available in this execution workspace, not attached to git.
Another machine needs its own authorized API download, using the saved links
and source/window inventory in `crates/firecal-ingest/firms-api-access.md`,
`firms-2026-download.md` and `firms-2026-files.md`. Do not request or publish a
credential-bearing URL. The N21 satellite alias now has parser regression coverage.
