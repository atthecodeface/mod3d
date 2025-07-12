//a Imports
use super::Model3DWGpu;
use mod3d_base::{BufferClient, BufferData, BufferElementType};
use std::rc::Rc;

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
/// For indices a buffer is created for the [mod3d_base::BufferAccessor], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The WGpu Buffer
    gl: Rc<Option<wgpu::Buffer>>,
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        let gl = Rc::new(None);
        Self { gl }
    }
}

//ip BufferClient for Buffer
impl BufferClient for Buffer {}

//ip Buffer
impl Buffer {
    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        self.gl.is_none()
    }

    //mp of_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_data(&mut self, data: &BufferData<Model3DWGpu>, render_context: &Model3DWGpu) {
        assert!(self.is_none());
        self.gl = Some(render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: data.as_slice(),
                usage: wgpu::BufferUsages::VERTEX,
            },
        ))
        .into();
    }

    //mp of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_indices(
        &mut self,
        view: &mod3d_base::BufferAccessor<Model3DWGpu>,
        render_context: &Model3DWGpu,
    ) {
        assert!(self.is_none());

        self.gl = Some(render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: data.as_slice(),
                usage: wgpu::BufferUsages::INDEX,
            },
        ))
        .into();
    }

    //mp uniform_buffer
    /// Create the OpenGL
    pub fn uniform_buffer<F: Sized>(&mut self, data: &[F], _is_dynamic: bool) -> Result<(), ()> {
        assert!(self.is_none());
        self.gl = Some(render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: data.as_slice(),
                usage: wgpu::BufferUsages::INDEX,
            },
        ))
        .into();
        let buffer = data.as_ptr();
        let byte_length = std::mem::size_of_val(data);
        let mut gl: gl::types::GLuint = 0;
        unsafe {
            gl::BindVertexArray(0);
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint);
            gl::BindBuffer(gl::UNIFORM_BUFFER, gl);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                byte_length as gl::types::GLsizeiptr,
                buffer as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0); // unbind to protect
        }
        self.gl = Rc::new(gl);
        Ok(())
    }

    //fp uniform_update_data
    pub fn uniform_update_data<F: Sized>(&self, data: &[F], byte_offset: u32) {
        let buffer = data.as_ptr();
        let byte_length = std::mem::size_of_val(data);
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.gl_buffer());
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                byte_offset as isize,
                byte_length as isize,
                buffer as *const std::os::raw::c_void,
            );
        }
    }

    //zz All done
}

//ip Drop for Buffer
impl Drop for Buffer {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if Rc::strong_count(&self.gl) == 1 && !self.is_none() {
            unsafe {
                gl::DeleteBuffers(1, self.as_ptr());
            }
        }
    }
}

//ip GlBuffer for Buffer
impl crate::GlBuffer for Buffer {}

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "GL({})", self.gl)
    }
}
