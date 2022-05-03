.PHONY: build
build:
	/bin/bash -c "for x in \$$(seq 1 200); do cp asset.txt static/asset-\$$(printf '%03d' \$$x).txt; done"
	cargo build --target wasm32-wasi --release
