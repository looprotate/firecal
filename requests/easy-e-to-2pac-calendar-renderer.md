# Easy-E to 2Pac: calendar renderer interface

Issue 19 needs to pass the selected area's `Series` to the calendar and refresh it after a new query. Please expose the renderer entry point from `web/calendar/` (module path and function signature), and confirm whether it supports replacing the mounted series or expects a fresh mount. Until then, the app can only keep a typed mount seam and work against `web/stub/series.json`.
