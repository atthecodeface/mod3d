//a Imports
use crate::console_log;
use crate::webgl_log::log_gl_vao;
use crate::{Gl, GlProgram, GlShaderType, Mat4, UniformBuffer};
use web_sys::WebGl2RenderingContext;

mod shader;
pub use shader::Shader;
mod program;
pub use program::Program;

mod buffer;

mod texture;

mod vao;
use vao::Vao;

//a Model3DWebGL
//tp Model3DWebGL
#[derive(Debug)]
pub struct Model3DWebGL {
    context: WebGl2RenderingContext,
}

//ip Model3DWebGL
impl Model3DWebGL {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        Self { context }
    }
    pub fn context(&self) -> &WebGl2RenderingContext {
        &self.context
    }
}

//ip Deref for Model3DWebGL
impl std::ops::Deref for Model3DWebGL {
    type Target = WebGl2RenderingContext;
    fn deref(&self) -> &WebGl2RenderingContext {
        &self.context
    }
}

//ip Gl for Model3DWebGL
impl Gl for Model3DWebGL {
    type Id = u32;
    type Shader = Shader;
    type Program = Program;
    type Buffer = buffer::Buffer;
    type Vao = vao::Vao;
    type Texture = texture::Texture;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(
        &self,
        srcs: &[&Self::Shader],
        named_attrs: &[(&str, mod3d_base::VertexAttr)],
        named_uniforms: &[(&str, crate::UniformId)],
        named_uniform_buffers: &[(&str, usize)],
        named_textures: &[(&str, crate::TextureId, usize)],
    ) -> Result<Self::Program, String> {
        let mut program = Program::link_program(&self.context, srcs)?;
        for (name, attr) in named_attrs {
            program.add_attr_name(self, name, *attr)?;
        }
        for (name, uniform) in named_uniforms {
            program.add_uniform_name(self, name, *uniform)?;
        }
        for (name, uniform) in named_uniform_buffers {
            program.add_uniform_buffer_name(self, name, *uniform)?;
        }
        for (name, texture_id, unit) in named_textures {
            program.add_uniform_texture_name(self, name, *texture_id, *unit)?;
        }
        Ok(program)
    }

