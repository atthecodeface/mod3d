use geo_nd::Vector;

use crate::{MeshSet, Triangle, Vector2D, Vector3D, Vertex, VertexIndex};

/// One of the 12 icosahedron vertices
///
/// The vertices are the four corners of three rectangles with the
/// golden ratio as an aspect ratio. Each rectangle is centred on the
/// origin, and is in the plane of one of the three axes
///
/// The first rectangle is vertices 0-3, which are the top-left,
/// top-right, bottom-left, bottom-right, respectively
pub struct IcosVertex {
    xyz: Vector3D,
}
impl IcosVertex {
    fn new(n: u8) -> Self {
        let phi: f32 = (1.0 + (5.0_f32).sqrt()) / 2.0;
        let vx = {
            if (n & 1) == 0 {
                -1.
            } else {
                1.
            }
        };
        let vy = {
            if (n & 2) == 0 {
                phi
            } else {
                -phi
            }
        };
        let xyz: Vector3D = {
            match n / 4 {
                0 => [vx, vy, 0.],
                1 => [0., vx, vy],
                _ => [vy, 0., vx],
            }
        }
        .into();
        // let xyz = xyz.normalize();
        Self { xyz }
    }
}
//fp texture_uv_of_u8
/// Create a texture uv from an index
pub fn texture_uv_of_u8(uv: u8) -> Vector2D {
    let u = uv % 6;
    let v = uv / 6;
    let u = (u as f32) * 0.2;
    let v = (v as f32) * 0.5;
    [u, v].into()
}

/// There are 12 vertices on an icosahedron, and 20 faces which we
/// split into two strips of ten triangles.
///
/// There are 18 texture coordinates for the two strips; each strip
/// has 12, with 6 shared between them. This actually uses a texture
/// vertex of 0 to 11.
#[derive(Debug, Clone, Copy, Default)]
pub struct IcoStripVertex {
    ico: u8,
    texture: u8,
}

/// The strip (10 triangles) is *not* a triangle strip in the OpenGL
/// sense - so 3 vertices per triangle and 30 vertices
/// for the whole strip
///
/// However, to provide a safer design, we create here the 12 vertices
/// (10 different XYZ, 12 different texture) for each ico strip
#[derive(Debug, Default)]
pub struct IcoStripVertices {
    strip: [IcoStripVertex; 12],
}
impl IcoStripVertex {
    //mp to_vertex
    /// Create a new Vertex for a scaled icosphere centred at the
    /// origin, from a icosphere index and a texture index
    pub fn to_vertex(&self) -> Vertex {
        let icos_vertex = IcosVertex::new(self.ico);
        let texture_uv = texture_uv_of_u8(self.texture);
        Vertex::new(icos_vertex.xyz)
            .with_normal(icos_vertex.xyz)
            .with_uv(texture_uv)
    }
}

/// The actual indices into ICOSTRIP_VERTICES_0/1 of the vertices of the
/// triangles for strip 0
const ICOSTRIP: [(u8, u8, u8); 10] = [
    (0, 2, 1),
    (1, 2, 3),
    (3, 2, 4),
    (4, 2, 5),
    (4, 5, 6),
    (5, 7, 6),
    (8, 6, 7),
    (7, 9, 8),
    (10, 8, 9),
    (11, 8, 10),
];

/// The mapping from icosahedron vertices to texture map vertices
const ICOSTRIP_VERTICES_0: IcoStripVertices = IcoStripVertices {
    strip: [
        IcoStripVertex { ico: 0, texture: 0 },
        IcoStripVertex { ico: 1, texture: 6 },
        IcoStripVertex { ico: 5, texture: 1 },
        IcoStripVertex { ico: 9, texture: 7 },
        IcoStripVertex { ico: 4, texture: 8 },
        IcoStripVertex {
            ico: 11,
            texture: 2,
        },
        IcoStripVertex { ico: 2, texture: 9 },
        IcoStripVertex {
            ico: 10,
            texture: 3,
        },
        IcoStripVertex {
            ico: 6,
            texture: 10,
        },
        IcoStripVertex { ico: 7, texture: 4 },
        IcoStripVertex { ico: 8, texture: 5 },
        IcoStripVertex {
            ico: 3,
            texture: 11,
        },
    ],
};

/// The mapping from icosahedron vertices to texture map vertices
const ICOSTRIP_VERTICES_1: IcoStripVertices = IcoStripVertices {
    strip: [
        IcoStripVertex { ico: 5, texture: 6 },
        IcoStripVertex {
            ico: 11,
            texture: 12,
        },
        IcoStripVertex { ico: 0, texture: 7 },
        IcoStripVertex {
            ico: 10,
            texture: 13,
        },
        IcoStripVertex {
            ico: 7,
            texture: 14,
        },
        IcoStripVertex { ico: 1, texture: 8 },
        IcoStripVertex {
            ico: 8,
            texture: 15,
        },
        IcoStripVertex { ico: 9, texture: 9 },
        IcoStripVertex {
            ico: 3,
            texture: 16,
        },
        IcoStripVertex {
            ico: 4,
            texture: 10,
        },
        IcoStripVertex {
            ico: 2,
            texture: 11,
        },
        IcoStripVertex {
            ico: 6,
            texture: 17,
        },
    ],
};

//tp IcoStrip
/// A representation of one of the two strips that makes up an icosphere
#[derive(Debug, Default)]
pub struct IcoStrip {
    #[allow(dead_code)]
    pub strip: [IcoStripVertex; 30],
}

