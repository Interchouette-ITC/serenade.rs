# Serenade.rs

The Serenade framework's product and documentation site. Its pages are built as
Leptos components, hydrated in the browser, and prerendered to HTML at build time.
The shared header, footer, and pages each use the rangular `.html` + `.scss` +
`.rs` component pattern. The build compiles their templates to Leptos views and
their nested SCSS to browser CSS.

## Requirements

- Rust toolchain compatible with the Serenade workspace
- `wasm32-unknown-unknown` Rust target
- Trunk
- Node.js and npm for the Starlight documentation site

## Build and preview

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
cd ../serenade/docs/astro && npm ci && cd -
make test
make build
make preview
```

`make build` generates the Leptos/Rangular product pages and builds the Astro/Starlight docs into
`apps/site/dist/docs/`. The preview is available at `http://127.0.0.1:4173/`; open
`http://127.0.0.1:4173/docs/` for the documentation.

## Product status

Serenade is a Symfony-oriented Rust framework in active development. Cargo is the
package manager; applications compose Serenade components and bundles rather than
adopting a mandatory ORM, HTTP server, or UI framework.

Project links:

- [Serenade on GitHub](https://github.com/Interchouette-ITC/Serenade)
- [rangular](https://github.com/Interchouette-ITC/rangular)
