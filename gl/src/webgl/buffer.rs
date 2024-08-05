//a Imports
use std::cell::RefCell;
use std::rc::Rc;

use mod3d_base::{BufferClient, BufferData};

use crate::webgl_log::log_gl_buffer;

use super::{Model3DWebGL, Program};
use crate::GlProgram;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

//a Buffer
//tp Buffer
/// A simple structure provides a reference-counted OpenGl buffer;
/// when the last reference is dropped it will drop the OpenGl buffer
/// that it contains, if any
///
/// Its actual buffer is created from vertex data or from indices;
/// from vertex data it is created *only* on the first invocation
/// (from a [mod3d_rs::BufferData]) as subsequent 'creations' will be
/// duplicates - the reference count should ont be changed either as
/// it is the *same* BufferData instance that is invoking the creation
///
/// For indices a buffer is created for the [mod3d_rs::BufferAccessor], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The WebGL Buffer if this has been set
    gl: Rc<RefCell<Option<WebGlBuffer>>>,
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        let gl = Rc::new(RefCell::new(None));
        Self { gl }
    }
}

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "WebGlBuf",)
    }
}

//ip BufferClient for Buffer
impl BufferClient for Buffer {}

//ip Buffer
impl Buffer {
    //ap is_none
    pub fn is_none(&self) -> bool {
        self.gl.borrow().is_none()
    }

