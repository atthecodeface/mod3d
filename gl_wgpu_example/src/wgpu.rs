use mod3d_base::{
    BufferDataAccessor, BufferDescriptor, BufferElementType, BufferIndexAccessor, VertexAttr,
};

use mod3d_gl::{Gl, GlProgram, GlShaderType, Mat4, UniformBuffer};

mod buffer;
mod index_buffer;
mod vertex_buffer;

use crate::utils::rtc::run_to_completion as rtc;

pub use buffer::Buffer;
pub use index_buffer::IndexBuffer;
pub use vertex_buffer::{RcVertexBuffer, VertexAccessor, VertexBuffer};

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
    //ap device
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    //cp new
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

//ip mod3d_base::Renderable for Model3DWGpu

// Renderable provides a means to generate an Instantiable which does
// not have the lifetime of the original data. The Instantiable has a
// RenderRecipe with the primitives that need to be drawn.
//
// It *can* be used to generate in a compatible RenderPipeline, using a RenderPass
//
// RenderPiperlineDescription has:
//
//  PipelineLayout
//
//  VertexState - an array of VertexBufferLayout, each of which must be given a set_vertex_buffer, and each of which contains one or more vertex attribute data. Plus a shader program.
//
//  FragmentState
//
//  Primitive Toplogy - TriangleStrip, Lines, etc
//
// RenderPass has:
//
//  set_pipeline( Pipeline ) - which influences the slot numbers
//
//  set_index_buffer( Buffer Slice, Index Format ) - so the Vertices client must have that information
//
//  set_vertex_buffer( slot #, Buffer Slice ) - so the Vertices client must have some vec of Buffer Slice
//
// The RenderRecipe requires each Primitive to refer to the corrent RenderPass somehow
//
// Note: BufferSlice has a sublifetime of the Buffer it refers to
//
impl<'tgt> mod3d_base::Renderable for Model3DWGpu<'tgt> {
    type Buffer = (); // Buffers are actually never created from data
    type Descriptor = vertex_buffer::RcVertexBuffer;
    type DataAccessor = vertex_buffer::VertexAccessor;
    type IndexAccessor = index_buffer::IndexBuffer;
    type Texture = (); // In the Instantiable
    type Material = (); // In the Instantiable
    type Vertices = (); // In the Instantiable

    //mp init_buffer_data_client
    /// Initialize a BufferData client
    ///
    /// This may be called multiple times for the same [BufferData]; if the
    /// gl buffer is 0 then create, else it already exists with the same data
    fn init_buffer_data_client(
        &mut self,
        _client: &mut Self::Buffer,
        _buffer_data: &mod3d_base::BufferData<Self>,
    ) {
        panic!("Buffers should never have their client created");
    }

    //mp init_buffer_desc_client
    /// Initialize a buffer descriptor client
    fn init_buffer_desc_client(
        &mut self,
        client: &mut Self::Descriptor,
        buffer_desc: &BufferDescriptor<Self>,
    ) {
        client.init(self, buffer_desc);
    }

    //mp init_index_accessor_client
    /// Initialize a client of a BufferIndexAccessor
    fn init_index_accessor_client(
        &mut self,
        client: &mut Self::IndexAccessor,
        index_accessor: &BufferIndexAccessor<Self>,
    ) {
    }

    //mp init_data_accessor_client
    /// Initialize a buffer view client
    fn init_data_accessor_client(
        &mut self,
        client: &mut Self::DataAccessor,
        bda: &BufferDataAccessor<Self>,
    ) {
        client.init(self, bda);
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
