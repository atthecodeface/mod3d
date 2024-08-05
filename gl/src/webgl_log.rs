//a Log
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (
        #[allow(unused_unsafe)]
        unsafe { $crate::webgl_log::log(&format_args!($($t)*).to_string())}
    )
}

pub fn log_gl_buffer(
    context: &WebGl2RenderingContext,
    gl_buf: Option<&WebGlBuffer>,
    reason: &str,
    target: u32,
    byte_offset: i32,
    byte_length: usize,
) {
    if let Some(gl_buf) = gl_buf {
        let reason = format!("{}: GlBuf={:?}", reason, gl_buf as *const WebGlBuffer);
        log_gl_buffer_data(context, gl_buf, &reason, target, byte_offset, byte_length);
    } else {
        console_log!("{}: GlBuf=None", reason,);
    }
}

pub fn log_gl_vao(
    _context: &WebGl2RenderingContext,
    gl_vao: Option<&WebGlVertexArrayObject>,
    reason: &str,
) {
    if let Some(gl_vao) = gl_vao {
        console_log!(
            "{}: GlVao={:?}",
            reason,
            gl_vao as *const WebGlVertexArrayObject
        );
    } else {
        console_log!("{}: GlVao=None", reason,);
    }
}

pub fn log_gl_buffer_data(
    context: &WebGl2RenderingContext,
    gl_buf: &WebGlBuffer,
    reason: &str,
    target: u32,
    byte_offset: i32,
    byte_length: usize,
) {
    let data = {
        if byte_length > 0 {
            // let mut data = vec![0u8; byte_length];
            context.bind_buffer(target, Some(gl_buf));

            let mut dst_data = vec![0u8; byte_length];
            let dst_array = js_sys::Uint8Array::new_with_length(byte_length as u32);
            context.get_buffer_sub_data_with_i32_and_array_buffer_view(
                target,
                byte_offset,
                &dst_array,
            );
            dst_array.copy_to(&mut dst_data);
            // context.get_buffer_sub_data_with_i32_and_u8_array_and_dst_offset(
            // target,
            // byte_offset,
            // &mut data,
            // 0,
            // );
            context.bind_buffer(target, None);
            dst_data
        } else {
            vec![0; 0]
        }
    };
    console_log!("{}: data={:?}", reason, data);
}
