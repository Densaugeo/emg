build:
	cargo build

build-example-blocks:
	cd examples/blocks && cargo build --target wasm32-unknown-unknown
