//a Imports
use std::cell::RefCell;
use std::rc::Rc;

use mod3d_base::{
    AccessorClient, BufferClient, BufferData, BufferDataAccessor, BufferDescriptor,
    BufferElementType, DescriptorClient, VertexDesc,
};

use super::{Buffer, Model3DWGpu};

//a VertexBuffer
//tp VertexBuffer
/// A simple structure provides a reference-counted WGpu buffer;
/// when the last reference is dropped it will drop the WGpu buffer
/// that it contains, if any.
///
/// This corresponds to an array of records of vertex data; the
/// records all have the same structure, described by the
/// Vec<VertexDesc>.
///
/// Its actual buffer is created from vertex data; from vertex data it
/// is created *only* on the first invocation (ultimately from a
/// [mod3d_base::BufferDataAccessor] client initialization) as
/// subsequent 'creations' will be duplicates
///
#[derive(Debug, Default)]
pub struct VertexBuffer {
    /// The WGpu Buffer
    buf: Buffer,
    /// The description of vertices from the object
    desc: Vec<VertexDesc>,
}

//ip VertexBuffer
impl VertexBuffer {
    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        self.buf.is_none()
    }

    //mp of_desc
    /// Create the WGpu buffer with all of the data
    pub fn of_desc(&mut self, wgpu: &Model3DWGpu, desc: &BufferDescriptor<Model3DWGpu>) {
        self.buf
            .create(wgpu, &desc.as_ref(), wgpu::BufferUsages::VERTEX);
        self.desc = desc.elements().to_vec();

        let attributes: Vec<wgpu::VertexAttribute> =
            wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4].to_vec();
        let desc = wgpu::VertexBufferLayout {
            array_stride: desc.stride() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        };
    }
    //zz All done
}

//ip Display for VertexBuffer
impl std::fmt::Display for VertexBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Vb({})", self.buf)
    }
}

//a RcVertexBuffer
//tp RcVertexBuffer
/// A reference-counted VertexBuffer
///
/// This can only be initialized *once*
#[derive(Debug, Default, Clone)]
pub struct RcVertexBuffer(Rc<RefCell<VertexBuffer>>);

impl RcVertexBuffer {
    pub fn init(&self, wgpu: &Model3DWGpu, desc: &BufferDescriptor<Model3DWGpu>) {
        self.0.borrow_mut().of_desc(wgpu, desc)
    }
}

//ip DescriptorClient for RcVertexBuffer
impl DescriptorClient for RcVertexBuffer {}

//a VertexAccessor
//tp VertexAccessor
#[derive(Debug, Default, Clone)]
pub struct VertexAccessor {
    buffer: RcVertexBuffer,
    desc_index: usize,
}

//ip AccessorClient for VertexAccessor
impl AccessorClient for VertexAccessor {}

//ip VertexAccessor
impl VertexAccessor {
    pub fn init<'tgt>(
        &mut self,
        wgpu: &mut Model3DWGpu<'tgt>,
        bda: &BufferDataAccessor<Model3DWGpu<'tgt>>,
    ) {
        bda.desc().create_client(wgpu);
        self.buffer = bda.desc().borrow_client().clone();
        self.desc_index = bda.desc_index() as usize;
    }
}
