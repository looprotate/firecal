# References

Every external resource we use, cited when it is first used. The challenge rules require crediting all of them,
open source code included, and the submission form has a References field that is filled from this file.

Add a line when you use anything from outside the team: data, a library, a method, text, an image. Format:
`- Name: what we used it for. URL.`

## Data

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
- MapLibre GL JS: the map in the web app, once it is added. https://maplibre.org

## Method

- Quantile mapping: how VIIRS counts are mapped to MODIS-equivalent counts. Add the paper or text we follow
  when Cube chooses it.

## Sources checked for the FIRMS facts

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

- Ottosson, OKLab (the colour space the ramp is built in and measured in): https://bottosson.github.io/posts/oklab/
- Machado, Oliveira and Fernandes 2009, A physiologically-based model for simulation of color vision deficiency
  (the matrices in `web/tokens/ramp.test.mjs`): https://www.inf.ufrgs.br/~oliveira/pubs_files/CVD_Simulation/CVD_Simulation.html
- WCAG 2.2, contrast (1.4.3, 1.4.11) and the relative luminance formula: https://www.w3.org/TR/WCAG22/
