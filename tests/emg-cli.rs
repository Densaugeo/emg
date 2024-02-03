use rstest::{rstest, fixture};
use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;
use emg::ErrorCode;

#[fixture]
fn cmd() -> std::process::Command {
  std::process::Command::cargo_bin("emg").unwrap()
}

///////////////////
// Tests for gen //
///////////////////

#[rstest]
fn gen_sunny_day_pretty(mut cmd: Command) {
  let expected = std::fs::read("tests/build_the_model-pretty.gltf").unwrap();
  
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("build_the_model").arg("1")
     .assert().code(ErrorCode::None as i32).stdout(expected);
}

#[rstest]
fn gen_sunny_day_glb(mut cmd: Command) {
  let expected = std::fs::read("tests/build_the_model.glb").unwrap();
  
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("build_the_model").arg("1").arg("--format").arg("glb")
     .assert().code(ErrorCode::None as i32).stdout(expected);
}

#[rstest]
fn gen_invalid_wasm(mut cmd: Command) {
  cmd.arg("inspect").arg("tests/invalid.wasm")
     .assert().code(ErrorCode::WebAssemblyCompile as i32).stdout("");
}

// emg::ErrorCode::WebAssemblyCompile not tested - I don't know of a way to
// make Web Assmbly instantiation fail

// emg::ErrorCode::ModuleNotEMG not tested - I don't want to make a .wasm for
// each possible failure mode right now

#[rstest]
fn gen_missing_model_generator(mut cmd: Command) {
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("invalid_generator")
     .assert().code(ErrorCode::ModelGeneratorNotFound as i32).stdout("");
}

#[rstest]
fn gen_parameter_count_low(mut cmd: Command) {
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("build_the_model")
     .assert().code(ErrorCode::ParameterCount as i32).stdout("");
}

#[rstest]
fn gen_parameter_count_high(mut cmd: Command) {
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("build_the_model").arg("1").arg("2")
     .assert().code(ErrorCode::ParameterCount as i32).stdout("");
}

#[rstest]
#[case("a")]
#[case("1.2")]
#[case("1e+6")]
#[case("5123456789")]
fn gen_parameter_bad_type(mut cmd: Command, #[case] bad_paramter: String) {
  cmd.arg("gen").arg("examples/blocks.wasm")
     .arg("build_the_model").arg(bad_paramter)
     .assert().code(ErrorCode::ParameterType as i32).stdout("");
}

// emg::ErrorCode::WebAssemblyExection not tested - I don't know of a way to
// make Web Assmbly execution fail

// emg::ErrorCode::OutputNotGLB not tested - I don't want to make a .wasm for
// it right now

///////////////////////
// Tests for inspect //
///////////////////////

#[rstest]
fn inspect_sunny_day(mut cmd: Command) {
  cmd.arg("inspect").arg("examples/blocks.wasm")
     .assert().code(ErrorCode::None as i32).stdout("")
     .stderr("Model generators found:\n\tbuild_the_model ( i32 )\n");
}

/////////////////////
// Tests for serve //
/////////////////////

#[rstest]
fn serve_not_implemented(mut cmd: Command) {
  cmd.arg("serve").arg("examples/blocks.wasm")
     .assert().code(ErrorCode::NotImplemented as i32).stdout("");
}
