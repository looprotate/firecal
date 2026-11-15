# The site

The page that presents firecal: the logo draws itself and flies to the bar, and each section draws its figure as
the reader scrolls, in the hand-drawn style of the demo film. Static files, no build and no dependencies.

- `index.html` and `style.css`: the page. Each `.story` section stays pinned while its drawing runs; its words
  change at the steps `data-from` and `data-to` give, on a timeline from 0 to 100.
- `js/draw.js`: the hand-drawn line (resampling, the wobble and its boil eight times a second, smooth curves). The
  app draws with it too, so the site, the app and the film are one hand.
- `js/scene.js`: one drawing, its lines, fills and labels, each with when it draws and when it leaves.
- `js/scenes/`: what each section draws. `js/data.js`: every number in them, the app's answers on the published
  south-east Australia tiles.
- `tokens.css` and `logo.svg` are written by `cargo xtask tokens`; never edit them by hand.

Check: `node --test site/js/site.test.mjs`, also run by `cargo xtask check`.

## Publishing it

Serve this folder as the root of the host, as https://firecal.pages.dev/ does. Both "Open the calendar" links point
at `app/`. The app fetches `/firecal.wasm`, `/tiles/`, `/assets/` and `/service-worker.js` from the root, so the built
app (`web/dist` after `npm --prefix web run build`) goes in with its `index.html` at `app/` and the rest at the root,
beside this page; its service worker then needs `/app/` in its shell and as the offline answer for `/app/` pages.
The tiles are the files of the `tiles-australia-2019` release, in `tiles/`. The film plays from `firecal-demo.mp4`
beside the page when the host has a copy, and otherwise from the `space-apps-2026` release on GitHub.
