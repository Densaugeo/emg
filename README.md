# emg

Ethereal Model Generator - Procedurally generate .gltf using WebAssembly modules

## TODO

- Tests for the macro
- Tests for the actual module
- Browser runtime
- CLI serve command
- Tests for CLI
  * First test case added!
  * Need set of test model gens
  * Need memory use info from emg-cli
  * Numerous .wasm files for testing emg::ErrorCode::ModuleNotEMG/OutputNotGLB cases
- Simple example project
- .glb generation should be default, other formats could be produced by CLI tool
  * Maybe abbreviate buffers out of them? Optionally?
- Some kind of starter project generator like cargo init
- Is there a good way to warn users if Cargo.toml optimizations aren't set up?
- CI setup
- Pick a license
- Find out if docs are autogenerated by crates.io
- Upload to crates.io
- And of course all the actual geometry work
