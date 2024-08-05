use mod3d_base::{BufferAccessor, BufferElementType, VertexAttr};

use crate::{Gl, GlProgram, GlShaderType, Mat4, UniformBuffer};

mod shader;
pub mod utils;
pub use shader::Shader;
mod program;
pub use program::Program;

mod buffer;
mod texture;

mod vao;
use vao::Vao;

//a Model3DOpenGL
//tp Model3DOpenGL
#[derive(Debug)]
pub struct Model3DOpenGL {}

//ip Default for Model3DOpenGL
impl Default for Model3DOpenGL {
    fn default() -> Self {
        Self::new()
    }
}

//ip Model3DOpenGL
impl Model3DOpenGL {
    pub fn new() -> Self {
        Self {}
    }
}

//ip Gl for Model3DOpenGL
impl Gl for Model3DOpenGL {
    type Id = u32;
    type Shader = Shader;
    type Program = Program;
    type Buffer = buffer::Buffer;
    type Vao = vao::Vao;
    type Texture = texture::Texture;

    //mp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(
        &self,
        srcs: &[&Self::Shader],
        named_attrs: &[(&str, VertexAttr)],
        named_uniforms: &[(&str, crate::UniformId)],
        named_uniform_buffers: &[(&str, usize)],
        named_textures: &[(&str, crate::TextureId, usize)],
    ) -> Result<Self::Program, String> {
        let mut program = Program::link_program(srcs)?;
        for (name, attr) in named_attrs {
            program.add_attr_name(name, *attr)?;
        }
        for (name, uniform) in named_uniforms {
            program.add_uniform_name(name, *uniform)?;
        }
        for (name, uniform) in named_uniform_buffers {
            program.add_uniform_buffer_name(name, *uniform)?;
        }
        for (name, texture_id, unit) in named_textures {
            program.add_uniform_texture_name(name, *texture_id, *unit)?;
        }
        Ok(program)
    }

    //mp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String> {
        Shader::compile(source, shader_type)
    }

    //mp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>) {
        if let Some(program) = program {
            program.set_used();
        } else {
            unsafe {
                gl::UseProgram(0);
            }
        }
    }
    //mp init_buffer_of_indices
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &BufferAccessor<Self>,
    ) {
        buffer.of_indices(view);
    }

    //mp vao_create_from_indices
    fn vao_create_from_indices(&mut self, indices: &crate::IndexBuffer<Self>) -> Result<Vao, ()> {
        Vao::create_from_indices(self, indices)
    }

    //mp buffer_bind_to_vao_attr
    fn buffer_bind_to_vao_attr(
        &mut self,
        buffer: &<Self as Gl>::Buffer,
        attr_id: &<Program as GlProgram>::GlAttrId,
        count: u32,
        ele_type: BufferElementType,
        byte_offset: u32,
        stride: u32,
    ) {
        buffer.bind_to_vao_attr(*attr_id, count, ele_type, byte_offset, stride);
    }

    //mp program_set_uniform_mat4
    fn program_set_uniform_mat4(&mut self, program: &Program, id: crate::UniformId, mat4: &Mat4) {
        if let Some(u) = program.uniform(id) {
            unsafe {
                gl::UniformMatrix4fv(u, 1, gl::FALSE, mat4.as_ptr());
            }
        }
    }

    //fp program_set_uniform_floats_4
    fn program_set_uniform_floats_4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        floats: &[f32],
    ) {
        if let Some(u) = program.uniform(id) {
            unsafe {
                gl::Uniform4fv(u, (floats.len() / 4) as i32, floats.as_ptr());
            }
        }
    }

    //mp program_bind_uniform_index
    fn program_bind_uniform_index(
        &mut self,
        program: &<Self as Gl>::Program,
        uniform_buffer_id: usize,
        gl_uindex: u32,
    ) -> Result<(), ()> {
        if let Some(u) = program.uniform(crate::UniformId::Buffer(uniform_buffer_id as u8)) {
            unsafe {
                println!(
                    "Bind program uniform buffer {} to the binding point {}",
                    u, gl_uindex
                );
                gl::UniformBlockBinding(program.id(), u as u32, gl_uindex);
            }
            utils::check_errors().expect("Bound uniform");
        }
        Ok(())
    }

    //mp program_use_texture
    /// Requires the program to be 'used'
    fn program_use_texture(
        &mut self,
        program: &<Self as Gl>::Program,
        texture_id: crate::TextureId,
        gl_texture: &<Self as Gl>::Texture,
    ) {
        if let Some((u, unit)) = program.texture_uniform(texture_id) {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + unit);
                gl::BindTexture(gl::TEXTURE_2D, gl_texture.gl_texture());
                gl::Uniform1i(u as i32, unit as i32);
            }
        }
    }

    //mp draw_primitive
    fn draw_primitive(&mut self, vaos: &[Vao], primitive: &mod3d_base::Primitive) {
        // (if p.vertices_index different to last)
        // (if p.material_index ...
        use mod3d_base::PrimitiveType::*;
        let gl_type = match primitive.primitive_type() {
            Points => gl::POINTS,
            Lines => gl::LINES,
            LineLoop => gl::LINE_LOOP,
            LineStrip => gl::LINE_STRIP,
            Triangles => gl::TRIANGLES,
            TriangleFan => gl::TRIANGLE_FAN,
            TriangleStrip => gl::TRIANGLE_STRIP,
        };

        let opt_vertices_index: Option<usize> = primitive.vertices_index().into();
        if let Some(vertices_index) = opt_vertices_index {
            let index_type = vaos[vertices_index].bind_vao();
            unsafe {
                gl::DrawElements(
                    gl_type,
                    primitive.index_count() as i32,
                    index_type,
                    primitive.byte_offset() as *const std::ffi::c_void,
                );
            }
        } else {
            unsafe {
                gl::DrawArrays(
                    gl_type,
                    primitive.byte_offset() as i32,
                    primitive.index_count() as i32,
                );
            }
        }
    }

    //mp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>) {
        if let Some(vao) = vao {
            vao.bind_vao();
        } else {
            unsafe {
                gl::BindVertexArray(0);
            }
        }
    }

    //mp uniform_buffer_create
    fn uniform_buffer_create<F: Sized>(
        &mut self,
        data: &[F],
        is_dynamic: bool,
    ) -> Result<UniformBuffer<Self>, ()> {
        let byte_length = std::mem::size_of_val(data);
        let mut gl = buffer::Buffer::default();
        gl.uniform_buffer(data, is_dynamic)?;
        Ok(UniformBuffer::new(gl, byte_length))
    }

    //mp uniform_buffer_update_data
    fn uniform_buffer_update_data<F: std::fmt::Debug>(
        &mut self,
        uniform_buffer: &UniformBuffer<Self>,
        data: &[F],
        byte_offset: u32,
    ) {
        uniform_buffer
            .gl_buffer()
            .uniform_update_data(data, byte_offset);
    }

    //mp uniform_index_of_range
    fn uniform_index_of_range(
        &mut self,
        uniform_buffer: &UniformBuffer<Self>,
        gl_uindex: u32,
        byte_offset: usize,
        byte_length: usize,
    ) {
        let (byte_offset, byte_length) = uniform_buffer.offset_and_length(byte_offset, byte_length);
        unsafe {
            gl::BindBufferRange(
                gl::UNIFORM_BUFFER,
                gl_uindex,
                uniform_buffer.gl_buffer().gl_buffer(),
                byte_offset as isize,
                byte_length as isize,
            );
        }
    }
}

