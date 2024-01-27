EXAMPLE=blocks

build:
	cargo build --bin emg --features cli

build-example:
	stat examples/$(EXAMPLE) > /dev/null
	cd examples/$(EXAMPLE) && cargo build --target wasm32-unknown-unknown

watch-example:
	stat examples/$(EXAMPLE) > /dev/null
	while true; \
		do inotifywait --event modify src/*.rs examples/$(EXAMPLE)/src/*.rs; \
		make build-example EXAMPLE=$(EXAMPLE); \
		curl -X POST http://localhost:8000/api-reloadserver/trigger-reload; \
	done
