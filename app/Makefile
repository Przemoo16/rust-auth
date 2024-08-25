.PHONY: build

build:
	mkdir -p ./dist/styles ./dist/scripts
	yarn run postcss --dir ./dist/styles/ ./styles/
	yarn run uglifyjs --compress --mangle -o ./dist/scripts/main.js ./scripts/main.js
