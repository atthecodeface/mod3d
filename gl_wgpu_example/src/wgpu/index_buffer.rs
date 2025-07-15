//a Imports
use std::cell::RefCell;
use std::rc::Rc;

use mod3d_base::{
    AccessorClient, BufferClient, BufferData, BufferDataAccessor, BufferDescriptor,
    BufferElementType, BufferIndexAccessor, DescriptorClient,
};

use super::{Buffer, Model3DWGpu};

//a IndexBuffer
//tp IndexBuffer
/// A simple structure provides a reference-counted WGpu buffer;
/// when the last reference is dropped it will drop the WGpu buffer
/// that it contains, if any.
///
/// This corresponds to an array of records of vertex data; the
/// records all have the same structure, described by the
/// Vec<IndexDesc>.
///
/// Its actual buffer is created from vertex data; from vertex data it
/// is created *only* on the first invocation (ultimately from a
/// [mod3d_base::BufferDataAccessor] client initialization) as
/// subsequent 'creations' will be duplicates
///
#[derive(Debug, Default, Clone)]
pub struct IndexBuffer {
    /// The WGpu Buffer
    buf: Buffer,
}

//ip IndexBuffer
impl IndexBuffer {
    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        self.buf.is_none()
    }

    //mp init
    /// Create the WGpu buffer with the data required by the IndexAccessor
    ///
    /// This is invoked by the 'create_client' method of the desc
    pub fn init(&mut self, wgpu: &Model3DWGpu, desc: &BufferIndexAccessor<Model3DWGpu>) {
        if self.is_none() {
            self.buf
                .create(wgpu, &desc.as_ref(), wgpu::BufferUsages::INDEX);
        }
    }

    //zz All done
}

//ip Display for IndexBuffer
impl std::fmt::Display for IndexBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Vb({})", self.buf)
    }
}

//ip AccessorClient for RcVertexBuffer
impl AccessorClient for IndexBuffer {}
