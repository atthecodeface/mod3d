//a Imports
use crate::{BufferView, UniformBuffer};
use crate::{Mat4, TextureId, UniformId};
use crate::{Material, Vertices};

//tp GlShader
pub trait GlShader: Sized {
    type Id<'a>: Sized + 'a
    where
        Self: 'a;
    //fp id
    /// Get the shader program id
    fn id(&self) -> Self::Id<'_>;
}

//tt GlProgram
pub trait GlProgram: Sized {
    type GlAttrId: Sized;
    // type Context;
    type GlUniformId<'a>: Sized + 'a
    where
        Self: 'a;
    /// Borrow a slice of attribute / program attribute location pairings
    fn attributes(&self) -> &[(Self::GlAttrId, mod3d_base::VertexAttr)];

    /// Attempt to retrieve a uniform from a [UniformId] - return None
    /// if the shader program does not have that uniform
    fn uniform(&self, uniform_id: UniformId) -> Option<Self::GlUniformId<'_>>;

    /// Attempt to retrieve the uniform and sampler from a [UniformId] - return None
    /// if the shader program does not have that uniform
    fn texture_uniform(&self, texture_id: TextureId) -> Option<(Self::GlUniformId<'_>, u32)>;
}

//tt GlShaderType
pub enum GlShaderType {
    Vertex,
    Fragment,
}

//tt GlBuffer
/// The GlBuffer is something that is the Gl context's static draw
/// copy of a [u8] that forms the values for vertices and indices etc.
///
/// A single GlBuffer will be cloned for different
/// mod3d_base::BufferAccessor of the same BufferData (by the
/// [VertexBuffer] type)
pub trait GlBuffer: Default + Clone + std::fmt::Debug + mod3d_base::BufferClient {}

//tt GlVao
/// The GlVao correlates to an OpenGl VAO buffer for a ShaderInstantiable mesh + GlProgram
pub trait GlVao: Sized {}

//tt Gl
/// This must provide Debug as Rust requires a type that is generic on
/// a type of trait [Gl] to have that generic support Debug in order
/// to derive Debug on the type.
///
/// The same is true of Clone, but that is too burdensome for Gl
pub trait Gl:
    std::fmt::Debug
    + mod3d_base::Renderable<
        Buffer = <Self as Gl>::Buffer,
        Vertices = Vertices<Self>,
        Texture = <Self as Gl>::Texture,
        Material = Material,  // <Self>,
        Accessor = BufferView<Self>,
    > + std::fmt::Debug
{
    // Lose Id?
    type Id: Sized;
    type Shader: GlShader;
    type Program: GlProgram;
    type Buffer: GlBuffer;
    type Vao: GlVao;
    type Texture;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(
        &self,
        srcs: &[&Self::Shader],
        named_attrs: &[(&str, mod3d_base::VertexAttr)],
        named_uniforms: &[(&str, UniformId)],
        named_uniform_buffers: &[(&str, usize)],
        named_textures: &[(&str, crate::TextureId, usize)],
    ) -> Result<Self::Program, String>;

    //fp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String>;

    //fp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>);

    //mp init_buffer_of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &mod3d_base::BufferAccessor<Self>,
    );

    //mp uniform_buffer_create
    /// Create a uniform buffer (a GlBuffer in the GPU bound to GlUniformBuffer)
    ///
    /// Fill the data; if is_dynamic is true then make it dynamic draw
    fn uniform_buffer_create<F: Sized>(
        &mut self,
        _data: &[F],
        _is_dynamic: bool,
    ) -> Result<UniformBuffer<Self>, ()>;

    //mp uniform_buffer_update_data
    /// Update (a portion) of a uniform GlBuffer
    fn uniform_buffer_update_data<F: std::fmt::Debug>(
        &mut self,
        _uniform_buffer: &UniformBuffer<Self>,
        _data: &[F],
        _byte_offset: u32,
    );

    //mp uniform_index_of_range
    /// Set the GPU's UniformBlockMatrix index N to a range of a UniformBuffer
    fn uniform_index_of_range(
        &mut self,
        _uniform_buffer: &UniformBuffer<Self>,
        _gl_uindex: u32,
        _byte_offset: usize,
        _byte_length: usize,
    );

    //fp vao_create_from_indices
    /// Create a VAO, add the indices as its element array buffer, and
    /// leave it bound
    fn vao_create_from_indices(
        &mut self,
        indices: &crate::IndexBuffer<Self>,
    ) -> Result<Self::Vao, ()>;

    //fp buffer_bind_to_vao_attr
    /// With the currently bound VAO add this view of the specified
    /// buffer as an attribute of the program, if the program has that
    /// attribute
    fn buffer_bind_to_vao_attr(
        &mut self,
        buffer: &<Self as Gl>::Buffer,
        attr_id: &<<Self as Gl>::Program as GlProgram>::GlAttrId,
        count: u32,
        ele_type: mod3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    );

    //mp program_set_uniform_mat4
    fn program_set_uniform_mat4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        mat4: &Mat4,
    );

    //mp program_set_uniform_floats_4
    fn program_set_uniform_floats_4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        floats: &[f32],
    );

    //mp program_bind_uniform_index
    fn program_bind_uniform_index(
        &mut self,
        program: &<Self as Gl>::Program,
        uniform_buffer_id: usize,
        gl_uindex: u32,
    ) -> Result<(), ()>;

    //mp program_use_texture
    /// Activate the required texture unit and set the program's
    /// uniform to that unit, and bind the Gl texture to the unit
    ///
    /// The texture unit and uniform are specified by the program, and
    /// can be gathered from program and texture_id
    fn program_use_texture(
        &mut self,
        program: &<Self as Gl>::Program,
        texture_id: crate::TextureId,
        gl_texture: &<Self as Gl>::Texture,
    );

    //fp draw_primitive
    /// Draw the specified primitive using its VAO index into the vaos slice
    fn draw_primitive(&mut self, vaos: &[Self::Vao], primitive: &mod3d_base::Primitive);

    //fp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>);
}
