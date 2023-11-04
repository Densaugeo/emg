use std::sync::Mutex;
use std::sync::atomic::{Ordering, AtomicU32};

pub mod prelude {
  pub use paragen_macros::paragen;
  pub use crate::GLTF;
  pub use crate::Scene;
  pub use crate::Node;
  pub use crate::ErrorCode;
}

pub static MUTEX_TEST: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static POINTER: AtomicU32 = AtomicU32::new(0);
static SIZE: AtomicU32 = AtomicU32::new(0);

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn pointer() -> i32 {
  POINTER.load(Ordering::Relaxed) as i32
}

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn size() -> i32 {
  SIZE.load(Ordering::Relaxed) as i32
}

// These error codes are return from WebAssembly functions, so must use a
// WebAssembly variable type
#[repr(i32)]
pub enum ErrorCode {
    None = 0,
    Mutex = 1,
    Generation = 2,
}

struct DryRunWriter {
  bytes_written: usize,
}

impl DryRunWriter {
  fn new() -> Self {
    Self { bytes_written: 0 }
  }
}

impl std::io::Write for DryRunWriter {
  fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
    self.bytes_written += buf.len();
    Ok(buf.len())
  }
  
  fn flush(&mut self) -> Result<(), std::io::Error> {
    Ok(())
  }
}

#[derive(Clone, serde::Serialize)]
pub struct GLTF {
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<Node>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub meshes: Vec<u8>,
}

impl GLTF {
  pub fn new() -> Self {
    Self { nodes: Vec::new(), meshes: Vec::new() }
  }
  
  // In the .gltf spec, but will have to wait for later
  /*pub fn accessors() -> ??
  pub fn animations() -> ??
  pub fn asset() -> ??
  pub fn extensionsUsed() -> ??
  pub fn extensionsRequired() -> ??
  pub fn buffers() -> ??
  pub fn bufferViews() -> ??
  pub fn cameras() -> ??
  pub fn images() -> ??
  pub fn materials() -> ??
  pub fn meshes() -> ??
  pub fn samplers() -> ??
  pub fn scene() -> ??
  pub fn scenes() -> ??
  pub fn skins() -> ??
  pub fn textures() -> ??
  pub fn extensions() -> ??
  pub fn extras() -> ??*/
}

#[derive(Clone, serde::Serialize)]
pub struct Scene {
  pub name: String,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<u32>,
  
  //pub extensions: Vec<??>,
  
  // In the .gltf spec but not currently used:
  //pub extras: Vec<A JSON-serializable struct>,
}

impl Scene {
  pub fn new() -> Self {
    Self { name: "".to_string(), nodes: Vec::new() }
  }
}

//pub type Translation = [f64; 3];

pub trait Translation {
  fn new() -> Self;
  fn is_default(&self) -> bool;
}

impl Translation for [f64; 3] {
  fn new() -> Self {
    [0.0; 3]
  }
  
  fn is_default(&self) -> bool {
    //self == Self::new()
    
    self[0] == 0.0 && self[1] == 0.0 && self[2] == 0.0
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Node {
  pub name: String,
  
  #[serde(skip_serializing_if = "Translation::is_default")]
  pub translation: [f64; 3],
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub children: Vec<u32>,
  
  //pub mesh: ??,
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub camera: ??,
  pub skin: ??,
  pub matrix: ??,
  pub weights: ??,
  pub extras: ??,*/
}

impl Node {
  pub fn new(name: String) -> Self {
    Self { name, translation: Translation::new(), children: Vec::new() }
  }
  
  pub fn rotation(&self) -> Option<[f64; 4]> { None }
  pub fn scale(&self) -> Option<[f64; 3]> { None }
}

pub fn write_gltf(buffer: &mut Vec<u8>, gltf: GLTF) {
  let mut dry_run_writer = DryRunWriter::new();
  serde_json::ser::to_writer(&mut dry_run_writer, &gltf).unwrap();
  let space_required = dry_run_writer.bytes_written;
  
  buffer.reserve_exact(space_required);
  serde_json::ser::to_writer(&mut (*buffer), &gltf).unwrap();
  buffer.shrink_to_fit();
  
  POINTER.store(buffer.as_ptr() as u32, Ordering::Relaxed);
  SIZE.store(buffer.len() as u32, Ordering::Relaxed);
}
