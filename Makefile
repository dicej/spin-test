.PHONY: build
build:
	/bin/bash -c "for x in \$$(seq 1 200); do head -c 1M /dev/urandom > static/asset-\$$(printf '%03d' \$$x).txt; done"
	cargo build --target wasm32-wasi --release
