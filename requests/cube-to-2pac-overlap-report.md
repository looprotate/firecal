# Count overlap CSV for the before/after figure (#34 → #40)

The `firecal-harmonize report` command now exports raw MODIS, raw VIIRS and mapped
VIIRS monthly counts per tile. See `crates/firecal-harmonize/overlap-report.md`
for arguments, fields, fixtures and source-coverage restrictions.

The existing South America package yields 24 complete interior-tile rows for
July 2026: MODIS 7000, VIIRS 67025, mapped VIIRS 7000. All rows are training
observations; mark that explicitly in the figure. Do not label the result as
independent validation. August VIIRS is provisional and is not included.

In Cube's current workspace, the ignored CSV is:
`/workspace/firecal-cube-overlap-report/data/south-america-overlap-counts.csv`.
It has 1260 bytes and SHA-256
`6fc30e3dc8107af656507b671413620d5a5287bb79d54811da30c9f6c4617a57`.
The generated data file is not committed or uploaded to GitHub.

On another machine, recover Dre's package from the existing private release:
https://github.com/looprotate/firecal/releases/tag/data-south-america-2026-20261114

Run the report from an authorized checkout of this implementation using the
three standard FCA1 files and argument pattern documented above. The same
inputs and arguments reproduce the same CSV bytes. For plotting, group by tile,
use month_index as x, and plot the three count columns; keep split visible.