    //mp of_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_data(&mut self, data: &BufferData<Model3DWebGL>, render_context: &Model3DWebGL) {
        assert!(self.is_none());
        let gl = render_context.create_buffer().unwrap();
        render_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&gl));
        // render_context.buffer_data_with_u8_array(
        //    WebGl2RenderingContext::ARRAY_BUFFER,
        //    data.as_slice(),
        //    WebGl2RenderingContext::STATIC_DRAW,
        // );
        unsafe {
            let buf_view = js_sys::Uint8Array::view(data.as_slice());
            render_context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        render_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        *self.gl.borrow_mut() = Some(gl);
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            // &format!("Buffer:of_data {} {:?}", self, data.as_slice()),
            &format!("Buffer:of_data {self}"),
            WebGl2RenderingContext::ARRAY_BUFFER,
            0,
            0, // data.as_slice().len(),
        );
    }

    //mp of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_indices(
        &mut self,
        view: &mod3d_base::BufferAccessor<Model3DWebGL>,
        render_context: &Model3DWebGL,
    ) {
        assert!(self.is_none());
        render_context.bind_vertex_array(None);
        let data = view.data.as_slice();
        let data = data.split_at(view.byte_offset as usize).1;
        let gl = render_context.create_buffer().unwrap();
        render_context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&gl));
        // render_context.buffer_data_with_u8_array(
        //    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        //    data,
        //    WebGl2RenderingContext::STATIC_DRAW,
        // );
        unsafe {
            let buf_view = js_sys::Uint8Array::view(data);
            render_context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        render_context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        *self.gl.borrow_mut() = Some(gl);
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            &format!("Buffer:of_indices {self}"),
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            0,
            0, // 3, // view.count as usize,
        );
    }

    //fp bind_to_context_buffer
    /// Bind the buffer to a context (e.g for indices to a vao)
    pub fn bind_to_context_buffer(&self, render_context: &Model3DWebGL, target: u32) {
        let gl_buffer_ref = self.gl.borrow();
        let gl_buffer = gl_buffer_ref.as_ref();
        render_context.bind_buffer(target, gl_buffer);
    }

    //fp bind_to_vao_attr
    /// Bind the buffer as a vertex attribute to the current VAO
    pub fn bind_to_vao_attr(
        &self,
        render_context: &Model3DWebGL,
        attr_id: <Program as GlProgram>::GlAttrId,
        count: u32,
        ele_type: mod3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    ) {
        let ele_type = {
            use mod3d_base::BufferElementType::*;
            match ele_type {
                Float32 => WebGl2RenderingContext::FLOAT,
                Float16 => WebGl2RenderingContext::HALF_FLOAT,
                Int8 => WebGl2RenderingContext::BYTE,
                Int16 => WebGl2RenderingContext::SHORT,
                Int32 => WebGl2RenderingContext::INT,
            }
        };
        crate::console_log!("bind_to_vao_attr");
        let gl_buffer_ref = self.gl.borrow();
        let gl_buffer = gl_buffer_ref.as_ref();
        render_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, gl_buffer);
        render_context.enable_vertex_attrib_array(attr_id);
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            &format!("Buffer:bind_to_vao_attr {attr_id}, #{count}, {ele_type:?}, {stride}, {byte_offset}"),
            0,
            0,
            0, // Do not output the buffer data
        );
        render_context.vertex_attrib_pointer_with_i32(
            attr_id,
            count as i32,
            ele_type,
            false,
            stride as i32,
            byte_offset as i32,
        );
    }

    //mp bind_buffer_range
    pub fn bind_buffer_range(
        &self,
        render_context: &Model3DWebGL,
        buffer_type: u32,
        gl_uindex: u32,
        byte_offset: i32,
        byte_length: i32,
    ) {
        let gl_buffer_ref = self.gl.borrow();
        let gl_buffer = gl_buffer_ref.as_ref();
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            &format!(
                "Buffer:bind_buffer_range: {buffer_type}, {gl_uindex}, {byte_offset}, #{byte_length}"
            ),
            0,
            0,
            0, // Do not output the buffer data
        );
        render_context.bind_buffer_range_with_i32_and_i32(
            buffer_type,
            gl_uindex,
            gl_buffer,
            byte_offset,
            byte_length,
        );
    }

    //mp uniform_buffer
    /// Create the OpenGL
    pub fn uniform_buffer<F: Sized>(
        &mut self,
        render_context: &Model3DWebGL,
        data: &[F],
        is_dynamic: bool,
    ) -> Result<(), ()> {
        assert!(self.is_none());
        let byte_length = std::mem::size_of_val(data);
        let buffer: *const u8 = &data[0] as *const F as *const u8;
        let buffer = unsafe { std::slice::from_raw_parts(buffer, byte_length) };

        let gl = render_context.create_buffer().unwrap();
        render_context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, Some(&gl));
        // render_context.buffer_data_with_u8_array(
        // WebGl2RenderingContext::UNIFORM_BUFFER,
        // buffer,
        // WebGl2RenderingContext::STATIC_DRAW,
        // );
        unsafe {
            let buf_view = js_sys::Uint8Array::view(buffer);
            render_context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::UNIFORM_BUFFER,
                &buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        render_context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, None);
        *self.gl.borrow_mut() = Some(gl);
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            &format!("Buffer:uniform_buffer: {is_dynamic}"),
            WebGl2RenderingContext::UNIFORM_BUFFER,
            0,
            0, // Do not output the buffer data
        );
        Ok(())
    }

    //fp uniform_update_data
    pub fn uniform_update_data<F: std::fmt::Debug>(
        &self,
        render_context: &Model3DWebGL,
        data: &[F],
        byte_offset: u32,
    ) {
        let byte_length = std::mem::size_of_val(data);
        let buffer: *const u8 = &data[0] as *const F as *const u8;
        let buffer = unsafe { std::slice::from_raw_parts(buffer, byte_length) };

        let gl_buffer_ref = self.gl.borrow();
        let gl_buffer = gl_buffer_ref.as_ref();
        render_context.bind_buffer(WebGl2RenderingContext::UNIFORM_BUFFER, gl_buffer);
        render_context.buffer_sub_data_with_i32_and_u8_array(
            WebGl2RenderingContext::UNIFORM_BUFFER,
            byte_offset as i32,
            buffer,
        );
        log_gl_buffer(
            render_context,
            self.gl.borrow().as_ref(),
            &format!("uniform_update_data: {:?} {}", data, byte_offset),
            0,
            0,
            0,
        );
    }

    //zz All done
}

//ip GlBuffer for Buffer
impl crate::GlBuffer for Buffer {}
