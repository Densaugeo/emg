use clap::Parser;

#[derive(Clone)]
enum WebAssemblyArg {
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long)]
  name: String,
  
  #[arg(long)]
  arg_test: WebAssemblyArg,
  
  /// Number of times to greet
  #[arg(short, long, default_value_t = 1)]
  count: u8,
}

fn main() {
  let args = Args::parse();
  
  for _ in 0..args.count {
    println!("Hello {}!", args.name)
  }
}
