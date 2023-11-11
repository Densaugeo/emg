use clap::Parser;

#[derive(Clone, Debug)]
struct WebAssemblyArgError {}

impl std::fmt::Display for WebAssemblyArgError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Parameters passed to a WebAssembly module must be numbers. \
      Maximum of 64 bits (either signed integer or floating point)")
  }
}

impl std::error::Error for WebAssemblyArgError {}

#[derive(Clone, Debug)]
enum WebAssemblyArg {
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
}

impl core::str::FromStr for WebAssemblyArg {
  type Err = WebAssemblyArgError;
  
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match i32::from_str(value) {
      Ok(v) => return Ok(Self::I32(v)),
      _ => {},
    }
    
    match i64::from_str(value) {
      Ok(v) => return Ok(Self::I64(v)),
      _ => {},
    }
    
    match f32::from_str(value) {
      Ok(v) => return Ok(Self::F32(v)),
      _ => {},
    }
    
    match f64::from_str(value) {
      Ok(v) => return Ok(Self::F64(v)),
      _ => {},
    }
    
    Err(Self::Err {})
  }
}

#[derive(clap::Subcommand, Debug)]
enum Subcommands {
  /// Generate a GLTF model using a Paragen .wasm file
  Gen {
    /// Path to .wasm file
    wasm: String,
    
    /// Name of model within .wasm file to generate
    model: String,
    
    /// Parameters to pass to model
    parameters: Vec<WebAssemblyArg>,
    
    /// Not yet implemented - Will be used to specify format of result
    #[clap(long)]
    format: Option<u8>,
  },
  
  /// Not yet implemented - Will run a web server for viewing a Paragen .wasm
  /// file
  Serve {
    /// Path to .wasm file
    wasm: String,
  },
  
  /// Not yet implemented - Will report metadata from a Paragen .wasm file
  Inspect {
    /// Path to .wasm file
    wasm: String,
  }
}

/// Paragen by Den Antares
///
/// Various tools for working with Paragen .wasm files
#[derive(clap::Parser, Debug)]
#[clap(author, version)]
struct Cli {
  #[clap(subcommand)]
  subcommand: Subcommands,
}

fn main() {
  let args = Cli::parse();
  
  match args.subcommand {
    Subcommands::Gen {
      format,
      wasm,
      model,
      parameters,
    } => {
      println!("Gen: format={:?}, wasm={}, model={}", format, wasm, model);
      for i in parameters {
        println!("{:?}", i);
      }
    },
    Subcommands::Serve { .. } => println!("Not yet implemented"),
    Subcommands::Inspect { .. } => println!("Not yet implemented"),
  }
}
