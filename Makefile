
.PHONY: build site prerender astro test preview clean

DOCS_DIR ?= ../serenade/docs/astro

# Main website: Leptos/WASM + SSR prerendering.
# Does NOT compile or copy Astro documentation.
build: site
	$(MAKE) prerender

# Build the Leptos frontend with Trunk.
site:
	cd apps/site && trunk build --release --locked --features hydrate

# Generate prerendered pages using the SSR binary.
prerender:
	cargo run --locked --release \
		--manifest-path apps/site/Cargo.toml \
		--no-default-features --features ssr --bin prerender

# Build Astro documentation and copy it into the website distribution.
astro:
	cd $(DOCS_DIR) && npm run build
	mkdir -p apps/site/dist/docs
	cp -a $(DOCS_DIR)/dist/. apps/site/dist/docs/

test:
	cargo test --locked --manifest-path apps/site/Cargo.toml \
		--no-default-features --features ssr

preview:
	python3 -m http.server 4173 --bind 127.0.0.1 --directory apps/site/dist

clean:
	cargo clean --manifest-path Cargo.toml
	rm -rf apps/site/dist
