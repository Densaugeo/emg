use std::sync::Mutex;
use std::sync::atomic::{Ordering, AtomicU32};

pub mod prelude {
  pub use emg_macros::emg;
  pub use crate::GLTF;
  pub use crate::Scene;
  pub use crate::Node;
  pub use crate::ErrorCode;
}

pub static MUTEX_TEST: Mutex<Vec<u8>> = Mutex::new(Vec::new());
static MODEL_POINTER: AtomicU32 = AtomicU32::new(0);
static MODEL_SIZE: AtomicU32 = AtomicU32::new(0);

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn model_pointer() -> i32 {
  MODEL_POINTER.load(Ordering::Relaxed) as i32
}

// WebAssembly is rumored to always be 32 bit, so assume that's the pointer size
#[no_mangle]
pub extern "C" fn model_size() -> i32 {
  MODEL_SIZE.load(Ordering::Relaxed) as i32
}

// These error codes are returned from WebAssembly functions, so must use a
// WebAssembly variable type
#[derive(Debug, Clone)]
#[repr(i32)]
pub enum ErrorCode {
  None = 0,
  Mutex = 1,
  Generation = 2,
  NotImplemented = 3,
  WebAssemblyCompile = 4,
  WebAssemblyInstance = 5,
  WebAssemblyExecution = 6,
  ModuleNotEMG = 7,
  ModelGeneratorNotFound = 8,
  ParameterCount = 9,
  ParameterType = 10,
  ParameterOutOfRange = 11,
  OutputNotGLB = 12,
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
pub struct Asset {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub copyright: String,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub generator: String,
  
  // Don't skip if empty...this field is mandatory per GLTF spec!
  pub version: String,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  #[serde(rename = "minVersion")]
  pub min_version: String,
  
  // pub extensions: ??,
  
