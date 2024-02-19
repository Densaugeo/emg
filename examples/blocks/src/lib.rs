use emg::prelude::*;

#[emg]
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
  
  let mut material_red = emg::Material::new();
  material_red.name = String::from("Red");
  material_red.pbr_metallic_roughness.metallic_factor = 0.0;
  material_red.pbr_metallic_roughness.roughness_factor = 0.5;
  material_red.pbr_metallic_roughness.base_color_factor.r = 1.0;
  material_red.pbr_metallic_roughness.base_color_factor.g = 0.0;
  material_red.pbr_metallic_roughness.base_color_factor.b = 0.0;
  material_red.pbr_metallic_roughness.base_color_factor.a = 1.0;
  gltf.materials.push(material_red);
  
  let mut material_black = emg::Material::new();
  material_black.name = String::from("Black");
  material_black.pbr_metallic_roughness.metallic_factor = 0.0;
  material_black.pbr_metallic_roughness.roughness_factor = 0.5;
  material_black.pbr_metallic_roughness.base_color_factor.r = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.g = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.b = 0.1;
  material_black.pbr_metallic_roughness.base_color_factor.a = 1.0;
  gltf.materials.push(material_black);
  
  let mut red_submesh = emg::MeshPrimitive::new();
  red_submesh.attributes.position = Some(0);
  red_submesh.indices = Some(1);
  red_submesh.material = Some(0);
  
  let mut black_submesh = emg::MeshPrimitive::new();
  black_submesh.attributes.position = Some(2);
  black_submesh.indices = Some(3);
  black_submesh.material = Some(1);
  
  let mut mesh = emg::Mesh::new();
  mesh.name = String::from("Fortress Wall Battlement");
  mesh.primitives.push(red_submesh);
  mesh.primitives.push(black_submesh);
  gltf.meshes.push(mesh);
  
  let mut buffer = emg::Buffer::new();
  buffer.byte_length = 324;
  gltf.buffers.push(buffer);
  
  let build_red_vertices: Vec<[f32; 3]> = vec![
    [-1.0, -0.5,  3.8],
    [-1.0, -0.5,  4.4],
    
    [-1.0, -1.0,  3.8],
    [-1.0, -1.0,  4.4],
    
    [ 1.0, -0.5,  3.8],
    [ 1.0, -0.5,  4.4],
    
    [ 1.0, -1.0,  3.8],
    [ 1.0, -1.0,  4.4],
  ];
  let (accessor_index, buffer_view_index) = gltf.append_to_glb_bin(
    build_red_vertices);
  gltf.accessors[accessor_index as usize].min.extend_from_slice(
    &[-1.0, -1.0, 3.8]);
  gltf.accessors[accessor_index as usize].max.extend_from_slice(
    &[ 1.0, -0.5, 4.4]);
  gltf.buffer_views[buffer_view_index as usize].target = Some(
    emg::Target::ArrayBuffer);
  
  let build_red_indices: Vec<u16> = vec![
    // Top
    1, 3, 5,
    3, 7, 5,
    
    // +X side
    4, 5, 6,
    5, 7, 6,
    
    // -X side
    0, 2, 1,
    1, 2, 3,
    
    // +Y side
    0, 1, 4,
    1, 5, 4,
    
    // -Y side
    2, 6, 3,
    3, 6, 7,
    
    // Bottom
    0, 4, 2,
    2, 4, 6,
  ];
  let (accessor_index, buffer_view_index) = gltf.append_to_glb_bin(
    build_red_indices);
  gltf.buffer_views[buffer_view_index as usize].target = Some(
    emg::Target::ElementArrayBuffer);
  
  let build_black_vertices: Vec<[f32; 3]> = vec![
    [-0.5, -0.5,  4.4],
    [-0.5, -0.5,  5.0],
    
    [-0.5, -1.0,  4.4],
    [-0.5, -1.0,  5.0],
    
    [ 0.5, -0.5,  4.4],
    [ 0.5, -0.5,  5.0],
    
    [ 0.5, -1.0,  4.4],
    [ 0.5, -1.0,  5.0],
  ];
  let (accessor_index, buffer_view_index) = gltf.append_to_glb_bin(
    build_black_vertices);
  gltf.accessors[accessor_index as usize].min.extend_from_slice(
    &[-0.5, -1.0, 4.4]);
  gltf.accessors[accessor_index as usize].max.extend_from_slice(
    &[ 0.5, -0.5, 5.0]);
  gltf.buffer_views[buffer_view_index as usize].target = Some(
    emg::Target::ArrayBuffer);
  
  let build_black_indices: Vec<u16> = vec![
    // Top
    1, 3, 5,
    3, 7, 5,
    
    // +X side
    4, 5, 6,
    5, 7, 6,
    
    // -X side
    0, 2, 1,
    1, 2, 3,
    
    // +Y side
    0, 1, 4,
    1, 5, 4,
    
    // -Y side
    2, 6, 3,
    3, 6, 7,
  ];
  let (accessor_index, buffer_view_index) = gltf.append_to_glb_bin(
    build_black_indices);
  gltf.buffer_views[buffer_view_index as usize].target = Some(
    emg::Target::ElementArrayBuffer);
  
  Ok(gltf)
}
