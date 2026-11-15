# References

Every external resource we use, cited when it is first used. The challenge rules require crediting all of them,
open source code included, and the submission form has a References field that is filled from this file.

Add a line when you use anything from outside the team: data, a library, a method, text, an image. Format:
`- Name: what we used it for. URL.`

## Data

- NASA FIRMS public country archive, Australia: MODIS 2000 to 2020 and VIIRS S-NPP 2012 to 2020 (30 files, 1.3 GB),
  the history of the demo's calendar and of the held-out test of harmonization; 159 rows have a negative FRP, read as
  0 MW (`docs/data-format.md`). https://firms.modaps.eosdis.nasa.gov/country/

- NASA FIRMS public country archive: all 48 available Uruguay CSVs (MODIS
  2000–2025, VIIRS S-NPP 2012–2025, NOAA-20 2018–2025) used for issue #27
  ingestion and throughput; 2023–2025 selected separately for the regional handoff.
  https://firms.modaps.eosdis.nasa.gov/country/
  https://firms.modaps.eosdis.nasa.gov/data/country/yearly_summary_files.txt

- NASA FIRMS public yearly country hotspot CSVs: January 2024 detections from Uruguay
  (MODIS, VIIRS S-NPP and VIIRS NOAA-20) used for the issue #26 real-month validation.
  https://firms.modaps.eosdis.nasa.gov/country/
  File listing: https://firms.modaps.eosdis.nasa.gov/data/country/yearly_summary_files.txt
  Attribute/provenance notes: https://firms.modaps.eosdis.nasa.gov/data/country/AttributesReadme.txt

- NASA FIRMS (Fire Information for Resource Management System): the MODIS and VIIRS active-fire hotspots that
  every number comes from. https://firms.modaps.eosdis.nasa.gov
- MODIS (Terra and Aqua) active fire product, collection 6.1; VIIRS 375 m active fire product (Suomi NPP,
  NOAA-20, NOAA-21): the sensors we harmonize.

## Software

- Rust and Cargo: the language and build tool of the core. https://www.rust-lang.org
- MapLibre GL JS: the renderer for the map in the web app. https://maplibre.org
- OpenFreeMap: the OpenStreetMap vector tiles and the Noto Sans glyphs of the basemap, drawn with firecal's own
  style (`web/map/style.ts`); attribution "OpenFreeMap © OpenMapTiles, data © OpenStreetMap contributors" is on the
  map. https://openfreemap.org/quick_start/ https://openfreemap.org/tos/
- OpenStreetMap contributors and OpenMapTiles: the map data and its schema behind those tiles.
  https://www.openstreetmap.org/copyright https://openmaptiles.org
- TerraDraw and `@watergis/maplibre-gl-terradraw`: the rectangle and polygon drawing tools on the map.
  https://github.com/JamesLMilner/terra-draw https://github.com/watergis/maplibre-gl-terradraw
- TypeScript and Vite: the language and the build of the web app. https://www.typescriptlang.org https://vite.dev
- Node.js: runs the web tests, the parity test and the smoke test; the smoke test drives headless Google Chrome over
  the Chrome DevTools Protocol. https://nodejs.org https://chromedevtools.github.io/devtools-protocol/
- Swift and Apple AVFoundation (`AVAssetWriter`): encode the demo film's frames, captured in headless Chrome, into
  an H.264 MP4. https://www.swift.org https://developer.apple.com/documentation/avfoundation/avassetwriter

## Method

- Quantile mapping: how VIIRS counts are mapped to MODIS-equivalent counts. Add the paper or text we follow
  when Cube chooses it.

## Sources checked for the FIRMS facts

- NASA FIRMS area CSV API, data availability API and MAP_KEY documentation:
  direct SP/NRT downloads for issue #23, requested in non-overlapping blocks
  of at most five days with credentials supplied only at the download edge.
  https://firms.modaps.eosdis.nasa.gov/api/area/
  https://firms.modaps.eosdis.nasa.gov/api/data_availability/
  https://firms.modaps.eosdis.nasa.gov/api/map_key/

- NASA FIRMS FAQ: issue #28 research on nominal overpasses, latitude effects,
  hotspot-only versus pixel-quality information, and the limits of an
  observation-effort proxy; checked 2026-11-14.
  https://www.earthdata.nasa.gov/data/tools/firms/faq

- FIRMS Archive Download (coverage dates, request form, standard versus NRT):
  https://firms2.modaps.eosdis.nasa.gov/download/
- FIRMS active fire data attributes (MODIS and VIIRS columns, confidence, type, day or night):
  https://earthdata.nasa.gov/data/tools/firms/active-fire-data-attributes-modis-viirs
- MODIS C6 and C6.1 Active Fire User's Guide (MODIS confidence, fields):
  https://earthdata.nasa.gov/s3fs-public/2023-09/MODIS_C6_C6.1_Fire_User_Guide_1.0.pdf
- NASA LANCE, NRT versus standard products: https://lance.modaps.eosdis.nasa.gov/data/difference.php

## Sources for the design tokens

- The reference image for the identity: a flat composition of geometric shapes on warm paper, chosen by Axel. Six
  colors were measured off it as pixel samples (`web/tokens/README.md`); the image itself is not in the repo.

- Ottosson, OKLab (the colour space the ramp is built in and measured in): https://bottosson.github.io/posts/oklab/
- Machado, Oliveira and Fernandes 2009, A physiologically-based model for simulation of color vision deficiency
  (the matrices in `web/tokens/ramp.test.mjs`): https://www.inf.ufrgs.br/~oliveira/pubs_files/CVD_Simulation/CVD_Simulation.html
