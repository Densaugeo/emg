use std::path::PathBuf;
use std::fmt::Write as _;
use std::io::Write as _;

use clap::Parser;
use base64::Engine;

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
    /// GLTF binary format (.glb)
    #[default]
    GLB,
    
    /// GLTF text format (.gltf)
    GLTF,
    
    /// Pretty-printed GLTF text format (for debugging, may omit buffers
    /// required for rendering)
    Pretty,
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

struct ChunkMetadata {
  start: u32,
  length: u32,
}

struct GLBMetadata {
  json: ChunkMetadata,
  bin: Option<ChunkMetadata>,
}

impl GLBMetadata {
  fn from_glb(glb: &[u8]) -> Self {
    if glb.len() < 24 {
      fail(emg::ErrorCode::OutputNotGLB, format!("Generated output is too \
        small ({} bytes) to contain required .glb headers (20 bytes)",
        glb.len()))
    }
    
    // Can .unwrap() because .glb size was just checked
    let magic = String::from_utf8_lossy(glb[0..4].try_into().unwrap());
    let version     = u32::from_le_bytes(glb[ 4.. 8].try_into().unwrap());
    let length      = u32::from_le_bytes(glb[ 8..12].try_into().unwrap());
    
    let json_length = u32::from_le_bytes(glb[12..16].try_into().unwrap());
    let json_type = String::from_utf8_lossy(glb[16..20].try_into().unwrap());
    
    if magic != "glTF" {
      fail(emg::ErrorCode::OutputNotGLB, format!("Generated output does not \
        begin with magic bytes `glTF` required in .glb files (has `{}` \
        instead)", magic))
    }
    
    if version != 2 {
      fail(emg::ErrorCode::NotImplemented, format!("Generated output gives a \
        .glb container version of {}, but this tool only supports version 2",
        version))
    }
    
    if length != glb.len() as u32 {
      fail(emg::ErrorCode::OutputNotGLB, format!("Header in generated output \
        gives a length of {} bytes, but output is {} bytes", length, glb.len()))
    }
    
    if length % 4 > 0 {
      fail(emg::ErrorCode::OutputNotGLB, format!("Generated output is {} \
        bytes, but .glb files must be multiples of 4 bytes", length))
    }
    
    if json_type != "JSON" {
      fail(emg::ErrorCode::OutputNotGLB, format!("First chunk in generated \
        output must be a JSON chunk, but is labeled `{}`", json_type))
    }
    
    if json_length > length - 20 {
      fail(emg::ErrorCode::OutputNotGLB, format!("JSON chunk header in \
        generated output gives a length of {} bytes, but output is only long \
        enough for up to {} bytes of JSON", json_length, length - 20))
    }
    
    if json_length % 4 > 0 {
      fail(emg::ErrorCode::OutputNotGLB, format!("JSON chunk in generated \
        output is {} bytes, but all chunks in .glb files must be multiples of \
        4 bytes", json_length))
    }
    
    if 0 < length - 20 - json_length && length - 20 - json_length < 8 {
      fail(emg::ErrorCode::OutputNotGLB, format!("Remaining space after JSON \
        chunk in generated output is too small for a valid .glb chunk ({} \
        bytes remaining, but a chunk header is 8 bytes)", length - 20 -
        json_length))
    }
    
    let second_chunk_present = length > 20 + json_length + 8;
    if !second_chunk_present {
      return GLBMetadata {
        json: ChunkMetadata { start: 20, length: json_length },
        bin: None,
      };
    }
    
    // Can .unwrap() because .glb size was just checked
    let bin_chunk_start = 20 + json_length;
    let bin_data_start  = 20 + json_length + 8;
    let bin_length = u32::from_le_bytes(
      glb[(bin_chunk_start    ) as usize..(bin_chunk_start + 4) as usize]
      .try_into().unwrap());
    let bin_type = String::from_utf8_lossy(
      glb[(bin_chunk_start + 4) as usize..(bin_chunk_start + 8) as usize]
      .try_into().unwrap());
    
    if bin_type != "BIN\0" {
      fail(emg::ErrorCode::OutputNotGLB, format!("Second chunk in generated \
        output must be a BIN chunk, but is labeled `{}`", bin_type))
    }
    
    if bin_data_start + bin_length > length {
      fail(emg::ErrorCode::OutputNotGLB, format!("BIN chunk header in \
        generated output gives a length of {} bytes, but output is only long \
        enough for up to {} bytes of BIN", bin_length, length - bin_data_start))
    }
    
    if bin_length % 4 > 0 {
      fail(emg::ErrorCode::OutputNotGLB, format!("BIN chunk in generated \
        output is {} bytes, but all chunks in .glb files must be multiples of \
        4 bytes", bin_length))
    }
    
    if bin_data_start + bin_length < length {
      fail(emg::ErrorCode::NotImplemented, format!("Generated output contains \
        additional space ({} bytes) after the JSON and BIN chunks, but this \
        tool does not support any other chunk types", length - bin_data_start -
        bin_length))
    }
    
    GLBMetadata {
      json: ChunkMetadata { start: 20, length: json_length },
      bin: Some(ChunkMetadata { start: bin_data_start, length: bin_length }),
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
  
  let glb_metadata = GLBMetadata::from_glb(memory_of_interest);
  
  let mut parsed: serde_json::Value = match serde_json::de::from_slice(
    &memory_of_interest[glb_metadata.json.start as usize..
    (glb_metadata.json.start + glb_metadata.json.length) as usize]) {
    Ok(json) => json,
    Err(e) => fail(emg::ErrorCode::OutputNotGLB,
      format!("model generated with invalid JSON: {:?}", e)),
  };
  
  match args.format {
    // Can .unwrap() because this JSON was just parsed, so it must be valid
    Format::Pretty => {
      println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    },
    Format::GLTF => {
      match glb_metadata.bin {
        None => {},
        Some(bin_metadata) => {
          let buffers = match parsed.get_mut("buffers") {
            Some(v) => v,
            None => fail(emg::ErrorCode::OutputNotGLB, "No `buffers` field \
              present in generated output"),
          };
          
          let buffer_0 = match buffers.get_mut(0) {
            Some(v) => v,
            None => fail(emg::ErrorCode::OutputNotGLB, "`buffers` field in \
              generated output has no entries"),
          };
          
          let buffer_0_as_object = match buffer_0.as_object_mut() {
            Some(v) => v,
            None => fail(emg::ErrorCode::OutputNotGLB, "`buffer[0]` field in \
              generated output is not an object"),
          };
          
          let mut base64_buffer = String::from("data:application/octet-stream;\
            base64,");
          
          base64::engine::general_purpose::STANDARD.encode_string(
            &memory_of_interest[bin_metadata.start as usize..
            (bin_metadata.start + bin_metadata.length) as usize],
            &mut base64_buffer);
          
          buffer_0_as_object.insert(String::from("uri"),
            serde_json::Value::String(base64_buffer));
        },
      }
      
      println!("{}", serde_json::to_string(&parsed).unwrap());
    },
    Format::GLB => std::io::stdout().write_all(&memory_of_interest).unwrap(),
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
