# Wire the calendar package into the web build

`web/calendar/` is a package of plain TypeScript modules with tests, written against `web/stub/series.json`
because the scaffold (#74) is not on `main` yet. Three one-line changes in files you own put it in the build:

- `web/tsconfig.json`: add `"calendar/**/*.ts"` to `include`, so `npm run typecheck` checks it.
- `web/package.json`: add `calendar/*.test.mjs` to the `test` script.
- Where the app draws the calendar: `import { mountCalendar } from '../calendar/index.ts'` and call
  `mountCalendar(element, series)`. `series` is any object with `plane`, `first_month_index`, `value` and
  `sigma` (the app's `Series` fits). `update(series)` draws another one. `index.ts` imports `calendar.css`;
  the page must load `web/tokens/tokens.css` first, because the cells take their colors from its variables.

The calendar draws and computes nothing about fire. The test that proves it is `web/calendar/calendar.test.mjs`.
