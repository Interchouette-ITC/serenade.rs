.PHONY: build docs test preview clean

DOCS_DIR ?= ../serenade/docs/astro

build:
	cd apps/site && trunk build --release --locked --features hydrate
	$(MAKE) docs
	cargo run --locked --release --manifest-path apps/site/Cargo.toml --no-default-features --features ssr --bin prerender

docs:
	cd $(DOCS_DIR) && npm run build
	mkdir -p apps/site/dist/docs
	cp -a $(DOCS_DIR)/dist/. apps/site/dist/docs/

test:
	cargo test --locked --manifest-path apps/site/Cargo.toml --no-default-features --features ssr

preview:
	python3 -m http.server 4173 --bind 127.0.0.1 --directory apps/site/dist

clean:
	cargo clean --manifest-path Cargo.toml
	rm -rf apps/site/dist
