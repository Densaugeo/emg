use std::path::PathBuf;

use clap::Parser;

/// emg by Den Antares
///
/// Various tools for working with emg .wasm files
#[derive(clap::Parser, Debug)]
#[clap(author, version)]
struct Cli {
  #[clap(subcommand)]
  subcommand: Subcommands,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommands {
  /// Generate a GLTF model using an emg .wasm file
  Gen(ArgsForGen),
  
  /// Not yet implemented - Will run a web server for viewing an emg .wasm
  /// file
  Serve(ArgsForServe),
  
  /// Not yet implemented - Will report metadata from an emg .wasm file
  Inspect(ArgsForInspect),
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
enum Format {
    /// Pretty-printed GLTF text format (.gltf)
    #[default]
    Pretty,
    
    /// GLTF text format (.gltf)
    GLTF,
    
    /// GLTF binary format (.glb)
    GLB,
}

#[derive(clap::Args, Debug)]
struct ArgsForGen {
  /// Path to .wasm file
  wasm: PathBuf,
  
  /// Name of model within .wasm file to generate
  model: String,
  
  /// Parameters to pass to model
  model_parameters: Vec<String>,
  
  /// Output format
  #[clap(short, long, default_value_t, value_enum)]
  format: Format,
  
  /// Print additional debug info to stderr (stdout is reserved for GLTF output)
  #[clap(short, long, default_value_t = false)]
  verbose: bool,
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
      eprintln!("Error: {:?}", e);
      std::process::exit(2);
    },
  };
  
  // The "store" seems to be wasmtime's container for instance data
  let mut store = wasmtime::Store::new(&engine, ());
  
