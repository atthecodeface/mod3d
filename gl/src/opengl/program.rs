//a Documentation

/*!

A shader program consists of a number of [GlShader]s linked together

!*/

//a Imports
use std::ffi::CString;

use super::utils;
use super::Shader;
use crate::{TextureId, UniformId};

//a Program
//tp Program
/// A shader program, with its 'known' attributes and uniforms
pub struct Program {
    /// The GL ID of the program
    id: gl::types::GLuint,
    /// attribute map
    attributes: Vec<(gl::types::GLuint, mod3d_base::VertexAttr)>,
    /// uniform map from UniformId to location
    uniforms: Vec<(gl::types::GLint, UniformId)>,
    /// texture map from TextureId to uniform location and unit
    textures: Vec<(gl::types::GLint, TextureId, u32)>,
}

///ip Program
impl Program {
    //mp get_attributes
    pub fn get_attributes(&self) -> Vec<String> {
        let mut count = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::ACTIVE_ATTRIBUTES, &mut count);
        }
        let mut result = Vec::with_capacity(count as usize);
        for i in 0..(count as u32) {
            let mut name: [u8; 256] = [0; 256];
            let mut length = 0;
            let mut size = 0;
            let mut gl_type = 0;
            unsafe {
                gl::GetActiveAttrib(
                    self.id,
                    i,
                    256,
                    &mut length,
                    &mut size,
                    &mut gl_type,
                    name.as_mut_ptr() as *mut gl::types::GLchar,
                );
            }
            result.push(
                std::str::from_utf8(&name[..length as usize])
                    .unwrap()
                    .to_string(),
            );
        }
        result
    }

    //mp get_uniforms
    pub fn get_uniforms(&self) -> Vec<String> {
        let mut count = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut count);
        }
        let mut result = Vec::with_capacity(count as usize);
        for i in 0..(count as u32) {
            let mut name: [u8; 256] = [0; 256];
            let mut length = 0;
            let mut size = 0;
            let mut gl_type = 0;
            unsafe {
                gl::GetActiveUniform(
                    self.id,
                    i,
                    256,
                    &mut length,
                    &mut size,
                    &mut gl_type,
                    name.as_mut_ptr() as *mut gl::types::GLchar,
                );
            }
            result.push(
                std::str::from_utf8(&name[..length as usize])
                    .unwrap()
                    .to_string(),
            );
        }
        result
    }

    //fp link_program
    /// Compile a program from a slice of kind/source pairs
    pub fn link_program(shaders: &[&Shader]) -> Result<Program, String> {
        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(program_id, shader.id());
            }
            gl::LinkProgram(program_id);
            program_id
        };

        if utils::get_programiv(program_id, gl::LINK_STATUS) == 0 {
            let err = utils::get_shader_error(
                program_id,
                |id| utils::get_programiv(id, gl::INFO_LOG_LENGTH),
                |id, len, buf| unsafe { gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buf) },
            );
            Err(format!(
                "Unable to create shader program, linking error {}",
                err
            ))?;
        }
        utils::check_errors().expect("Linked");

        let attributes = Vec::new();
        let uniforms = Vec::new();
        let textures = Vec::new();
        let program = Program {
            id: program_id,
            attributes,
            uniforms,
            textures,
        };
        Ok(program)
    }

    //mp add_attr_name
    /// Add an attribute to the [Program] from its name (that should be in the shader source)
    pub fn add_attr_name(
        &mut self,
        name: &str,
        vertex_attr: mod3d_base::VertexAttr,
    ) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let attr_index = unsafe { gl::GetAttribLocation(self.id, name_c.as_ptr()) };
        if attr_index < 0 {
            Err(format!("Unable to find attribute {} in program", name))
        } else {
            self.attributes
                .push((attr_index as gl::types::GLuint, vertex_attr));
            Ok(self)
        }
    }

    //mp add_uniform_name
    /// Add a uniform to the [Program] from its name (that should be in the shader source)
    pub fn add_uniform_name(
        &mut self,
        name: &str,
        uniform_id: UniformId,
    ) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let uniform_index = unsafe { gl::GetUniformLocation(self.id, name_c.as_ptr()) };
        if uniform_index == (gl::INVALID_INDEX as i32) {
            Err(format!("Unable to find uniform {} in program", name))
        } else {
            self.uniforms
                .push((uniform_index as gl::types::GLint, uniform_id));
            Ok(self)
        }
    }

    //mp add_uniform_buffer_name
    /// Add a uniform buffer (or 'block') to the [Program] from its
    /// name (that should be in the shader source)
    pub fn add_uniform_buffer_name(&mut self, name: &str, id: usize) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let uniform_index = unsafe { gl::GetUniformBlockIndex(self.id, name_c.as_ptr()) };
        if uniform_index == gl::INVALID_INDEX {
            Err(format!("Unable to find uniform block {} in program", name))
        } else {
            self.uniforms.push((
                uniform_index as gl::types::GLint,
                UniformId::Buffer(id as u8),
            ));
            Ok(self)
        }
    }

    //mp add_uniform_texture_name
    /// Add a texture assigned to a texture unit and a named uniform sampler
    ///
    /// The unit is 0 upwards; it must be mapped to gl::TEXTURE<unit>
    pub fn add_uniform_texture_name(
        &mut self,
        name: &str,
        texture_id: TextureId,
        unit: usize,
    ) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let uniform_index = unsafe { gl::GetUniformLocation(self.id, name_c.as_ptr()) };
        if uniform_index == (gl::INVALID_INDEX as i32) {
            Err(format!("Unable to find texture {} in program", name))
        } else {
            let gl_unit = unit as u32;
            self.textures
                .push((uniform_index as gl::types::GLint, texture_id, gl_unit));
            Ok(self)
        }
    }

    //fp id
    /// Get the program id
    #[inline]
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    //fp set_used
    /// Use the program
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }
}

//ip Drop for Program
impl Drop for Program {
    //fp drop
    /// Drop requires the GLProgram to be deleted
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    //zz All done
}

//ip GlProgram for Program
impl crate::GlProgram for Program {
    type GlAttrId = gl::types::GLuint;
    type GlUniformId<'a> = gl::types::GLint;
    fn attributes(&self) -> &[(gl::types::GLuint, mod3d_base::VertexAttr)] {
        &self.attributes
    }
    fn uniform(&self, uniform_id: UniformId) -> Option<gl::types::GLint> {
        for (gl_id, u) in &self.uniforms {
            if *u == uniform_id {
                return Some(*gl_id);
            }
        }
        None
    }
    fn texture_uniform(
        &self,
        texture_id: crate::TextureId,
    ) -> Option<(Self::GlUniformId<'_>, u32)> {
        for (uniform, t, unit) in &self.textures {
            if *t == texture_id {
                return Some((*uniform, *unit));
            }
        }
        None
    }
}
