# Easy-E to Biggie: browser tile catalog contract

Issues 19, 20 and 22 need the browser to find the tile files for a selected region and know the month range available in those files. `docs/data-format.md` names `tiles/index.json` and the tile filenames, but does not define the JSON fields. Please specify the index schema (tile row/column/path, month range, and how a missing tile is represented) or add a fixture index that the app can consume. The WASM series adapter is ready to accept tile bytes; until the catalog exists, I am testing it with the WASM fixture tile.
