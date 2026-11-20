
.PHONY: build ssg tools astro test preview clean

DOCS_DIR ?= ../serenade/docs/astro
SERENADE_SSG ?= serenade-ssg

# Main website: Leptos/WASM + SSR prerendering.
# Does NOT compile or copy Astro documentation.
build: ssg

# Build the hydration bundle with Serenade's managed Trunk, then prerender every
# route into complete documents carrying Trunk's generated assets.
ssg:
	$(SERENADE_SSG) --release --app-dir apps/site

# Install the Serenade SSG command from crates.io.
tools:
	cargo install --locked serenade-cli --bin serenade-ssg

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
