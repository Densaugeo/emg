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
  
  let mut material_red = emg::Material::new("Red");
  material_red.m(0.0).rh(0.5).rgba(1.0, 0.0, 0.0, 1.0);
  gltf.materials.push(material_red);
  
  let mut material_black = emg::Material::new("Black");
  material_black.m(0.0).rh(0.5).rgba(0.1, 0.1, 0.1, 1.0);
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
  
  let mut red_block = Geometry::cube();
  red_block.scale(V3::new(1.0, 0.25, 0.3)).translate(V3::new(0.0, -0.75, 4.1));
  red_block.pack(&mut gltf);
  
  let mut black_block = Geometry::cube();
  black_block.scale(V3::new(0.5, 0.25, 0.3))
    .translate(V3::new(0.0, -0.75, 4.7));
  let lower_face_tris = black_block.select_triangles(V3::new(-10.0, -10.0, 4.3),
    V3::new(10.0, 10.0, 4.5));
  black_block.delete_triangles(&lower_face_tris);
  black_block.pack(&mut gltf);
  
  Ok(gltf)
}
