use mod3d_base::{BufferAccessor, BufferElementType, VertexAttr};

use mod3d_gl::{Gl, GlProgram, GlShaderType, Mat4, UniformBuffer};

mod buffer;

// mod shader;
// pub mod utils;
// pub use shader::Shader;
// mod program;
// pub use program::Program;

// mod texture;

// mod vao;
// use vao::Vao;
use crate::utils::rtc::run_to_completion as rtc;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to create surface")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("failed to request device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("failed to find an appropriate GPU adapter")]
    NoAdapter,
}

//a Model3DWGpu
//tp Model3DWGpu
#[derive(Debug)]
pub struct Model3DWGpu<'tgt> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'tgt>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

//ip Model3DWGpu
impl<'tgt> Model3DWGpu<'tgt> {
    pub fn new<I>(target: I) -> Result<Self, Error>
    where
        I: Into<wgpu::SurfaceTarget<'tgt>>,
    {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(target)?;
        let adapter_request = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        });
        let adapter = rtc(adapter_request).ok_or(Error::NoAdapter)?;

        // Create the logical device and command queue
        let device_request = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        );
        let (device, queue) = rtc(device_request)?;
        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
        })
    }
}

/*
//ip Gl for Model3DWGpu
impl Gl for Model3DWGpu {
    // type Program = Program;
    // type Buffer = buffer::Buffer;
    // type Vao = vao::Vao;
    // type Texture = texture::Texture;
    type Program = ();
    type Buffer = ();
    type Vao = ();
    type Texture = ();
    type PipelineDesc<'a> = ();

    fn create_pipeline<F: Fn(&str) -> Result<String, String>>(
        &mut self,
        read_src: &F,
        pipeline_desc: Box<Self::PipelineDesc<'_>>,
    ) -> Result<Self::Program, String> {
        Err("NYI".to_string())
    }

    //mp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>) {
        if let Some(program) = program {
        } else {
        }
    }

    //mp init_buffer_of_indices
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &BufferAccessor<Self>,
    ) {
    }

    //mp vao_create_from_indices
    fn vao_create_from_indices(
        &mut self,
        indices: &mod3d_gl::IndexBuffer<Self>,
    ) -> Result<Self::Vao, ()> {
        Err("NYI".to_string())
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
    }

    //mp program_set_uniform_mat4
    fn program_set_uniform_mat4(
        &mut self,
        program: &Program,
        id: mod3d_gl::UniformId,
        mat4: &Mat4,
    ) {
    }

    //fp program_set_uniform_floats_4
    fn program_set_uniform_floats_4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        floats: &[f32],
    ) {
    }

    //mp program_bind_uniform_index
    fn program_bind_uniform_index(
        &mut self,
        program: &<Self as Gl>::Program,
        uniform_buffer_id: usize,
        gl_uindex: u32,
    ) -> Result<(), ()> {
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
    }

    //mp draw_primitive
    fn draw_primitive(&mut self, vaos: &[Vao], primitive: &mod3d_base::Primitive) {}

    //mp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>) {}

    //mp uniform_buffer_create
    fn uniform_buffer_create<F: Sized>(
        &mut self,
        data: &[F],
        is_dynamic: bool,
    ) -> Result<UniformBuffer<Self>, ()> {
        Err(())
    }

    //mp uniform_buffer_update_data
    fn uniform_buffer_update_data<F: std::fmt::Debug>(
        &mut self,
        uniform_buffer: &UniformBuffer<Self>,
        data: &[F],
        byte_offset: u32,
    ) {
    }

    //mp uniform_index_of_range
    fn uniform_index_of_range(
        &mut self,
        uniform_buffer: &UniformBuffer<Self>,
        gl_uindex: u32,
        byte_offset: usize,
        byte_length: usize,
    ) {
    }
}

 */
//ip mod3d_base::Renderable for Model3DWGpu
impl<'tgt> mod3d_base::Renderable for Model3DWGpu<'tgt> {
    type Buffer = (); // buffer::Buffer;
    type Accessor = (); // mod3d_gl::BufferView<Self>;
    type Texture = (); // texture::Texture;
    type Material = (); // mod3d_gl::Material;
    type Vertices = (); // mod3d_gl::Vertices<Self>;

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
        buffer_view: &BufferAccessor<Self>,
        attr: VertexAttr,
    ) {
        // client.init_buffer_view_client(buffer_view, attr, self);
    }

    //mp create_vertices_client
    fn create_vertices_client(&mut self, vertices: &mod3d_base::Vertices<Self>) -> Self::Vertices {
        // Self::Vertices::create(vertices, self)
    }

    //mp create_texture_client
    fn create_texture_client(&mut self, texture: &mod3d_base::Texture<Self>) -> Self::Texture {
        // eprintln!("Create texture client");
        // Self::Texture::of_texture(texture) // , self)
    }

    fn create_material_client<M>(
        &mut self,
        object: &mod3d_base::Object<M, Self>,
        material: &M,
    ) -> Self::Material
    where
        M: mod3d_base::Material,
    {
        // eprintln!("Create material client");
        // mod3d_gl::Material::create(self, object, material).unwrap()
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
