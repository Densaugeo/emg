use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn first_test() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("emg")?;
  
  cmd.arg("gen")
    .arg("examples/blocks/target/wasm32-unknown-unknown/debug/blocks.wasm")
    .arg("invalid_generator");
  cmd.assert().code(emg::ErrorCode::ModelGeneratorNotFound as i32);
  
  Ok(())
}

#[test]
fn sunny_day() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("emg")?;
  
  cmd.arg("gen")
    .arg("examples/blocks/target/wasm32-unknown-unknown/debug/blocks.wasm")
    .arg("build_the_model")
    .arg("1");
  cmd.assert().code(emg::ErrorCode::None as i32);
  
  Ok(())
}
