# Parser fixture

`ten-hotspots.csv` is entirely synthetic, not downloaded FIRMS data. It mixes the
two sensor schemas intentionally: unrelated brightness fields must not affect
the parser. The input contract and source references are in
`docs/data-format.md` and `docs/references.md` at the repository root.

Hand-checkable expectations:

- Days 1–6 are vegetation (type 0), nominal or high confidence. Their FRP is
  1 + 2 + 4 + 8 + 16 + 8 = **39 MW**. Four are daytime; two are nighttime.
- Day 7 is MODIS confidence 29 and day 8 is VIIRS low (`l`): drop both.
- Days 9–10 are volcano (1) and static land source (2): drop both.
- Exactly six rows survive and four are filtered. No malformed rows are present.

The parser tests additionally exercise real sensor-specific header shapes,
reordered and quoted columns, optional `type`, duplicate and missing columns,
CRLF, BOM, invalid CSV/UTF-8, coordinate bounds, non-finite or negative FRP,
Gregorian leap dates, satellite/instrument mismatches, and confidence boundaries.
