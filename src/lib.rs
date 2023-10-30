use std::sync::Mutex;
use std::sync::atomic::{Ordering, AtomicU32};

pub mod prelude {
  pub use paragen_macros::paragen;
  pub use crate::Scene;
}

static MUTEX_TEST: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static POINTER: AtomicU32 = AtomicU32::new(0);
static SIZE: AtomicU32 = AtomicU32::new(0);

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

#[derive(serde::Serialize)]
pub struct Scene {
  pub name: String,
  pub nodes: Vec<i32>,
}

#[derive(serde::Serialize)]
pub struct Node {
  pub name: String,
  pub mesh: i32,
}

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

pub fn write_gltf(scene: Scene) -> Result<(), i32> {
  let mut dry_run_writer = DryRunWriter::new();
  serde_json::ser::to_writer(&mut dry_run_writer, &scene).unwrap();
  let space_required = dry_run_writer.bytes_written;
  
  match MUTEX_TEST.try_lock() {
    Ok(mut guard) => {
      guard.reserve_exact(space_required);
      serde_json::ser::to_writer(&mut (*guard), &scene).unwrap();
      guard.shrink_to_fit();
      
      POINTER.store((*guard).as_ptr() as u32, Ordering::Relaxed);
      SIZE.store(guard.len() as u32, Ordering::Relaxed);
    },
    // TODO Find a way to throw this error to JS
    Err(_) => return Err(2),
  }
  
  Ok(())
}

pub fn clear_gltf() -> Result<(), i32> {
  match MUTEX_TEST.try_lock() {
    Ok(mut guard) => *guard = Vec::new(),
    // TODO Find a way to throw this error to JS
    Err(_e) => return Err(1),
  }
  
  Ok(())
}