    //fp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String> {
        Shader::compile(&self.context, source, shader_type)
    }

    //fp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>) {
        if let Some(program) = program {
            program.set_used(&self.context);
        } else {
            self.context.use_program(None);
        }
    }

    //mp init_buffer_of_indices
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &mod3d_base::BufferAccessor<Self>,
    ) {
        buffer.of_indices(view, self);
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
        ele_type: mod3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    ) {
        buffer.bind_to_vao_attr(self, *attr_id, count, ele_type, byte_offset, stride);
    }

    //mp program_set_uniform_mat4
    fn program_set_uniform_mat4(&mut self, program: &Program, id: crate::UniformId, mat4: &Mat4) {
        console_log!("program_set_uniform_mat4: {:?} {:?}", id, mat4);
        if let Some(u) = program.uniform(id) {
            self.context
                .uniform_matrix4fv_with_f32_array(Some(u), false, mat4);
        }
    }

    //fp program_set_uniform_floats_4
    fn program_set_uniform_floats_4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        floats: &[f32],
    ) {
        console_log!("webgl: set uniform [vec4] {id:?} {floats:?}");
        if let Some(u) = program.uniform(id) {
            self.context.uniform4fv_with_f32_array(Some(u), floats);
        }
    }

    //mp program_bind_uniform_index
    fn program_bind_uniform_index(
        &mut self,
        program: &<Self as Gl>::Program,
        uniform_buffer_id: usize,
        gl_uindex: u32,
    ) -> Result<(), ()> {
        if let Some(u) = program.uniform_buffer(uniform_buffer_id) {
            self.context
                .uniform_block_binding(program.program(), u, gl_uindex);
            Ok(())
        } else {
            Err(())
        }
    }

    //mp program_use_texture
    /// Requires the program to be 'used'
    fn program_use_texture(
        &mut self,
        program: &<Self as Gl>::Program,
        texture_id: crate::TextureId,
        gl_texture: &<Self as Gl>::Texture,
    ) {
        console_log!("webgl: set texture {texture_id:?}");
        if let Some((u, unit)) = program.texture_uniform(texture_id) {
            self.context
                .active_texture(WebGl2RenderingContext::TEXTURE0 + unit);
            self.context
                .bind_texture(WebGl2RenderingContext::TEXTURE_2D, gl_texture.gl_texture());
            self.context.uniform1i(Some(u), unit as i32);
        }
    }

    //mp draw_primitive
    fn draw_primitive(&mut self, vaos: &[Vao], primitive: &mod3d_base::Primitive) {
        console_log!("webgl: draw_primitive {primitive:?}");
        use mod3d_base::PrimitiveType::*;
        let gl_type = match primitive.primitive_type() {
            Points => WebGl2RenderingContext::POINTS,
            Lines => WebGl2RenderingContext::LINES,
            LineLoop => WebGl2RenderingContext::LINE_LOOP,
            LineStrip => WebGl2RenderingContext::LINE_STRIP,
            Triangles => WebGl2RenderingContext::TRIANGLES,
            TriangleFan => WebGl2RenderingContext::TRIANGLE_FAN,
            TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP,
        };
        let opt_vertices_index: Option<usize> = primitive.vertices_index().into();
        if let Some(vertices_index) = opt_vertices_index {
            let index_type = vaos[vertices_index].bind_vao(self);
            self.draw_elements_with_i32(
                gl_type,
                primitive.index_count() as i32,
                index_type,
                primitive.byte_offset() as i32,
            );
        } else {
            self.draw_arrays(
                gl_type,
                primitive.byte_offset() as i32,
                primitive.index_count() as i32,
            );
        }
    }

    //mp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>) {
        if let Some(vao) = vao {
            vao.bind_vao(self);
        } else {
            self.bind_vertex_array(None);
            log_gl_vao(self, None, "bind_vao");
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
        gl.uniform_buffer(self, data, is_dynamic)?;
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
            .uniform_update_data(self, data, byte_offset);
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
        uniform_buffer.gl_buffer().bind_buffer_range(
            self,
            WebGl2RenderingContext::UNIFORM_BUFFER,
            gl_uindex,
            byte_offset as i32,
            byte_length as i32,
        );
    }
}

//ip mod3d_base::Renderable for Model3DWebGL
impl mod3d_base::Renderable for Model3DWebGL {
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
            client.of_data(buffer_data, self)
        }
    }

    //mp init_buffer_view_client
    /// Initialize a buffer view client
    fn init_buffer_view_client(
        &mut self,
        client: &mut Self::Accessor,
        buffer_view: &mod3d_base::BufferAccessor<Self>,
        attr: mod3d_base::VertexAttr,
    ) {
        client.init_buffer_view_client(buffer_view, attr, self);
    }

    //mp create_vertices_client
    fn create_vertices_client(&mut self, vertices: &mod3d_base::Vertices<Self>) -> Self::Vertices {
        Self::Vertices::create(vertices, self)
    }

    //mp create_texture_client
    fn create_texture_client(&mut self, texture: &mod3d_base::Texture<Self>) -> Self::Texture {
        Self::Texture::of_texture(texture, self) // , self)
    }

    //mp init_material_client
    fn init_material_client<M: mod3d_base::Material>(
        &mut self,
        _client: &mut Self::Material,
        _material: &M,
    ) {
    }

    fn create_material_client<M>(
        &mut self,
        object: &mod3d_base::Object<M, Self>,
        material: &M,
    ) -> crate::Material
    where
        M: mod3d_base::Material,
    {
        crate::Material::create(self, object, material).unwrap()
    }

    //zz All done
}
