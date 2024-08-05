//a Imports
use super::utils;
use super::Model3DOpenGL;

//a Vao
//tp Vao
/// The [Vao] *must* be owned by a [ShaderInstantiable], which borrows
/// from the Instantiable, which owns the GL buffers for the indices
/// and vertices etc
///
/// Because of this the [Vao] cannot outlive the [ShaderInstantiable], which
/// cannot outlive the GL buffer for the vertices and indices etc
pub struct Vao {
    gl_vao: u32,
    index_type: gl::types::GLuint,
}

//ip Vao
impl Vao {
    //fp bind_vao
    pub fn bind_vao(&self) -> gl::types::GLuint {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }
        self.index_type
    }

    //fp create_from_indices
    pub fn create_from_indices(
        _context: &Model3DOpenGL,
        indices: &crate::IndexBuffer<Model3DOpenGL>,
    ) -> Result<Self, ()> {
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices.gl_buffer().gl_buffer());
            println!("VAO {} {:?}", gl_vao, indices);
        }
        utils::check_errors().expect("Added indices to VAO");
        let index_type = {
            match indices.ele_type {
                mod3d_base::BufferElementType::Int16 => gl::UNSIGNED_SHORT,
                mod3d_base::BufferElementType::Int32 => gl::UNSIGNED_INT,
                _ => gl::UNSIGNED_BYTE,
            }
        };
        Ok(Self { gl_vao, index_type })
    }
}

//ip GlVao for Vao
impl crate::GlVao for Vao {}
