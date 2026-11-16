# Serenade.rs

The Serenade framework's product and documentation site. Its pages are built as
Leptos components, hydrated in the browser, and prerendered to HTML at build time.
The shared header, footer, and pages each use the rangular `.html` + `.scss` +
`.rs` component pattern. The build compiles their templates to Leptos views and
their nested SCSS to browser CSS.

The HTML document is owned by Serenade. `apps/site/src/document/document.html`
(document shell) and `apps/site/src/document/head.html` (page metadata and Open
Graph attributes) are Rangular templates compiled for SSR only. The prerender binary
composes them with each page through Serenade's `Document`, then adds the exact
CSS, preload, and WASM loader markup from Trunk's asset manifest. Prerendering fails
unless the manifest has exactly one main WASM bundle whose generated loader is present,
so no page is written that cannot hydrate. Trunk's `index.html` only declares
`data-trunk` assets. Browser hydration mounts the page body.

## Requirements

- Rust toolchain compatible with the Serenade workspace
- `wasm32-unknown-unknown` Rust target
- `serenade-ssg` (installed by `make tools`; it installs Serenade's pinned Trunk on first use)
- Node.js and npm for the Starlight documentation site

## Build and preview

```sh
rustup target add wasm32-unknown-unknown
make tools
make test
make build
make preview
```

`make build` runs `serenade-ssg --release --app-dir apps/site`. It builds the
hydration bundle with Trunk, then prerenders every route in `ROUTES` to
`apps/site/dist/<route>/index.html` and minifies the HTML. The preview is
available at `http://127.0.0.1:4173/`.

The shared header uses the round logo at widths up to 1240px and places navigation
on a second row at widths up to 900px. The GitHub button stays visible with the
same pill styling at every screen size.

To include the documentation, run
`cd ../serenade/docs/astro && npm ci && cd -` once, then `make astro` after
`make build`. Open `http://127.0.0.1:4173/docs/`.

## Product status

Serenade is a Symfony-oriented Rust framework in active development. Cargo is the
package manager; applications compose Serenade components and bundles rather than
adopting a mandatory ORM, HTTP server, or UI framework.

Project links:

- [Serenade on GitHub](https://github.com/Interchouette-ITC/Serenade)
- [rangular](https://github.com/Interchouette-ITC/rangular)
