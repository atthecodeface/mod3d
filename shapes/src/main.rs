mod icosphere;
mod mesh_set;
mod rectangle;

mod types;

pub type Vector2D = geo_nd::FArray<f32, 2>;
pub type Vector3D = geo_nd::FArray<f32, 3>;
pub type Matrix3D = geo_nd::FArray2<f32, 3, 9>;
use geo_nd::SqMatrix;
pub use mod3d_gltf::{
    AccessorIndex, BufferIndex, ImageIndex, Indexable, MaterialIndex, MeshIndex, NodeIndex,
    PrimitiveIndex, SceneIndex, TextureIndex, ViewIndex,
};

use icosphere::Icosphere;
use mesh_set::{MeshSet, Triangle, Vertex, VertexIndex};
use mod3d_gltf::{Gltf, GltfAsset, GltfNode, GltfScene};
use rectangle::Rectangle;

//a Main
fn main() {
    let mut icosphere = Icosphere::new(1.0);
    for i in 0..20 {
        // icosphere.add_subdivided_face(i, (2 + i / 2) as u32);
        icosphere.add_subdivided_face(i, 1);
    }

    let mut gltf = Gltf::default();

    gltf.set_asset(GltfAsset::new("Copyright".to_string()));
    let mut mesh_set = MeshSet::default();
    let _p = icosphere.add_to_mesh_set(&mut mesh_set);

    let mut wall: Rectangle<bool> = Rectangle::new(900, 230, false);
    for (ri, _, _, _, _) in wall.split_region_iter(100..190, 130..170) {
        wall[ri] = true;
    }
    for (ri, _, _, _, _) in wall.split_region_iter(400..530, 0..190) {
        wall[ri] = true;
    }
    let mut nv = mesh_set.next_vertex();
    let first_i = mesh_set.next_index();
    // let m: Matrix3D = [1., 0., 0., 0., 1., 0., 0., 0., 1.].into();
    let m: Matrix3D = [0., 1., 0., -1., 0., 0., 0., 0., 1.].into();
    let m = m * 0.01;
    let t: Vector3D = [0., 0., 0.].into();
    for (r, _, _, _, _) in wall.iter() {
        if !wall[r] {
            let (v0, v1, v2, v3) = wall.map_region(r, |x, y| {
                let pt: Vector3D = [x as f32, y as f32, 0.].into();
                Vertex::new(m.transform(&pt) + t)
            });
            mesh_set.push_vertex(v0);
            mesh_set.push_vertex(v1);
            mesh_set.push_vertex(v2);
            mesh_set.push_vertex(v3);
            mesh_set.push_index(nv);
            mesh_set.push_index(nv + 1);
            mesh_set.push_index(nv + 2);
            mesh_set.push_index(nv);
            mesh_set.push_index(nv + 2);
            mesh_set.push_index(nv + 3);
            nv += 4;
        }
    }
    let last_i = mesh_set.next_index();
    mesh_set.add_primitive(mod3d_base::PrimitiveType::Triangles, first_i..last_i);

    let mut n = GltfNode::default();
    let m = mesh_set.add_to_gltf(&mut gltf);
    n.set_mesh(m);
    n.trans_mut().rotate_axis_angle(&[1., 0., 0.], 30.);
    n.trans_mut().translate(&[1., 0., 0.], 3.);
    n.derive_gltf();
    let n0 = gltf.add_node(n);

    let mut n = GltfNode::default();
    let m = mesh_set.add_to_gltf(&mut gltf);
    n.set_mesh(m);
    n.derive_gltf();
    let n1 = gltf.add_node(n);

    let mut s = GltfScene::default();
    s.add_node(n0);
    s.add_node(n1);
    gltf.add_scene(s);
    let _j = serde_json::to_string_pretty(&gltf).unwrap();
    println!("{_j}");
}
