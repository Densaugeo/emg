use base64::Engine;

use paragen::prelude::*;

#[paragen]
fn build_the_model(_a: i32) -> Result<GLTF, ErrorCode> {
  let mut gltf = GLTF::new();
  
  let mut scene = Scene::new();
  scene.name = String::from("A name for a scene");
  scene.nodes.push(0);
  gltf.scenes.push(scene);
  gltf.scene = Some(0);
  
  let mut node = Node::new();
  node.name = String::from("Fortress Wall Battlement");
  node.mesh = Some(0);
  gltf.nodes.push(node);
  
  let mut material_red = paragen::Material::new();
  material_red.name = String::from("Red");
  material_red.pbr_metallic_roughness.metallic_factor = 0.0;
  material_red.pbr_metallic_roughness.roughness_factor = 0.5;
  material_red.pbr_metallic_roughness.base_color_factor.r = 1.0;
  material_red.pbr_metallic_roughness.base_color_factor.g = 0.0;
  material_red.pbr_metallic_roughness.base_color_factor.b = 0.0;
  material_red.pbr_metallic_roughness.base_color_factor.a = 1.0;
  gltf.materials.push(material_red);
  
  let mut material_black = paragen::Material::new();
  material_black.name = String::from("Black");
  material_black.pbr_metallic_roughness.metallic_factor = 0.0;
  material_black.pbr_metallic_roughness.roughness_factor = 0.5;
  material_black.pbr_metallic_roughness.base_color_factor.r = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.g = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.b = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.a = 1.0;
  gltf.materials.push(material_black);
  
  let mut red_submesh = paragen::MeshPrimitive::new();
  red_submesh.attributes.position = Some(0);
  red_submesh.indices = Some(1);
  red_submesh.material = Some(0);
  
  let mut black_submesh = paragen::MeshPrimitive::new();
  black_submesh.attributes.position = Some(2);
  black_submesh.indices = Some(3);
  black_submesh.material = Some(1);
  
  let mut mesh = paragen::Mesh::new();
  mesh.name = String::from("Fortress Wall Battlement");
  mesh.primitives.push(red_submesh);
  mesh.primitives.push(black_submesh);
  gltf.meshes.push(mesh);
  
  let mut red_pos_accessor = paragen::Accessor::new();
  red_pos_accessor.buffer_view = Some(0);
  red_pos_accessor.type_ = paragen::Type::VEC3;
  red_pos_accessor.component_type = paragen::ComponentType::Float;
  red_pos_accessor.count = 8; //32;
  red_pos_accessor.max.push(1.0);
  red_pos_accessor.max.push(-0.5);
  red_pos_accessor.max.push(4.4);
  red_pos_accessor.min.push(-1.0);
  red_pos_accessor.min.push(-1.0);
  red_pos_accessor.min.push(3.8);
  gltf.accessors.push(red_pos_accessor);
  
  let mut red_pos_buffer_view = paragen::BufferView::new();
  red_pos_buffer_view.buffer = 0;
  red_pos_buffer_view.byte_length = 96; //384;
  red_pos_buffer_view.byte_offset = 0;
  red_pos_buffer_view.target = Some(paragen::Target::ArrayBuffer);
  gltf.buffer_views.push(red_pos_buffer_view);
  
  let mut red_vert_accessor = paragen::Accessor::new();
  red_vert_accessor.buffer_view = Some(1);
  red_vert_accessor.type_ = paragen::Type::SCALAR;
  red_vert_accessor.component_type = paragen::ComponentType::UnsignedShort;
  red_vert_accessor.count = 36; //54;
  gltf.accessors.push(red_vert_accessor);
  
  let mut red_vert_buffer_view = paragen::BufferView::new();
  red_vert_buffer_view.buffer = 0;
  red_vert_buffer_view.byte_length = 72; // 108;
  red_vert_buffer_view.byte_offset = 96; // 384;
  red_vert_buffer_view.target = Some(paragen::Target::ElementArrayBuffer);
  gltf.buffer_views.push(red_vert_buffer_view);
  
  let mut black_pos_accessor = paragen::Accessor::new();
  black_pos_accessor.buffer_view = Some(2);
  black_pos_accessor.type_ = paragen::Type::VEC3;
  black_pos_accessor.component_type = paragen::ComponentType::Float;
  black_pos_accessor.count = 8; //20;
  black_pos_accessor.max.push(0.5);
  black_pos_accessor.max.push(-0.5);
  black_pos_accessor.max.push(5.0);
  black_pos_accessor.min.push(-0.5);
  black_pos_accessor.min.push(-1.0);
  black_pos_accessor.min.push(4.4);
  gltf.accessors.push(black_pos_accessor);
  
  let mut black_pos_buffer_view = paragen::BufferView::new();
  black_pos_buffer_view.buffer = 0;
  black_pos_buffer_view.byte_length = 96; // 240;
  black_pos_buffer_view.byte_offset = 168; // 492;
  black_pos_buffer_view.target = Some(paragen::Target::ArrayBuffer);
  gltf.buffer_views.push(black_pos_buffer_view);
  
  let mut black_vert_accessor = paragen::Accessor::new();
  black_vert_accessor.buffer_view = Some(3);
  black_vert_accessor.type_ = paragen::Type::SCALAR;
  black_vert_accessor.component_type = paragen::ComponentType::UnsignedShort;
  black_vert_accessor.count = 30;
  gltf.accessors.push(black_vert_accessor);
  
  let mut black_vert_buffer_view = paragen::BufferView::new();
  black_vert_buffer_view.buffer = 0;
  black_vert_buffer_view.byte_length = 60;
  black_vert_buffer_view.byte_offset = 264; // 588; // 732;
  black_vert_buffer_view.target = Some(paragen::Target::ElementArrayBuffer);
  gltf.buffer_views.push(black_vert_buffer_view);
  
  let mut buffer = paragen::Buffer::new();
  buffer.byte_length = 324; // 648; // 792;
  
  let mut build_red_vertices: Vec<u8> = Vec::new();
  
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 3.8 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 3.8 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&( 1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 3.8 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&( 1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&( 1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 3.8 as f32).to_le_bytes());
  
  build_red_vertices.extend_from_slice(&( 1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_red_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  let mut build_black_vertices: Vec<u8> = Vec::new();
  
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 5.0 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 5.0 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&( 0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&( 0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 5.0 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&( 0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 4.4 as f32).to_le_bytes());
  
  build_black_vertices.extend_from_slice(&( 0.5 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&(-1.0 as f32).to_le_bytes());
  build_black_vertices.extend_from_slice(&( 5.0 as f32).to_le_bytes());
  
  let mut build_red_indices: Vec<u8> = Vec::new();
  // Top
  build_red_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  // +X side
  build_red_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  // -X side
  build_red_indices.extend_from_slice(&(0 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  // +Y side
  build_red_indices.extend_from_slice(&(0 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  // -Y side
  build_red_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  // Bottom
  build_red_indices.extend_from_slice(&(0 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_red_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  
  let mut build_black_indices: Vec<u8> = Vec::new();
  // Top
  build_black_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  // +X side
  build_black_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  // -X side
  build_black_indices.extend_from_slice(&(0 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  // +Y side
  build_black_indices.extend_from_slice(&(0 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(1 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(4 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(5 as u16).to_le_bytes());
  // -Y side
  build_black_indices.extend_from_slice(&(2 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(3 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(6 as u16).to_le_bytes());
  build_black_indices.extend_from_slice(&(7 as u16).to_le_bytes());
  
  buffer.uri = String::from("data:application/octet-stream;base64,");
  
  base64::engine::general_purpose::STANDARD.encode_string(
    build_red_vertices, &mut buffer.uri
  );
  
  base64::engine::general_purpose::STANDARD.encode_string(
    build_red_indices, &mut buffer.uri
  );
  
  base64::engine::general_purpose::STANDARD.encode_string(
    build_black_vertices, &mut buffer.uri
  );
  
  base64::engine::general_purpose::STANDARD.encode_string(
    build_black_indices, &mut buffer.uri
  );
  
  gltf.buffers.push(buffer);
  
  Ok(gltf)
}
