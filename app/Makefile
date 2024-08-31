.PHONY: build

build:
	rm -rf ./dist
	mkdir -p ./dist/styles ./dist/scripts
	yarn run postcss --dir ./dist/styles/ ./styles/
	yarn run uglifyjs --compress --mangle -o ./dist/scripts/main.js ./scripts/main.js
	cargo run --manifest-path tools/cache_buster/Cargo.toml dist
