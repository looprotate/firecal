# The site

The page that presents firecal: the logo draws itself and flies to the bar, and each section draws its figure as
the reader scrolls, in the hand-drawn style of the demo film. Static files, no build and no dependencies.

- `index.html` and `style.css`: the page. Each `.story` section stays pinned while its drawing runs; its words
  change at the steps `data-from` and `data-to` give, on a timeline from 0 to 100.
- `js/draw.js`: the hand-drawn line (resampling, the wobble and its boil eight times a second, smooth curves).
- `js/scene.js`: one drawing, its lines, fills and labels, each with when it draws and when it leaves.
- `js/scenes/`: what each section draws. `js/data.js`: every number in them, the app's answers on the published
  south-east Australia tiles.
- `tokens.css` and `logo.svg` are written by `cargo xtask tokens`; never edit them by hand.

Check: `node --test site/js/site.test.mjs`, also run by `cargo xtask check`.

## Publishing it

Serve this folder as the root of the host. Both "Open the calendar" links point at `app/`, but the app cannot live
under a path yet: it fetches `/firecal.wasm`, `/tiles/` and `/service-worker.js` from the root of its host. Until
that changes, serve the built app (`web/dist` after `npm --prefix web run build`) at the root of a host of its own
and point the two links there. The film plays from `firecal-demo.mp4` beside the page when the host has a copy, and
otherwise from the `space-apps-2026` release on GitHub.