- WCAG 2.2, contrast (1.4.3, 1.4.11) and the relative luminance formula: https://www.w3.org/TR/WCAG22/

## Count-writer regional validation

- Dre's South America 2026 FCA1 release, checked 2026-11-14: archive checksum,
  per-file hashes, source-window/availability manifests and README establish the
  standard-month and complete-interior-tile subset used for #31. The original
  NASA FIRMS source requests and retrieval timestamps are retained in the release
  manifests; no credentials or raw records are copied into git.
  https://github.com/looprotate/firecal/releases/tag/data-south-america-2026-20261114

## Site photography

- Nathan Stephenson / USGS Western Ecological Research Center, “Fire at Night”: real photograph of a forest wildfire in the western United States, used without a watermark only behind the Site title. Public Domain. The photograph is illustrative; the calendar demo is south-east Australia. https://www.usgs.gov/media/images/fire-night

## Continent shortcuts

- United Nations Statistics Division M49: country grouping for the continent picker. Russia is assigned once to Europe; China is the largest country by area in the resulting Asia group. Canada, Brazil, Algeria and Australia are the largest countries by area in the other selected groups. Central America is excluded; Antarctica has no national capital. https://unstats.un.org/unsd/methodology/m49/overview/
- Capital-city shortcuts: Ottawa, Brasília, Beijing, Moscow, Algiers and Canberra; each opens a four-degree box around a rounded city-center coordinate. Location shortcuts do not imply data coverage.

## Donation destinations

Official pages checked on 2026-11-15, linked once each from the Site. These are external contribution pages; firecal does not collect donations or process payments.

- One Tree Planted Wildfire Fund: post-fire reforestation where natural regeneration is not possible. https://onetreeplanted.org/products/wildfire-fund
- Direct Relief wildfire response: emergency medical support in the United States. https://www.directrelief.org/emergency/wildfire/
- National Forest Foundation: broader forest conservation including wildfire recovery, reforestation and watershed restoration. https://www.nationalforests.org/donate-support/
- Fundación Bomberos de Argentina: general support for volunteer firefighter programs; not restricted to wildfires. https://fundacionbomberos.org.ar/dona

## Tree-and-flame brand mark

- AI-assisted original logo generated with OpenAI image generation in this conversation. The user selected concept 3 (pine and terracotta flame). An isolated transparent PNG is used as the header logo and favicon, and the same header image travels from the center of the welcome screen to its final slot. Source contact sheet and original isolated output are retained outside the source repository; the shipped asset is web/public/branding/firecal-tree.png. No NASA branding is used.

## Six capital regions: published historical tiles

- NASA FIRMS public country archive, retrieved 2026-11-15: https://firms.modaps.eosdis.nasa.gov/data/country/
- Added 208 MODIS and VIIRS Suomi NPP country/year CSVs for 2012–2019 (3,362,513,202 bytes), covering Canada, United States, Brazil, Russia, China, Mongolia, Belarus, Ukraine, Algeria, Tunisia, Libya, Spain and Italy. Neighboring-country inputs fill the same ten-degree tiles as the target capitals; they are not separate countrywide coverage promises.
- Six new tiles: (4,10), (10,13), (4,29), (5,29), (3,21), (5,18). Together with seven Australian tiles they support four-degree boxes around Ottawa, Brasília, Beijing, Moscow, Algiers and Canberra. The published coverage catalog identifies exact tile boundaries and shared month spans; unknown neighbors are rejected rather than treated as zero observations.
- Native firecal ingestion and harmonization use MODIS Terra and Suomi NPP; calibration fits through 2018-12 and excludes 2019. Added archives span 2012-01–2019-12; Australian data retains 2000-11–2019-12. These are historical records, not current wildfire tracking. Product versions vary across MODIS country archives; harmonized values and estimated uncertainty are exploratory. No observation-effort correction is applied.
- Deployment DATA-PROVENANCE.json records generated-file hashes and SOURCE-ARCHIVES.json records raw input checksums. Large raw files and runtime tile binaries are excluded from source Git.
- Period selection retains earlier records for historical context, truncates all analysis at the chosen end month, and exports only the displayed interval. Comparison uses a shared interval, reference years and color scale; absolute totals are not area-normalized. The first available year has no historical baseline and explicitly says so.

## Team visual reference

- User-supplied firecal.rar (received 2026-11-15), HTML/CSS visual mockup attributed by the user to a teammate's Figma. Used only for the forest-dark palette, larger editorial typography, card geometry and spacing. The original application, real data, scientific ramp, routes, translations and controls are retained; mockup statistics, stylized map, download handlers and page structure are not imported. Existing USGS public-domain photography and the approved transparent logo are retained.

## Screen structure and supplied photograph

- The user clarified that the supplied firecal.rar controls screen structure, layout and the single background photograph; existing Firecal information and working tools take priority. Six public screens follow index.html, explorer.html, approach.html, data.html, about.html and support.html. Screen navigation preserves the mounted scientific application and its query state. Direct static entry pages and offline navigation use the same built application.
- The supplied assets/forest-hero.png is used unchanged as web/public/photography/forest-hero.png, replacing the previous USGS photograph. Its source is the user-supplied archive, not the former USGS credit. The footer is plain as in the supplied design. The original file remains in the received archive; the previous image is retained outside the published build.
