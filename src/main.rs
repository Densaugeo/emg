use std::path::PathBuf;

use clap::Parser;

/// Paragen by Den Antares
///
/// Various tools for working with Paragen .wasm files
#[derive(clap::Parser, Debug)]
#[clap(author, version)]
struct Cli {
  #[clap(subcommand)]
  subcommand: Subcommands,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommands {
  /// Generate a GLTF model using a Paragen .wasm file
  Gen(ArgsForGen),
  
  /// Not yet implemented - Will run a web server for viewing a Paragen .wasm
  /// file
  Serve(ArgsForServe),
  
  /// Not yet implemented - Will report metadata from a Paragen .wasm file
  Inspect(ArgsForInspect),
}

#[derive(clap::Args, Debug)]
struct ArgsForGen {
  /// Path to .wasm file
  wasm: PathBuf,
  
  /// Name of model within .wasm file to generate
  model: String,
  
  /// Parameters to pass to model
  model_parameters: Vec<String>,
  
  /// Not yet implemented - Will be used to specify format of result
  #[clap(long)]
  format: Option<u8>,
}

#[derive(clap::Args, Debug)]
struct ArgsForServe {
  /// Path to .wasm file
  wasm: PathBuf,
}

#[derive(clap::Args, Debug)]
struct ArgsForInspect {
  /// Path to .wasm file
  wasm: PathBuf,
}

fn gen(args: ArgsForGen) {
  let engine = wasmtime::Engine::default();
  
  let module = match wasmtime::Module::from_file(&engine, args.wasm) {
    Ok(m) => m,
    Err(e) => {
      println!("Error: {:?}", e);
      std::process::exit(2);
    },
  };
  
  // The "store" seems to be wasmtime's container for instance data
  let mut store = wasmtime::Store::new(&engine, ());
  
  let instance = match wasmtime::Instance::new(&mut store, &module, &[]) {
    Ok(i) => i,
    Err(e) => {
      println!("Error: {:?}", e);
      std::process::exit(3);
    },
  };
  
  let generate_model = match instance.get_func(&mut store,
    (String::from("paragen_") + &args.model).as_str(),
  ) {
    Some(f) => f,
    None => {
      println!("Error: .wasm file does not contain the model `{}`", args.model);
      std::process::exit(4);
    }
  };
  
  let parameter_count = generate_model.ty(&store).params().len();
  if args.model_parameters.len() != parameter_count {
    println!("Error: model expects {} parameters, but {} were given",
      parameter_count, args.model_parameters.len());
    std::process::exit(5);
  }
  
  let mut model_args: Vec<wasmtime::Val> = Vec::new();
  let mut i = 0;
  for parameter in generate_model.ty(&store).params() {
    match parameter {
      wasmtime::ValType::I32 => {
        match args.model_parameters[i].parse::<i32>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            println!("Error: model parameter {} (`{}`) should be a 32-bit \
              integer", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::I64 => {
        match args.model_parameters[i].parse::<i64>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            println!("Error: model parameter {} (`{}`) should be a 64-bit \
              integer", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::F32 => {
        match args.model_parameters[i].parse::<f32>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            println!("Error: model parameter {} (`{}`) should be a 32-bit \
              floating-point value", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::F64 => {
        match args.model_parameters[i].parse::<f64>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            println!("Error: model parameter {} (`{}`) should be a 64-bit \
              floating-point value", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      _ => {
        println!("Error: paragen models only support parameters of type i32, \
          i64, f32, or f64");
        std::process::exit(6);
      }
    };
    
    i += 1;
  }
  
  println!("Extracted arguments for passing through to model: {:?}", model_args);
  
  let result_count = generate_model.ty(&store).results().len();
  if result_count != 1 {
    println!("Error: paragen models must return a 32-bit integer error code");
    std::process::exit(8);
  }
  
  // .unwrap() acceptable here because the length is asserted == 1
  match generate_model.ty(&store).results().next().unwrap() {
    wasmtime::ValType::I32 => {},
    _ => {
      println!("Error: paragen models must return a 32-bit integer error code");
      std::process::exit(8);
    }
  }
  
  println!("Trying to call some paragen functions...");
  let mut result = [wasmtime::Val::from(0)];
  match generate_model.call(&mut store, &model_args, &mut result) {
    Ok(_) => {},
    Err(e) => {
      println!("Error: {:?}", e);
      std::process::exit(9);
    },
  }
  // .unwrap() acceptable here because the type was previously asserted
  match result[0].i32().unwrap() {
    0 => {},
    e => {
      println!("Error: model generation returned error code: {}", e);
      std::process::exit(10);
    },
  }
  
  let mut pointer = [wasmtime::Val::from(0)];
  instance.get_func(&mut store, "pointer").unwrap().call(&mut store, &[], &mut pointer).unwrap();
  println!("Got pointer: {:?}", pointer);
  
  let mut size = [wasmtime::Val::from(0)];
  instance.get_func(&mut store, "size").unwrap().call(&mut store, &[], &mut size).unwrap();
  println!("Got size: {:?}", size);
  
}

fn serve(_args: ArgsForServe) {
  println!("Not implemented yet");
  std::process::exit(1);
}

fn inspect(_args: ArgsForInspect) {
  println!("Not implemented yet");
  std::process::exit(1);
}

fn main() {
  let args = Cli::parse();
  
  match args.subcommand {
    Subcommands::Gen(args) => gen(args),
    Subcommands::Serve(args) => serve(args),
    Subcommands::Inspect(args) => inspect(args),
  }
}
