.PHONY: build test

build:
	rm -rf ./dist
	mkdir -p ./dist/styles ./dist/scripts
	yarn run postcss --dir ./dist/styles/ ./styles/
	yarn run uglifyjs --compress --mangle -o ./dist/scripts/htmx.js ./scripts/htmx.js ./scripts/htmx-ext-loading-states.js
	yarn run uglifyjs --compress --mangle -o ./dist/scripts/main.js ./scripts/main.js
	cargo run --manifest-path tools/cache_buster/Cargo.toml dist

test: build
	cargo test