#[derive(Debug, Default, Clone)]
pub struct SubdivVertex {
    division: u32,
    ico_face: Triangle<u8>,
    weights: Triangle<u32>,
}
impl SubdivVertex {
    pub fn new(division: u32, ico_face: Triangle<u8>, weights: Triangle<u32>) -> Self {
        Self {
            division,
            ico_face,
            weights,
        }
    }
    pub fn to_vertex(&self, ico_vertices: &[Vertex], scale: f32) -> Vertex {
        let (a, b, c) = self.ico_face.to_tuple();
        let iv_a = &ico_vertices[a as usize];
        let iv_b = &ico_vertices[b as usize];
        let iv_c = &ico_vertices[c as usize];
        let division = self.division as f32;
        let (wa, wb, wc) = self.weights.to_tuple();
        let mut xyz = iv_a.position_xyz * (wa as f32);
        xyz += iv_b.position_xyz * (wb as f32);
        xyz += iv_c.position_xyz * (wc as f32);
        let xyz = xyz / division;

        let mut uv = iv_a.texture_uv * (wa as f32);
        uv += iv_b.texture_uv * (wb as f32);
        uv += iv_c.texture_uv * (wc as f32);
        let uv = uv / division;

        let normal_xyz = xyz.normalize();
        let position_xyz = normal_xyz * scale;
        Vertex::new(position_xyz)
            .with_normal(normal_xyz)
            .with_uv(uv)
    }
}

#[derive(Debug)]
pub struct Icosphere {
    /// Size of the icosphere
    scale: f32,
    /// The 12 vertices times 2 (as texture mapping is different for
    /// the two strips)
    ico_vertices: Vec<Vertex>,
    /// The 20 faces from the two strips, as indicies into ico_vertices
    ico_faces: Vec<Triangle<u8>>,
    /// The vertices required as subdivisions of the ico_vertices as triangles
    subdiv_vertices: Vec<SubdivVertex>,
    /// Triangles of indices of subdiv_vertices
    subdiv_faces: Vec<Triangle<VertexIndex>>,
}

impl Icosphere {
    pub fn new(scale: f32) -> Self {
        let mut ico_vertices = vec![];
        let mut ico_faces = vec![];
        for x in ICOSTRIP_VERTICES_0.strip.iter() {
            ico_vertices.push(x.to_vertex());
        }
        for x in ICOSTRIP_VERTICES_1.strip.iter() {
            ico_vertices.push(x.to_vertex());
        }
        for (v0, v1, v2) in ICOSTRIP.iter() {
            let tri: Triangle<u8> = (*v0, *v1, *v2).into();
            ico_faces.push(tri);
        }
        for (v0, v1, v2) in ICOSTRIP.iter() {
            let tri: Triangle<u8> = (*v0 + 12, *v1 + 12, *v2 + 12).into();
            ico_faces.push(tri);
        }

        let subdiv_vertices = vec![];
        let subdiv_faces = Default::default();
        Self {
            scale,
            ico_faces,
            ico_vertices,
            subdiv_vertices,
            subdiv_faces,
        }
    }

    //mp add_subdivided_face
    pub fn add_subdivided_face(&mut self, face: usize, division: u32) {
        let face_tri = &self.ico_faces[face];
        let first_vertex = self.subdiv_vertices.len() as u32;
        for y in 0..division + 1 {
            for x in 0..division + 1 {
                if x + y > division {
                    continue;
                }
                let z = division - x - y;
                self.subdiv_vertices.push(SubdivVertex::new(
                    division,
                    face_tri.clone(),
                    (x, y, z).into(),
                ));
            }
        }
        let _n_vertices = self.subdiv_vertices.len() as u32;

        // Want to add triangles (0, 1, d+1), (1, 2, d+2), .., (d-1, d, d+d), (d+1, d+2, d+2+d-1), ..
        //
        // Particularly for d = 4 we want to add triangle (n, n+1, n
        //
        // We maintain y_i which is the index of the first vertex on
        // the current y, and ny_i which is the index of the first
        // vertex on the next y
        let mut y_i = first_vertex;
        for y in 0..division {
            let ny_i = y_i + 1 + division - y;
            for x in 0..division {
                if x + y + 1 > division {
                    continue;
                }
                let face = (y_i + x, y_i + x + 1, ny_i + x).into();
                self.subdiv_faces.push(face);
                if x + y + 2 > division {
                    continue;
                }
                let face = (y_i + x + 1, ny_i + x + 1, ny_i + x).into();
                self.subdiv_faces.push(face);
            }
            y_i = ny_i;
        }
    }

    //mp add_to_mesh_set
    pub fn add_to_mesh_set(&self, mesh_set: &mut MeshSet) -> usize {
        let first_vertex: usize = mesh_set.next_vertex().into();
        let first_index = mesh_set.next_index();
        for c in &self.subdiv_vertices {
            let cv = c.to_vertex(&self.ico_vertices, self.scale);
            mesh_set.push_vertex(cv);
        }
        for f in &self.subdiv_faces {
            let (a, b, c) = f.to_tuple();
            mesh_set.push_index(a + first_vertex);
            mesh_set.push_index(b + first_vertex);
            mesh_set.push_index(c + first_vertex);
        }
        let next_index = mesh_set.next_index();
        mesh_set.add_primitive(
            mod3d_base::PrimitiveType::Triangles,
            first_index..next_index,
        )
    }
}
