//a Imports
use std::rc::Rc;

use mod3d_base::TextureClient;

use super::Model3DWebGL;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

//a Texture
//tp Texture
/// A simple structure provides a reference-counted OpenGl texture;
/// when the last reference is dropped it will drop the OpenGl texture
/// that it contains, if any
///
#[derive(Debug, Clone)]
pub struct Texture {
    /// The OpenGL Texture
    gl: Rc<Option<WebGlTexture>>,
}

//ip Default for Texture
impl Default for Texture {
    fn default() -> Self {
        let gl = Rc::new(None);
        Self { gl }
    }
}

//ip TextureClient for Texture
impl TextureClient for Texture {}

//ip Texture
impl Texture {
    //ap gl_texture
    /// Get the gl_buffer associated with the data
    pub fn gl_texture(&self) -> Option<&WebGlTexture> {
        (self.gl.as_ref()).as_ref()
    }

    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        self.gl.is_none()
    }

    //mp of_texture
    /// Create a texture
    pub fn of_texture(
        texture: &mod3d_base::Texture<Model3DWebGL>,
        render_context: &WebGl2RenderingContext,
    ) -> Self {
        let (width, height, depth) = *texture.dims();
        assert!(depth == 0);
        assert!(height != 0);
        let data_type = texture.data_type();
        let data_format = {
            match data_type.0 {
                1 => WebGl2RenderingContext::RED,
                2 => WebGl2RenderingContext::RG,
                3 => WebGl2RenderingContext::RGB,
                4 => WebGl2RenderingContext::RGBA,
                _ => WebGl2RenderingContext::RGBA,
            }
        };
        let data_type = {
            match data_type.1 {
                mod3d_base::BufferElementType::Int8 => WebGl2RenderingContext::UNSIGNED_BYTE,
                mod3d_base::BufferElementType::Int16 => WebGl2RenderingContext::UNSIGNED_SHORT,
                mod3d_base::BufferElementType::Float16 => WebGl2RenderingContext::SHORT,
                mod3d_base::BufferElementType::Float32 => WebGl2RenderingContext::FLOAT,
                _ => WebGl2RenderingContext::UNSIGNED_BYTE,
            }
        };
        let gl = render_context.create_texture().unwrap();
        render_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&gl));
        render_context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width as i32,
            height as i32,
            0,
            data_format,
            data_type,
            Some(texture.data()),
        );

        render_context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::REPEAT as i32,
        );
        render_context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::REPEAT as i32,
        );
        render_context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        render_context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        render_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Self {
            gl: Rc::new(Some(gl)),
        }
    }

    //zz All done
}
