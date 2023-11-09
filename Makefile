build:
	echo "You don't build the library, you build a project that uses it"

build-example-blocks:
	cd examples/blocks && cargo build --target wasm32-unknown-unknown
