use std::path::PathBuf;
use std::fmt::Write;

use clap::Parser;

fn fail<S: AsRef<str>>(code: emg::ErrorCode, message: S) -> ! {
  eprintln!("Error: {}", message.as_ref());
  std::process::exit(code as i32);
}

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
  
  /// Report metadata from an emg .wasm file
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
  
  /// Name of model generator within .wasm file to use
  generator: String,
  
  /// Parameters to pass to model generator
  parameters: Vec<String>,
  
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

// .engine and .module aren't accessed, I just keep them here to remind myself of
// the wasmtime variables
#[allow(dead_code)]
struct EMGModule {
  engine: wasmtime::Engine,
  module: wasmtime::Module,
  store: wasmtime::Store<()>,
  instance: wasmtime::Instance,
  
  generator_names: Vec<String>,
}

impl EMGModule {
  fn from_file(wasm: PathBuf) -> Self {
    let engine = wasmtime::Engine::default();
    
    let module = match wasmtime::Module::from_file(&engine, wasm) {
      Ok(m) => m,
      Err(e) => fail(emg::ErrorCode::WebAssemblyCompile,
        format!("Unable to compile .wasm file: {:?}", e)),
    };
    
    // The "store" seems to be wasmtime's container for instance data
    let mut store = wasmtime::Store::new(&engine, ());
    
    let instance = match wasmtime::Instance::new(&mut store, &module, &[]) {
      Ok(i) => i,
      Err(e) => fail(emg::ErrorCode::WebAssemblyInstance,
        format!("Unable to instantiate WebAssembly module: {:?}", e)),
    };
    
    let mut result = Self { engine, module, store, instance,
      generator_names: Vec::new() };
    
    result.validate_pointer_accessor("model_pointer");
    result.validate_pointer_accessor("model_size");
    
    let mut possible_model_generators = Vec::new();
    
    for export in result.instance.exports(&mut result.store) {
      if export.name().starts_with("gen_") {
        // Converting to String "launders" the name to break all links with the
        // original string. Required by borrow checker
        possible_model_generators.push(String::from(export.name()));
      }
    }
    
    // Generator validation must be done outside the loop in which possible
    // generators are found due to borrowing issues related to the store
    for name in possible_model_generators {
      result.validate_model_generator(name.as_str());
      result.generator_names.push(name);
    }
    
    result
  }
  
  fn validate_model_generator(&mut self, name: &str) {
    let export = match self.instance.get_export(&mut self.store, name) {
      Some(function) => function,
      None => fail(emg::ErrorCode::ModelGeneratorNotFound, format!(".wasm file \
        does not contain model generator `{}`", name)),
    };
    
    let generator = match export.into_func() {
      Some(function) => function,
      None => fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a \
          valid emg module: export `{}` must be a function", name)),
    };
    
    if generator.ty(&self.store).results().len() != 1 {
      fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a valid \
        emg module: function `{}()` must return one result", name));
    }
    // .unwrap() acceptable here because the length is asserted == 1
    match generator.ty(&self.store).results().next().unwrap() {
      wasmtime::ValType::I32 => {},
      _ => fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a \
        valid emg module: function `{}()` must return a 32-bit integer", name)),
    }
  }
  
  fn validate_pointer_accessor(&mut self, name: &str) {
    let export = match self.instance.get_export(&mut self.store, name) {
      Some(function) => function,
      None => fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a \
        valid emg module: missing required function `{}()`", name)),
    };
    
    let accessor = match export.into_func() {
      Some(function) => function,
      None => fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a \
          valid emg module: export `{}` must be a function", name)),
    };
    
    if accessor.ty(&self.store).params().len() != 0 {
      fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a valid \
        emg module: function `{}()` must not accept any arguments", name));
    }
    if accessor.ty(&self.store).results().len() != 1 {
      fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a valid \
        emg module: function `{}()` must return one result", name));
    }
    // .unwrap() acceptable here because the length is asserted == 1
    match accessor.ty(&self.store).results().next().unwrap() {
      wasmtime::ValType::I32 => {},
      _ => fail(emg::ErrorCode::ModuleNotEMG, format!(".wasm file is not a \
        valid emg module: function `{}()` must return a 32-bit integer", name)),
    }
  }
}