  // In the .gltf spec, but will have to wait for later
  //pub extra: ??,
}

impl Asset {
  pub fn new() -> Self {
    Self {
      copyright: String::from(""),
      generator: String::from("emg v0.1.0"),
      version: String::from("2.0"),
      min_version: String::from("2.0"),
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct GLTF {
  // Don't skip if empty...this field is mandatory per GLTF spec!
  pub asset: Asset,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scene: Option<u32>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub scenes: Vec<Scene>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<Node>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub materials: Vec<Material>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub meshes: Vec<Mesh>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub accessors: Vec<Accessor>,
  
  #[serde(rename = "bufferViews")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub buffer_views: Vec<BufferView>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub buffers: Vec<Buffer>,
  
  // TODO Not sure about the memory use effects of putting all GLB BIN data
  // into one vector during model construction. Look into using a
  // Vec<Vec<u8>> or similar when I have a suitable test setup
  #[serde(skip_serializing)]
  pub glb_bin: Vec<u8>,
  
  // In the .gltf spec, but will have to wait for later
  /*pub animations: ??
  pub asset: ??
  pub extensionsUsed: ??
  pub extensionsRequired: ??
  pub cameras: ??
  pub images: ??
  pub samplers: ??
  pub skins: ??
  pub textures: ??
  pub extensions: ??
  pub extras: ??*/
}

pub trait GLTFBufferElement {
  fn get_type() -> Type;
  fn get_component_type() -> ComponentType;
}

macro_rules! gltf_buffer_element {
  ($type_:ty, $gltf_type:expr, $gltf_component_type:expr) => {
    impl GLTFBufferElement for $type_ {
      fn get_type() -> Type { $gltf_type }
      fn get_component_type() -> ComponentType { $gltf_component_type }
    }
  };
}

gltf_buffer_element!( u8, Type::SCALAR, ComponentType::UnsignedByte );
gltf_buffer_element!( i8, Type::SCALAR, ComponentType::Byte         );
gltf_buffer_element!(u16, Type::SCALAR, ComponentType::UnsignedShort);
gltf_buffer_element!(i16, Type::SCALAR, ComponentType::Short        );
gltf_buffer_element!(u32, Type::SCALAR, ComponentType::UnsignedInt  );
gltf_buffer_element!(f32, Type::SCALAR, ComponentType::Float        );

gltf_buffer_element!([ u8; 2], Type::VEC2, ComponentType::UnsignedByte );
gltf_buffer_element!([ i8; 2], Type::VEC2, ComponentType::Byte         );
gltf_buffer_element!([u16; 2], Type::VEC2, ComponentType::UnsignedShort);
gltf_buffer_element!([i16; 2], Type::VEC2, ComponentType::Short        );
gltf_buffer_element!([u32; 2], Type::VEC2, ComponentType::UnsignedInt  );
gltf_buffer_element!([f32; 2], Type::VEC2, ComponentType::Float        );

gltf_buffer_element!([ u8; 3], Type::VEC3, ComponentType::UnsignedByte );
gltf_buffer_element!([ i8; 3], Type::VEC3, ComponentType::Byte         );
gltf_buffer_element!([u16; 3], Type::VEC3, ComponentType::UnsignedShort);
gltf_buffer_element!([i16; 3], Type::VEC3, ComponentType::Short        );
gltf_buffer_element!([u32; 3], Type::VEC3, ComponentType::UnsignedInt  );
gltf_buffer_element!([f32; 3], Type::VEC3, ComponentType::Float        );

gltf_buffer_element!([ u8; 4], Type::VEC4, ComponentType::UnsignedByte );
gltf_buffer_element!([ i8; 4], Type::VEC4, ComponentType::Byte         );
gltf_buffer_element!([u16; 4], Type::VEC4, ComponentType::UnsignedShort);
gltf_buffer_element!([i16; 4], Type::VEC4, ComponentType::Short        );
gltf_buffer_element!([u32; 4], Type::VEC4, ComponentType::UnsignedInt  );
gltf_buffer_element!([f32; 4], Type::VEC4, ComponentType::Float        );

gltf_buffer_element!([[ u8; 2]; 2], Type::MAT2, ComponentType::UnsignedByte );
gltf_buffer_element!([[ i8; 2]; 2], Type::MAT2, ComponentType::Byte         );
gltf_buffer_element!([[u16; 2]; 2], Type::MAT2, ComponentType::UnsignedShort);
gltf_buffer_element!([[i16; 2]; 2], Type::MAT2, ComponentType::Short        );
gltf_buffer_element!([[u32; 2]; 2], Type::MAT2, ComponentType::UnsignedInt  );
gltf_buffer_element!([[f32; 2]; 2], Type::MAT2, ComponentType::Float        );

gltf_buffer_element!([[ u8; 3]; 3], Type::MAT3, ComponentType::UnsignedByte );
gltf_buffer_element!([[ i8; 3]; 3], Type::MAT3, ComponentType::Byte         );
gltf_buffer_element!([[u16; 3]; 3], Type::MAT3, ComponentType::UnsignedShort);
gltf_buffer_element!([[i16; 3]; 3], Type::MAT3, ComponentType::Short        );
gltf_buffer_element!([[u32; 3]; 3], Type::MAT3, ComponentType::UnsignedInt  );
gltf_buffer_element!([[f32; 3]; 3], Type::MAT3, ComponentType::Float        );

gltf_buffer_element!([[ u8; 4]; 4], Type::MAT4, ComponentType::UnsignedByte );
gltf_buffer_element!([[ i8; 4]; 4], Type::MAT4, ComponentType::Byte         );
gltf_buffer_element!([[u16; 4]; 4], Type::MAT4, ComponentType::UnsignedShort);
gltf_buffer_element!([[i16; 4]; 4], Type::MAT4, ComponentType::Short        );
gltf_buffer_element!([[u32; 4]; 4], Type::MAT4, ComponentType::UnsignedInt  );
gltf_buffer_element!([[f32; 4]; 4], Type::MAT4, ComponentType::Float        );

impl GLTF {
  pub fn new() -> Self {
    Self {
      asset: Asset::new(),
      nodes: Vec::new(),
      materials: Vec::new(),
      scene: None,
      scenes: Vec::new(),
      meshes: Vec::new(),
      accessors: Vec::new(),
      buffer_views: Vec::new(),
      buffers: Vec::new(),
      glb_bin: Vec::new(),
    }
  }
  
  /// Returns indices for automatically created Accessor and BufferView
  pub fn append_to_glb_bin<T: GLTFBufferElement>(&mut self, buffer: Vec<T>) {
    let mut accessor = Accessor::new();
    accessor.buffer_view = Some(self.buffer_views.len() as u32);
    accessor.type_ = T::get_type();
    accessor.component_type = T::get_component_type();
    accessor.count = buffer.len() as u32;
    self.accessors.push(accessor);
    
    let mut buffer_view = BufferView::new();
    buffer_view.buffer = 0;
    buffer_view.byte_length = (buffer.len() as u32)*(
      ::core::mem::size_of::<T>() as u32);
    buffer_view.byte_offset = self.glb_bin.len() as u32;
    self.buffer_views.push(buffer_view);
    
    for value in buffer {
      let sliced = unsafe { any_as_u8_slice(&value) };
      self.glb_bin.extend_from_slice(sliced);
    }
  }
}

// WARNING: Do not edit!
//
// Found this function here:
// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
//
// Getting something into raw bytes in Rust is absurdly overcomplicated. Code
// that does this is densely packed with subtle dangers, hidden complications,
// and unpleasant surprises. Do not attempt to edit it.
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
  ::core::slice::from_raw_parts(
    (p as *const T) as *const u8,
    ::core::mem::size_of::<T>(),
  )
}

#[derive(Clone, serde::Serialize)]
pub struct Scene {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub nodes: Vec<u32>,
  
  //pub extensions: Vec<??>,
  
  // In the .gltf spec but not currently used:
  //pub extras: Vec<A JSON-serializable struct>,
}

impl Scene {
  pub fn new() -> Self {
    Self { name: String::from(""), nodes: Vec::new() }
  }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Translation {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Translation {
  pub fn new() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Rotation {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Rotation {
  pub fn new() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Scale {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Scale {
  pub fn new() -> Self { Self { x: 1.0, y: 1.0, z: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, serde::Serialize)]
pub struct Node {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mesh: Option<u32>,
  
  #[serde(rename = "translation")]
  #[serde(skip_serializing_if = "Translation::is_default")]
  pub t: Translation,
  
  #[serde(rename = "rotation")]
  #[serde(skip_serializing_if = "Rotation::is_default")]
  pub r: Rotation,
  
  #[serde(rename = "scale")]
  #[serde(skip_serializing_if = "Scale::is_default")]
  pub s: Scale,
  
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
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      mesh: None,
      t: Translation::new(),
      r: Rotation::new(),
      s: Scale::new(),
      children: Vec::new(),
    }
  }
}

#[derive(Clone, PartialEq, serde::Serialize)]
pub enum AlphaMode {
  OPAQUE,
  MASK,
  BLEND,
}

#[derive(Clone, PartialEq)]
#[derive(serde_tuple::Serialize_tuple)]
pub struct Color4 {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

impl Color4 {
  pub fn new() -> Self { Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } }
  pub fn is_default(&self) -> bool { *self == Self::new() }
}

#[derive(Clone, serde::Serialize)]
pub struct PBRMetallicRoughness {
  #[serde(rename = "baseColorFactor")]
  #[serde(skip_serializing_if = "Color4::is_default")]
  pub base_color_factor: Color4,
  
  #[serde(rename = "metallicFactor")]
  #[serde(skip_serializing_if = "is_default_metallic_factor")]
  pub metallic_factor: f64,
  
  #[serde(rename = "roughnessFactor")]
  #[serde(skip_serializing_if = "is_default_roughness_factor")]
  pub roughness_factor: f64,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub metallicRoughnessTexture: ??,
  pub baseColorTexture: ??,
  */
}

impl PBRMetallicRoughness {
  pub fn new() -> Self {
    Self {
      base_color_factor: Color4::new(),
      metallic_factor: 1.0,
      roughness_factor: 1.0,
    }
  }
}

fn is_default_metallic_factor(value: &f64) -> bool {
  *value == 1.0
}

fn is_default_roughness_factor(value: &f64) -> bool {
  *value == 1.0
}

fn is_default_emissive_factor(value: &[f64; 3]) -> bool {
  *value == [0.0, 0.0, 0.0]
}

fn is_default_alpha_mode(value: &AlphaMode) -> bool {
  *value == AlphaMode::OPAQUE
}

fn is_default_alpha_cutoff(value: &f64) -> bool {
  *value == 0.5
}

fn is_default_double_sided(value: &bool) -> bool {
  *value == false
}

#[derive(Clone, serde::Serialize)]
pub struct Material {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "emissiveFactor")]
  #[serde(skip_serializing_if = "is_default_emissive_factor")]
  pub emissive_factor: [f64; 3],
  
  #[serde(rename = "alphaMode")]
  #[serde(skip_serializing_if = "is_default_alpha_mode")]
  pub alpha_mode: AlphaMode,
  
  #[serde(rename = "alphaCutoff")]
  #[serde(skip_serializing_if = "is_default_alpha_cutoff")]
  pub alpha_cutoff: f64,
  
  #[serde(rename = "doubleSided")]
  #[serde(skip_serializing_if = "is_default_double_sided")]
  pub double_sided: bool,
  
  #[serde(rename = "pbrMetallicRoughness")]
  // Not sure how to skip serializing when unused for this one
  pub pbr_metallic_roughness: PBRMetallicRoughness,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub normalTexture: ??,
  pub occlusionTexture: ??,
  pub emissiveTexture: ??,*/
}

impl Material {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      emissive_factor: [0.0, 0.0, 0.0],
      alpha_mode: AlphaMode::OPAQUE,
      alpha_cutoff: 0.5,
      double_sided: false,
      pbr_metallic_roughness: PBRMetallicRoughness::new(),
    }
  }
}

// The fields here are in the spec in section 3.7 - Concepts / Geometry,
// which took me a while to find
#[derive(Clone, serde::Serialize)]
pub struct Attributes {
  #[serde(rename = "COLOR_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub color_0: Option<u32>,
  
  #[serde(rename = "JOINTS_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub joints_0: Option<u32>,
  
  #[serde(rename = "NORMAL")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub normal: Option<u32>,
  
  #[serde(rename = "POSITION")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<u32>,
  
  #[serde(rename = "TANGENT")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tangent: Option<u32>,
  
  #[serde(rename = "TEXCOORD_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_0: Option<u32>,
  
  #[serde(rename = "TEXCOORD_1")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_1: Option<u32>,
  
  #[serde(rename = "TEXCOORD_2")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_2: Option<u32>,
  
  #[serde(rename = "TEXCOORD_3")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub texcoord_3: Option<u32>,
  
  #[serde(rename = "WEIGHTS_0")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub weights_0: Option<u32>,
}

impl Attributes {
  pub fn new() -> Self {
    Self {
      color_0: None,
      joints_0: None,
      normal: None,
      position: None,
      tangent: None,
      texcoord_0: None,
      texcoord_1: None,
      texcoord_2: None,
      texcoord_3: None,
      weights_0: None,
    }
  }
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum Mode {
  Points = 0,
  Lines = 1,
  LineLoop = 2,
  LineStrip = 3,
  Triangles = 4,
  TriangleStrip = 5,
  TriangleFan = 6,
}

fn is_default_mode(value: &Mode) -> bool {
  *value == Mode::Triangles
}

#[derive(Clone, serde::Serialize)]
pub struct MeshPrimitive {
  pub attributes: Attributes,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub indices: Option<u32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub material: Option<u32>,
  
  #[serde(skip_serializing_if = "is_default_mode")]
  pub mode: Mode, // Default is triangles
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,
  pub targets: ??,*/
}

impl MeshPrimitive {
  pub fn new() -> Self {
    Self {
      attributes: Attributes::new(),
      indices: None,
      material: None,
      mode: Mode::Triangles,
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Mesh {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  // No serialization filter, this is required per spec
  pub primitives: Vec<MeshPrimitive>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub weights: Vec<f64>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl Mesh {
  pub fn new() -> Self {
    Self {
      primitives: Vec::new(),
      weights: Vec::new(),
      name: String::from(""),
    }
  }
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u16)]
pub enum ComponentType {
  Byte = 5120,
  UnsignedByte = 5121,
  Short = 5122,
  UnsignedShort = 5123,
  UnsignedInt = 5125,
  Float = 5126,
}

#[derive(Clone, serde::Serialize)]
pub enum Type {
  SCALAR,
  VEC2,
  VEC3,
  VEC4,
  MAT2,
  MAT3,
  MAT4,
}

#[derive(Clone, serde::Serialize)]
pub struct Accessor {
  // Next time I modify this, I want to try out:
  // #[serde(rename_all = "camelCase")]
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "bufferView")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub buffer_view: Option<u32>,
  
  #[serde(rename = "byteOffset")]
  #[serde(skip_serializing_if = "is_default_byte_offset")]
  pub byte_offset: u32,
  
  #[serde(rename = "componentType")]
  pub component_type: ComponentType,
  
  #[serde(skip_serializing_if = "is_default_normalized")]
  pub normalized: bool,
  
  pub count: u32,
  
  #[serde(rename = "type")]
  pub type_: Type,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub max: Vec<f64>,
  
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub min: Vec<f64>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /* pub max: ??,
  pub min: ??,
  pub sparse: ??,
  pub extras: ??,*/
}

impl Accessor {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      buffer_view: None,
      byte_offset: 0,
      component_type: ComponentType::Byte,
      normalized: false,
      count: 0,
      type_: Type::SCALAR,
      min: Vec::new(),
      max: Vec::new(),
    }
  }
}

fn is_default_byte_offset(value: &u32) -> bool {
  *value == 0
}

fn is_default_normalized(value: &bool) -> bool {
  *value == false
}

#[derive(Clone, PartialEq, serde_repr::Serialize_repr)]
#[repr(u16)]
pub enum Target {
  ArrayBuffer = 34962,
  ElementArrayBuffer = 34963,
}

#[derive(Clone, serde::Serialize)]
pub struct BufferView {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  pub buffer: u32,
  
  #[serde(rename = "byteLength")]
  pub byte_length: u32,
  
  #[serde(rename = "byteOffset")]
  pub byte_offset: u32,
  
  #[serde(rename = "byteStride")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byte_stride: Option<u32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target: Option<Target>,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl BufferView {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      buffer: 0,
      byte_length: 0,
      byte_offset: 0,
      byte_stride: None,
      target: None,
    }
  }
}

#[derive(Clone, serde::Serialize)]
pub struct Buffer {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  
  #[serde(rename = "byteLength")]
  pub byte_length: u32,
  
  #[serde(skip_serializing_if = "String::is_empty")]
  pub uri: String,
  
  //pub extensions: ??,
  
  // In the .gltf spec but will have to wait for now:
  /*pub extras: ??,*/
}

impl Buffer {
  pub fn new() -> Self {
    Self {
      name: String::from(""),
      byte_length: 0,
      uri: String::from(""),
    }
  }
}

pub fn write_gltf(buffer: &mut Vec<u8>, gltf: GLTF) {
  let mut dry_run_writer = DryRunWriter::new();
  serde_json::ser::to_writer(&mut dry_run_writer, &gltf).unwrap();
  
  // Per GLB spec, the length field of each chunk EXCLUDES headers and INCLUDES 
  // padding
  let json_padding = (4 - dry_run_writer.bytes_written % 4) % 4;
  let json_length = dry_run_writer.bytes_written + json_padding;
  let bin_padding = (4 - gltf.glb_bin.len() % 4) % 4;
  let bin_length = gltf.glb_bin.len() + bin_padding;
  
  // Per GLB spec, overall length field INCLUDES headers
  let mut glb_length = 12 + 8 + json_length;
  if gltf.glb_bin.len() > 0 {
    glb_length += 8 + bin_length;
  }
  
  buffer.reserve_exact(glb_length);
  
  // GLB header
  buffer.append(&mut String::from("glTF").into_bytes());
  buffer.extend_from_slice(&2u32.to_le_bytes()); // GLTF version #
  buffer.extend_from_slice(&(glb_length).to_le_bytes());
  
  // JSON chunk
  buffer.extend_from_slice(&(json_length).to_le_bytes());
  buffer.append(&mut String::from("JSON").into_bytes());
  serde_json::ser::to_writer(&mut (*buffer), &gltf).unwrap();
  for _ in 0..json_padding {
    // Per GLB spec, JSON chunk is padded with ASCII spaces
    buffer.push(0x20);
  }
  
  // BIN chunk
  if gltf.glb_bin.len() > 0 {
    buffer.extend_from_slice(&(bin_length).to_le_bytes());
    buffer.append(&mut String::from("BIN\0").into_bytes());
    buffer.extend(gltf.glb_bin);
    for _ in 0..bin_padding {
      // Per GLB spec, BIN chunk is padded with zeroes
      buffer.push(0);
    }
  }
  
  buffer.shrink_to_fit();
  
  MODEL_POINTER.store(buffer.as_ptr() as u32, Ordering::Relaxed);
  MODEL_SIZE.store(buffer.len() as u32, Ordering::Relaxed);
}
