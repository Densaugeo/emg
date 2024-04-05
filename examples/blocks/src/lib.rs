use emg::prelude::*;

#[emg]
fn build_the_model(_a: i32) -> Result<GLTF, ErrorCode> {
  let mut gltf = GLTF::new("A name for a scene");
  
  let node = gltf.nodes.len() as u32;
  gltf.new_root_node(0, "Fortress Wall Battlement");
  
  let mesh = gltf.meshes.len();
  gltf.new_mesh(node, "Fortress Wall Battlement");
  
  let red = gltf.materials.len() as u32;
  gltf.new_material("Red").m(0.0).rh(0.5).rgba(1.0, 0.0, 0.0, 1.0);
  
  let black = gltf.materials.len() as u32;
  gltf.new_material("Black").m(0.0).rh(0.5).rgba(0.1, 0.1, 0.1, 1.0);
  
  let mut red_block = Geometry::cube();
  red_block.s(1.0, 0.25, 0.3).t(0.0, -0.75, 4.1);
  let red_submesh = red_block.pack(&mut gltf);
  gltf.meshes[mesh].copy_primitive(red_submesh).material(red);
  
  let mut black_block = Geometry::cube();
  black_block.s(0.5, 0.25, 0.3).t(0.0, -0.75, 4.7);
  black_block.select_triangles(V3::new(-10.0, -10.0, 4.3),
    V3::new(10.0, 10.0, 4.5));
  black_block.delete_triangles();
  let black_submesh = black_block.pack(&mut gltf);
  gltf.meshes[mesh].copy_primitive(black_submesh).material(black);
  
  Ok(gltf)
}
