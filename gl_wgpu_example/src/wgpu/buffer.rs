//a Imports
use std::rc::Rc;

use mod3d_base::{BufferClient, BufferData, BufferElementType};
use wgpu::util::DeviceExt;

use super::Model3DWGpu;

//a Buffer
//tp Buffer
/// A simple structure provides a reference-counted WGpu buffer;
/// when the last reference is dropped it will drop the WGpu buffer
/// that it contains, if any
///
/// Its actual buffer is created from vertex data or from indices;
/// from vertex data it is created *only* on the first invocation
/// (from a [mod3d_base::BufferData]) as subsequent 'creations' will be
/// duplicates - the reference count should ont be changed either as
/// it is the *same* BufferData instance that is invoking the creation
///
/// For indices a buffer is created for the [mod3d_base::BufferIndexAccessor], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The WGpu Buffer
    buf: Rc<Option<wgpu::Buffer>>,
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        let buf = Rc::new(None);
        Self { buf }
    }
}

//ip BufferClient for Buffer
impl BufferClient for Buffer {}

//ip Buffer
impl Buffer {
    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        self.buf.is_none()
    }

    //mp create
    /// Create the WGpu buffer with all of the data
    ///
    /// wgpu::BufferUsages::VERTEX, INDEX, UNIFORM, COPY_DST, COPY_SRC, MAP_READ, MAP_WRITE
    pub fn create<F: Sized>(&mut self, wgpu: &Model3DWGpu, data: &[F], usage: wgpu::BufferUsages) {
        if self.buf.is_none() {
            let byte_length = std::mem::size_of_val(data);
            let data = data.as_ptr() as *const u8;
            // SAFETY:
            //
            // These values are directly from the slice of F
            let data = unsafe { std::slice::from_raw_parts(data, byte_length) };
            self.buf = Some(
                wgpu.device()
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(&format!(
                            "Buffer of {byte_length} bytes with usage {usage:0x?}"
                        )),
                        contents: data,
                        usage,
                    }),
            )
            .into();
        }
    }

    //fp uniform_update_data
    pub fn uniform_update_data<F: Sized>(&self, data: &[F], byte_offset: u32) {}

    //zz All done
}

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if let Some(buffer) = self.buf.as_ref() {
            write!(f, "Buffer({:?})", buffer)
        } else {
            write!(f, "Buffer(<none>)")
        }
    }
}