fn gen(args: ArgsForGen) {
  let emg_module = EMGModule::from_file(args.wasm);
  let mut store = emg_module.store;
  let instance = emg_module.instance;
  
  let generator = match instance.get_func(&mut store,
    (String::from("gen_") + &args.generator).as_str(),
  ) {
    Some(f) => f,
    None => fail(emg::ErrorCode::ModelGeneratorNotFound, format!(".wasm file \
      does not contain model generator `{}`", args.generator)),
  };
  
  let parameter_count = generator.ty(&store).params().len();
  if args.parameters.len() != parameter_count {
    fail(emg::ErrorCode::ParameterCount,
      format!("model generator expects {} parameters, but {} were given",
      parameter_count, args.parameters.len()));
  }
  
  let mut generator_args: Vec<wasmtime::Val> = Vec::new();
  let mut i = 0;
  for parameter in generator.ty(&store).params() {
    match parameter {
      wasmtime::ValType::I32 => {
        match args.parameters[i].parse::<i32>() {
          Ok(v) => generator_args.push(wasmtime::Val::from(v)),
          Err(_) => fail(emg::ErrorCode::ParameterType,
            format!("model generator parameter {} (set to `{}`) should be a \
            32-bit integer", i + 1, args.parameters[i])),
        }
      },
      
      wasmtime::ValType::I64 => {
        match args.parameters[i].parse::<i64>() {
          Ok(v) => generator_args.push(wasmtime::Val::from(v)),
          Err(_) => fail(emg::ErrorCode::ParameterType,
            format!("model generator parameter {} (set to `{}`) should be a \
            64-bit integer", i + 1, args.parameters[i])),
        }
      },
      
      wasmtime::ValType::F32 => {
        match args.parameters[i].parse::<f32>() {
          Ok(v) => generator_args.push(wasmtime::Val::from(v)),
          Err(_) => fail(emg::ErrorCode::ParameterType,
            format!("model generator parameter {} (set to `{}`) should be a \
            32-bit floating-point value", i + 1, args.parameters[i])),
        }
      },
      
      wasmtime::ValType::F64 => {
        match args.parameters[i].parse::<f64>() {
          Ok(v) => generator_args.push(wasmtime::Val::from(v)),
          Err(_) => fail(emg::ErrorCode::ParameterType,
            format!("model generator parameter {} (set to `{}`) should be a \
            64-bit floating-point value", i + 1, args.parameters[i])),
        }
      },
      
      _ => fail(emg::ErrorCode::ParameterType,
        "emg model generators only support parameters of type i32, \
        i64, f32, or f64"),
    };
    
    i += 1;
  }
  
  if args.verbose {
    eprintln!("Extracted arguments for passing through to model generator: \
      {:?}", generator_args);
  }
  
  let mut result = [wasmtime::Val::from(0)];
  match generator.call(&mut store, &generator_args, &mut result) {
    Ok(_) => {},
    Err(e) => fail(emg::ErrorCode::WebAssemblyExecution,
      format!("WebAssembly execution failed: {:?}", e)),
  }
  // .unwrap() acceptable here because the type was previously asserted
  match result[0].i32().unwrap() {
    0 => {},
    e => {
      // I don't use emg::ErrorCode here because rust makes it hard to convert
      // an integer to an enum
      eprintln!("model generation returned error code: {}", e);
      std::process::exit(e);
    },
  }
  
  // Can use .unwrap() because validator checked this exists
  let get_pointer = instance.get_func(&mut store, "model_pointer").unwrap();
  let mut pointer = [wasmtime::Val::from(0)];
  match get_pointer.call(&mut store, &[], &mut pointer) {
    Ok(_) => {},
    Err(e) => fail(emg::ErrorCode::WebAssemblyExecution,
      format!("Unable to retrieve model: {:?}", e)),
  }
  if args.verbose { eprintln!("Got pointer: {:?}", pointer) }
  
  // Can use .unwrap() because validator checked this exists
  let get_size = instance.get_func(&mut store, "model_size").unwrap();
  let mut size = [wasmtime::Val::from(0)];
  match get_size.call(&mut store, &[], &mut size) {
    Ok(_) => {},
    Err(e) => fail(emg::ErrorCode::WebAssemblyExecution,
      format!("Unable to retrieve model: {:?}", e)),
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
    Err(e) => fail(emg::ErrorCode::OutputNotGLB,
      format!("model generated with invalid JSON: {:?}", e)),
  };
  
  match args.format {
    // Can .unwrap() because this JSON was just parsed, so it must be valid
    Format::Pretty => {
      println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    },
    Format::GLTF => println!("{}", serde_json::to_string(&parsed).unwrap()),
    Format::GLB => fail(emg::ErrorCode::NotImplemented,
      ".glb generation is not supported yet"),
  }
}

fn serve(_args: ArgsForServe) {
  fail(emg::ErrorCode::NotImplemented, "Serve not implemented yet");
}

fn inspect(args: ArgsForInspect) {
  let mut emg_module = EMGModule::from_file(args.wasm);
  
  eprintln!("Model generators found:");
  
  for name in emg_module.generator_names {
    // Can .unwrap() because these functions were already validated
    let generator = emg_module.instance.get_func(&mut emg_module.store,
      name.as_str()).unwrap();
    
    let mut parameter_strings = Vec::new();
    
    for parameter in generator.ty(&emg_module.store).params() {
      let mut string = String::new();
      // .unwrap() because I don't understand how writing a string could fail
      write!(string, "{}", parameter).unwrap();
      parameter_strings.push(string);
    }
    
    eprintln!("\t{} ( {} )", &name[4..], parameter_strings.join(", "))
  }
}

fn main() {
  let args = Cli::parse();
  
  match args.subcommand {
    Subcommands::Gen(args) => gen(args),
    Subcommands::Serve(args) => serve(args),
    Subcommands::Inspect(args) => inspect(args),
  };
}