//ip mod3d_base::Renderable for Model3DOpenGL
impl mod3d_base::Renderable for Model3DOpenGL {
    type Buffer = buffer::Buffer;
    type Accessor = crate::BufferView<Self>;
    type Texture = texture::Texture;
    type Material = crate::Material;
    type Vertices = crate::Vertices<Self>;

    //mp init_buffer_data_client
    /// Initialize a BufferData client
    ///
    /// This may be called multiple times for the same [BufferData]; if the
    /// gl buffer is 0 then create, else it already exists with the same data
    fn init_buffer_data_client(
        &mut self,
        client: &mut Self::Buffer,
        buffer_data: &mod3d_base::BufferData<Self>,
    ) {
        if client.is_none() {
            client.of_data(buffer_data)
        }
    }

    //mp init_buffer_view_client
    /// Initialize a buffer view client
    fn init_buffer_view_client(
        &mut self,
        client: &mut Self::Accessor,
        buffer_view: &BufferAccessor<Self>,
        attr: VertexAttr,
    ) {
        client.init_buffer_view_client(buffer_view, attr, self);
    }

    //mp create_vertices_client
    fn create_vertices_client(&mut self, vertices: &mod3d_base::Vertices<Self>) -> Self::Vertices {
        Self::Vertices::create(vertices, self)
    }

    //mp create_texture_client
    fn create_texture_client(&mut self, texture: &mod3d_base::Texture<Self>) -> Self::Texture {
        eprintln!("Create texture client");
        Self::Texture::of_texture(texture) // , self)
    }

    fn create_material_client<M>(
        &mut self,
        object: &mod3d_base::Object<M, Self>,
        material: &M,
    ) -> crate::Material
    where
        M: mod3d_base::Material,
    {
        eprintln!("Create material client");
        crate::Material::create(self, object, material).unwrap()
    }

    //mp init_material_client
    fn init_material_client<M: mod3d_base::Material>(
        &mut self,
        _client: &mut Self::Material,
        _material: &M,
    ) {
    }

    //zz All done
}
