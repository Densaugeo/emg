EXAMPLE=blocks

build-cli:
	cargo build --bin emg --features cli
	# Binary is at ./target/debug/emg

build-example:
	stat examples/$(EXAMPLE) > /dev/null
	cd examples/$(EXAMPLE) && cargo build --target wasm32-unknown-unknown
	ln -sf "$(EXAMPLE)/target/wasm32-unknown-unknown/debug/$(EXAMPLE).wasm" \
		"examples/$(EXAMPLE).wasm"

watch-example:
	stat examples/$(EXAMPLE) > /dev/null
	while true; \
		do inotifywait --event modify src/*.rs examples/$(EXAMPLE)/src/*.rs; \
		make build-example EXAMPLE=$(EXAMPLE); \
		curl -X POST http://localhost:8000/api-reloadserver/trigger-reload; \
	done

test:
	make build-cli
	make build-example EXAMPLE=blocks
	cargo test

clean:
	cargo clean
	cd examples/blocks && cargo clean
	rm examples/blocks.wasm
	rm -rf examples/output/*
