//a Imports
use crate::{Vector2D, Vector3D};
use mod3d_base::ByteBuffer;

use mod3d_gltf::Indexable;
use mod3d_gltf::{
    AccessorIndex, BufferIndex, ImageIndex, MaterialIndex, MeshIndex, NodeIndex, PrimitiveIndex,
    SceneIndex, TextureIndex, ViewIndex,
};
use mod3d_gltf::{Gltf, GltfBuffer, GltfMesh};

use std::ops::Range;

use index_vec::{index_vec, IndexVec};

index_vec::define_index_type! {
    pub struct VertexIndex = u32;
}

index_vec::define_index_type! {
    pub struct IndexIndex = u32;
}

impl From<u32> for VertexIndex {
    fn from(x: u32) -> Self {
        (x as usize).into()
    }
}
//a Vertex
//tp Vertex
#[derive(Debug, Default, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub position_xyz: Vector3D,
    pub normal_xyz: Vector3D,
    pub texture_uv: Vector2D,
}

//ip Vertex
impl Vertex {
    //cp new
    pub fn new(xyz: Vector3D) -> Self {
        Self {
            position_xyz: xyz,
            normal_xyz: [0., 0., 0.].into(),
            texture_uv: [0., 0.].into(),
        }
    }
    //bp with_uv
    pub fn with_uv(mut self, uv: Vector2D) -> Self {
        self.texture_uv = uv;
        self
    }
    //bp with_normal
    pub fn with_normal(mut self, xyz: Vector3D) -> Self {
        self.normal_xyz = xyz;
        self
    }
}

//a VertexArray
//tp VertexArray
#[derive(Debug, Default, Clone)]
pub struct VertexArray {
    vertices: IndexVec<VertexIndex, Vertex>,
}

//ip VertexArray
impl VertexArray {
    //ap vertices
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices.as_slice().raw
    }

    //mp add_gltf_data
    pub fn add_gltf_data(&self, gltf: &mut Gltf) -> (AccessorIndex, AccessorIndex, AccessorIndex) {
        let buffer = GltfBuffer::of_base64(self.vertices().borrow_bytes());

        let buffer = gltf.add_buffer(buffer);
        let view = gltf.add_view(
            buffer,
            0,
            self.vertices().byte_length(),
            Some(32), // 3+3+2 f32
        );

        let n = self.vertices.len() as u32;
        let position = gltf.add_accessor(view, 0, n, mod3d_base::BufferElementType::Float32, 3);
        let normal = gltf.add_accessor(view, 12, n, mod3d_base::BufferElementType::Float32, 3);
        let texture = gltf.add_accessor(view, 24, n, mod3d_base::BufferElementType::Float32, 2);
        (position, normal, texture)
    }
}

//ip Deref for VertexArray
impl std::ops::Deref for VertexArray {
    type Target = IndexVec<VertexIndex, Vertex>;
    fn deref(&self) -> &IndexVec<VertexIndex, Vertex> {
        &self.vertices
    }
}

//ip DerefMut for VertexArray
impl std::ops::DerefMut for VertexArray {
    fn deref_mut(&mut self) -> &mut IndexVec<VertexIndex, Vertex> {
        &mut self.vertices
    }
}

//ip Index<VertexIndex> for VertexArray
impl std::ops::Index<VertexIndex> for VertexArray {
    type Output = Vertex;
    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self.vertices[index]
    }
}

//ip IndexMut<VertexIndex> for VertexArray
impl std::ops::IndexMut<VertexIndex> for VertexArray {
    fn index_mut(&mut self, index: VertexIndex) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

//a Triangle
//tp Triangle
#[derive(Debug, Default, Clone)]
pub struct Triangle<T> {
    vertices: [T; 3],
}

//ip From <&(T, T, T)> for Triangle<T>
impl<T: Copy> From<&(T, T, T)> for Triangle<T> {
    fn from((a, b, c): &(T, T, T)) -> Triangle<T> {
        let vertices = [*a, *b, *c];
        Self { vertices }
    }
}

//ip From <(T, T, T)> for Triangle<T>
impl<I: Into<T>, T> From<(I, I, I)> for Triangle<T> {
    fn from((a, b, c): (I, I, I)) -> Triangle<T> {
        let vertices = [a.into(), b.into(), c.into()];
        Self { vertices }
    }
}

//ip Triangle
impl<T: Copy> Triangle<T> {
    pub fn to_tuple(&self) -> (T, T, T) {
        (self.vertices[0], self.vertices[1], self.vertices[2])
    }
}

//a IndexArray
//tp IndexArray
#[derive(Debug, Default)]
pub struct IndexArray {
    indices: IndexVec<IndexIndex, VertexIndex>,
}

//ip IndexArray
impl IndexArray {
    fn indices(&self) -> &[VertexIndex] {
        &self.indices.as_slice().raw
    }

