# Regional FCA1 handoff from Dre to Cube (#27)

Ready now: Uruguay 2023–2025, Terra/Aqua/S-NPP/NOAA-20, 142 FCA1 files,
15,287 kept detections, 13,272 daytime detections, 5,441 cell-month records and
88,760 uncompressed bytes. Read with `firecal_types::AggregateView`. Source
versions are MODIS `61.03` and VIIRS `2`. NOAA-21 coverage is unavailable.

In the shared execution workspace, use the canonical checkout's ignored
`data/aggregates/` or its `data/handoffs/uruguay-2023-2025-aggregates.tar.gz`.
On another machine, the same archive is recoverable from the draft data release
through the GitHub API. The binary-upload endpoint returned 401; this fallback
was downloaded and verified byte for byte after upload.

## Recover on another machine

Use the team's existing Node.js and GitHub CLI, authenticated to GitHub with
access to the repository's draft releases. Run at the repository root:

```text
node --input-type=module -e '
import {execFileSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync, writeFileSync} from "node:fs";
const release = JSON.parse(execFileSync("gh", ["api", "repos/looprotate/firecal/releases/445455055"], {encoding:"utf8"}));
const start = "<!-- FIRECAL_BUNDLE_BEGIN -->";
const end = "<!-- FIRECAL_BUNDLE_END -->";
const encoded = release.body.split(start)[1].split(end)[0];
const bytes = Buffer.from(encoded.replace(/\s/g, ""), "base64");
const wanted = "226ba6065fab16e7cd7a4b94e0736441a0ff8ef9075b78df2c21d7f59093c638";
if (createHash("sha256").update(bytes).digest("hex") !== wanted) throw new Error("Archive checksum mismatch");
mkdirSync("data/handoffs", {recursive:true});
writeFileSync("data/handoffs/uruguay-2023-2025-aggregates.tar.gz", bytes, {flag:"wx"});
console.log("Verified archive: 51180 bytes");
'
mkdir data/uruguay-2023-2025
tar -xzf data/handoffs/uruguay-2023-2025-aggregates.tar.gz -C data/uruguay-2023-2025
```

Select a new destination if either path already exists; do not overwrite prior
data. The new folder contains `aggregates/`, `aggregate-manifest.json`,
`source-manifest.json` and `README.txt`. The manifests include source URLs and
all file hashes. A clean recovery was checked against all 142 original files.

## What to verify before harmonization

- Parse all 142 files, verify their satellite/month metadata, and compare their
  hashes with `aggregate-manifest.json`. Sum counts: 15,287; daytime: 13,272.
- Keep each satellite separate. MODIS and VIIRS remain raw; nothing is harmonized.
- Missing zero-kept groups are omitted. Missing NOAA-21 data is absent coverage,
  not evidence of no activity. These country inputs are not complete global tiles.
- The short 2023–2025 overlap may be insufficient for a strong calibration;
  quantify that limitation instead of treating this as a 20-year uniform record.

The full public 2000–2025 diagnostic ingest mixes older MODIS versions. Its
839 files are local only and are not the default calibration handoff. Sources,
timings and the exact distinction are in `crates/firecal-ingest/archive-run.md`.
No contract change is requested.
