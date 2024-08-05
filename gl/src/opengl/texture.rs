//a Imports
use std::rc::Rc;

use mod3d_base::TextureClient;

use crate::Model3DOpenGL;

//a Texture
//tp Texture
/// A simple structure provides a reference-counted OpenGl texture;
/// when the last reference is dropped it will drop the OpenGl texture
/// that it contains, if any
///
#[derive(Debug, Clone)]
pub struct Texture {
    /// The OpenGL Texture
    gl: Rc<gl::types::GLuint>,
}

//ip Default for Texture
impl Default for Texture {
    fn default() -> Self {
        let gl = Rc::new(0);
        Self { gl }
    }
}

//ip TextureClient for Texture
impl TextureClient for Texture {}

//ip Texture
impl Texture {
    //ap gl_texture
    /// Get the gl_buffer associated with the data
    pub fn gl_texture(&self) -> gl::types::GLuint {
        *self.gl
    }

    //ap as_ptr
    /// Get a pointer to this gl
    pub fn as_ptr(&self) -> *const gl::types::GLuint {
        Rc::as_ptr(&self.gl)
    }

    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        *self.gl == 0
    }

    //mp of_texture
    /// Create a texture
    pub fn of_texture(texture: &mod3d_base::Texture<Model3DOpenGL>) -> Self {
        let mut gl: gl::types::GLuint = 0;
        let (width, height, depth) = *texture.dims();
        assert!(depth == 0);
        assert!(height != 0);
        let data_type = texture.data_type();
        let data_format = {
            match data_type.0 {
                1 => gl::RED,
                2 => gl::RG,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => gl::RGBA,
            }
        };
        let data_type = {
            match data_type.1 {
                mod3d_base::BufferElementType::Int8 => gl::UNSIGNED_BYTE,
                mod3d_base::BufferElementType::Int16 => gl::UNSIGNED_SHORT,
                mod3d_base::BufferElementType::Float16 => gl::SHORT,
                mod3d_base::BufferElementType::Float32 => gl::FLOAT,
                _ => gl::UNSIGNED_BYTE,
            }
        };
        unsafe {
            gl::GenTextures(1, (&mut gl) as *mut gl::types::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, gl);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                data_format,
                data_type,
                texture.data().as_ptr() as *const gl::types::GLvoid,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            eprintln!("Created texture {gl:?}");
        }
        Self { gl: Rc::new(gl) }
    }

    //zz All done
}

//ip Drop for Texture
impl Drop for Texture {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if Rc::strong_count(&self.gl) == 1 && !self.is_none() {
            unsafe {
                gl::DeleteTextures(1, self.as_ptr());
            }
        }
    }
}