    //mp add_gltf_buffer
    pub fn add_gltf_buffer(&self, gltf: &mut Gltf) -> ViewIndex {
        let buffer = GltfBuffer::of_base64(self.indices().borrow_bytes());
        let buffer = gltf.add_buffer(buffer);

        gltf.add_view(buffer, 0, self.indices().byte_length(), None)
    }
}

//ip Deref for IndexArray
impl std::ops::Deref for IndexArray {
    type Target = IndexVec<IndexIndex, VertexIndex>;
    fn deref(&self) -> &IndexVec<IndexIndex, VertexIndex> {
        &self.indices
    }
}

//ip DerefMut for IndexArray
impl std::ops::DerefMut for IndexArray {
    fn deref_mut(&mut self) -> &mut IndexVec<IndexIndex, VertexIndex> {
        &mut self.indices
    }
}

//ip Index<IndexIndex> for IndexArray
impl std::ops::Index<IndexIndex> for IndexArray {
    type Output = VertexIndex;
    fn index(&self, index: IndexIndex) -> &Self::Output {
        &self.indices[index]
    }
}

//ip IndexMut<IndexIndex> for IndexArray
impl std::ops::IndexMut<IndexIndex> for IndexArray {
    fn index_mut(&mut self, index: IndexIndex) -> &mut Self::Output {
        &mut self.indices[index]
    }
}

//a MeshSet
//tp MeshSet
#[derive(Debug)]
pub struct Primitive {
    // material
    indices: Range<IndexIndex>,
    primitive_type: mod3d_base::PrimitiveType,
}

//tp MeshSet
#[derive(Debug, Default)]
pub struct MeshSet {
    vertices: VertexArray,
    indices: IndexArray,
    primitives: Vec<Primitive>,
}

//ip MeshSet
impl MeshSet {
    pub fn next_vertex(&self) -> VertexIndex {
        self.vertices.len().into()
    }
    pub fn next_index(&self) -> IndexIndex {
        self.indices.len().into()
    }
    pub fn push_index(&mut self, index: VertexIndex) -> IndexIndex {
        self.indices.push(index)
    }
    pub fn push_vertex(&mut self, vertex: Vertex) -> VertexIndex {
        self.vertices.push(vertex)
    }
    pub fn add_primitive(
        &mut self,
        primitive_type: mod3d_base::PrimitiveType,
        indices: Range<IndexIndex>,
    ) -> usize {
        let n = self.primitives.len();
        let p = Primitive {
            primitive_type,
            indices,
        };
        self.primitives.push(p);
        n
    }
    pub fn add_to_gltf(&self, gltf: &mut Gltf) -> MeshIndex {
        let (pa, na, ta) = self.vertices.add_gltf_data(gltf);
        let ib = self.indices.add_gltf_buffer(gltf).into();

        let mut m = GltfMesh::default();
        for pr in &self.primitives {
            let ia = gltf.add_accessor(
                ib,
                (pr.indices.start.index() as u32) * (std::mem::size_of::<u32>() as u32),
                (pr.indices.end - pr.indices.start).index() as u32,
                mod3d_base::BufferElementType::Int32,
                1,
            );
            let p = m.add_primitive(pr.primitive_type, Some(ia), None);
            // m[p].attributes = vec![
            // (mod3d_base::VertexAttr::Position, pa),
            // (mod3d_base::VertexAttr::Normal, na),
            // (mod3d_base::VertexAttr::TexCoords0, ta),
            // ];
            m[p].add_attribute(mod3d_base::VertexAttr::Position, pa);
            m[p].add_attribute(mod3d_base::VertexAttr::Normal, na);
            m[p].add_attribute(mod3d_base::VertexAttr::TexCoords0, ta);
        }
        gltf.add_mesh(m)
    }
}