  let instance = match wasmtime::Instance::new(&mut store, &module, &[]) {
    Ok(i) => i,
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(3);
    },
  };
  
  let generate_model = match instance.get_func(&mut store,
    (String::from("emg_") + &args.model).as_str(),
  ) {
    Some(f) => f,
    None => {
      eprintln!("Error: .wasm file does not contain the model `{}`", args.model);
      std::process::exit(4);
    }
  };
  
  let parameter_count = generate_model.ty(&store).params().len();
  if args.model_parameters.len() != parameter_count {
    eprintln!("Error: model expects {} parameters, but {} were given",
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
            eprintln!("Error: model parameter {} (`{}`) should be a 32-bit \
              integer", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::I64 => {
        match args.model_parameters[i].parse::<i64>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            eprintln!("Error: model parameter {} (`{}`) should be a 64-bit \
              integer", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::F32 => {
        match args.model_parameters[i].parse::<f32>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            eprintln!("Error: model parameter {} (`{}`) should be a 32-bit \
              floating-point value", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      wasmtime::ValType::F64 => {
        match args.model_parameters[i].parse::<f64>() {
          Ok(v) => model_args.push(wasmtime::Val::from(v)),
          Err(_) => {
            eprintln!("Error: model parameter {} (`{}`) should be a 64-bit \
              floating-point value", i + 1, args.model_parameters[i]);
            std::process::exit(7);
          },
        }
      },
      
      _ => {
        eprintln!("Error: emg models only support parameters of type i32, \
          i64, f32, or f64");
        std::process::exit(6);
      }
    };
    
    i += 1;
  }
  
  if args.verbose {
    eprintln!("Extracted arguments for passing through to model: {:?}", 
        model_args);
  }
  
  let result_count = generate_model.ty(&store).results().len();
  if result_count != 1 {
    eprintln!("Error: emg models must return a 32-bit integer error code");
    std::process::exit(8);
  }
  
  // .unwrap() acceptable here because the length is asserted == 1
  match generate_model.ty(&store).results().next().unwrap() {
    wasmtime::ValType::I32 => {},
    _ => {
      eprintln!("Error: emg models must return a 32-bit integer error code");
      std::process::exit(8);
    }
  }
  
  let mut result = [wasmtime::Val::from(0)];
  match generate_model.call(&mut store, &model_args, &mut result) {
    Ok(_) => {},
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(9);
    },
  }
  // .unwrap() acceptable here because the type was previously asserted
  match result[0].i32().unwrap() {
    0 => {},
    e => {
      eprintln!("Error: model generation returned error code: {}", e);
      std::process::exit(10);
    },
  }
  
  let get_pointer = match instance.get_func(&mut store, "pointer") {
    Some(function) => function,
    None => {
      eprintln!("Error: .wasm file is not a valid emg module: missing \
        required function `pointer()`");
      std::process::exit(11);
    },
  };
  if get_pointer.ty(&store).params().len() != 0 {
    eprintln!("Error: .wasm file is not a valid emg module: function \
      `pointer()` must not accept any arguments");
    std::process::exit(11);
  }
  if get_pointer.ty(&store).results().len() != 1 {
    eprintln!("Error: .wasm file is not a valid emg module: function \
      `pointer()` must return one result");
    std::process::exit(11);
  }
  // .unwrap() acceptable here because the length is asserted == 1
  match get_pointer.ty(&store).results().next().unwrap() {
    wasmtime::ValType::I32 => {},
    _ => {
      eprintln!("Error: .wasm file is not a valid emg module: function \
        `pointer()` must return a 32-bit integer");
      std::process::exit(11);
    }
  }
  let mut pointer = [wasmtime::Val::from(0)];
  match get_pointer.call(&mut store, &[], &mut pointer) {
    Ok(_) => {},
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(9);
    },
  }
  if args.verbose { eprintln!("Got pointer: {:?}", pointer) }
  
  let get_size = match instance.get_func(&mut store, "size") {
    Some(function) => function,
    None => {
      eprintln!("Error: .wasm file is not a valid emg modules: missing \
        required function `size()`");
      std::process::exit(11);
    },
  };
  if get_size.ty(&store).params().len() != 0 {
    eprintln!("Error: .wasm file is not a valid emg module: function \
      `size()` must not accept any arguments");
    std::process::exit(11);
  }
  if get_size.ty(&store).results().len() != 1 {
    eprintln!("Error: .wasm file is not a valid emg module: function \
      `size()` must return one result");
    std::process::exit(11);
  }
  // .unwrap() acceptable here because the length is asserted == 1
  match get_size.ty(&store).results().next().unwrap() {
    wasmtime::ValType::I32 => {},
    _ => {
      eprintln!("Error: .wasm file is not a valid emg module: function \
        `size()` must return a 32-bit integer");
      std::process::exit(11);
    }
  }
  let mut size = [wasmtime::Val::from(0)];
  match get_size.call(&mut store, &[], &mut size) {
    Ok(_) => {},
    Err(e) => {
      eprintln!("Error: {:?}", e);
      std::process::exit(9);
    },
  }
  if args.verbose { eprintln!("Got size: {:?}", size) }
  
  // Can .unwrap() because WebAssembly modules always have a "memory" export
  let memory = instance.get_memory(&mut store, "memory").unwrap();
  // Can .unwrap() because the types of pointer and size were asserted earlier
  let pointer_plain_int = pointer[0].i32().unwrap() as usize;
  let size_plain_int = size[0].i32().unwrap() as usize;
  let memory_of_interest = &(memory.data(&store))[pointer_plain_int..pointer_plain_int + size_plain_int];
  
  let parsed: serde_json::Value = match  serde_json::de::from_slice(memory_of_interest) {
    Ok(json) => json,
    Err(e) => {
      eprintln!("Error: model produced invalid JSON: {:?}", e);
      std::process::exit(12);
    },
  };
  
  match args.format {
    // Can .unwrap() because this JSON was just parsed, so it must be valid
    Format::Pretty => {
      println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    },
    Format::GLTF => println!("{}", serde_json::to_string(&parsed).unwrap()),
    Format::GLB => {
      eprintln!("Error: .glb generation is not supported yet");
      std::process::exit(13);
    },
  }
}

fn serve(_args: ArgsForServe) {
  eprintln!("Not implemented yet");
  std::process::exit(1);
}

fn inspect(_args: ArgsForInspect) {
  eprintln!("Not implemented yet");
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
