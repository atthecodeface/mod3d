//a Documentation

/*!

A shader program consists of a number of [webgl::Shader]s linked together

The attributes and uniforms can be accessed in a simple common manner
!*/

//a Imports
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::Shader;
use crate::{GlProgram, GlShader, TextureId, UniformId};

//a Program
//tp Program
/// A shader program, with its 'known' attributes and uniforms
pub struct Program {
    /// The GL ID of the program
    program: WebGlProgram,
    /// attribute names
    attributes: Vec<(u32, mod3d_base::VertexAttr)>,
    /// uniform names
    uniforms: Vec<(WebGlUniformLocation, UniformId)>,
    /// uniform buffer names
    uniform_buffers: Vec<(u32, usize)>,
    /// texture map from TextureId to uniform location and unit
    textures: Vec<(WebGlUniformLocation, TextureId, u32)>,
}

//ip Program
impl Program {
    //ap program
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }

    //fp link_program
    /// Compile a program from a slice of kind/source pairs
    pub fn link_program(
        context: &WebGl2RenderingContext,
        shaders: &[&Shader],
    ) -> Result<Program, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader program"))?;

        for shader in shaders {
            context.attach_shader(&program, shader.id());
        }
        context.link_program(&program);

        if !context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")));
        }

        let attributes = Vec::new();
        let uniforms = Vec::new();
        let uniform_buffers = Vec::new();
        let textures = Vec::new();
        Ok(Program {
            program,
            attributes,
            textures,
            uniforms,
            uniform_buffers,
        })
    }

    //mp add_attr_name
    /// Add an attribute to the [Program] from its name (that should be in the shader source)
    pub fn add_attr_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        vertex_attr: mod3d_base::VertexAttr,
    ) -> Result<&mut Self, String> {
        let attr_index = context.get_attrib_location(&self.program, name);
        if attr_index < 0 {
            Err(format!("Unable to find attribute {} in program", name))
        } else {
            let attr_index = attr_index as u32;
            self.attributes.push((attr_index, vertex_attr));
            Ok(self)
        }
    }

    //mp add_uniform_name
    /// Add a uniform to the [Program] from its name (that should be in the shader source)
    pub fn add_uniform_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        uniform_id: UniformId,
    ) -> Result<&mut Self, String> {
        if let Some(uniform_index) = context.get_uniform_location(&self.program, name) {
            self.uniforms.push((uniform_index, uniform_id));
            Ok(self)
        } else {
            Err(format!("Unable to find uniform {} in program", name))
        }
    }

    //mp add_uniform_buffer_name
    /// WebGl2RenderingContext supports uniform buffers
    pub fn add_uniform_buffer_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        id: usize,
    ) -> Result<&mut Self, String> {
        let uniform_index = context.get_uniform_block_index(&self.program, name);
        //        if uniform_index == gl::INVALID_INDEX {
        //            Err(format!("Unable to find uniform block {} in program", name))
        //        } else {
        self.uniform_buffers.push((uniform_index, id));
        Ok(self)
        //        }
    }

    //mp add_uniform_texture_name
    /// Add a texture assigned to a texture unit and a named uniform sampler
    ///
    /// The unit is 0 upwards; it must be mapped to gl::TEXTURE<unit>
    pub fn add_uniform_texture_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        texture_id: TextureId,
        unit: usize,
    ) -> Result<&mut Self, String> {
        if let Some(uniform_index) = context.get_uniform_location(&self.program, name) {
            self.textures.push((uniform_index, texture_id, unit as u32));
            Ok(self)
        } else {
            Err(format!("Unable to find uniform {} in program", name))
        }
    }

    //mp uniform_buffer
    pub fn uniform_buffer(&self, uniform_id: usize) -> Option<u32> {
        for (gl_id, u) in &self.uniform_buffers {
            if *u == uniform_id {
                return Some(*gl_id);
            }
        }
        None
    }
    //fp set_used
    pub fn set_used(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }
}

//ip GlProgram for Program
impl GlProgram for Program {
    type GlAttrId = u32;
    type GlUniformId<'a> = &'a WebGlUniformLocation;
    // type Context = WebGl2RenderingContext;
    fn attributes(&self) -> &[(Self::GlAttrId, mod3d_base::VertexAttr)] {
        &self.attributes
    }
    fn uniform(&self, uniform_id: UniformId) -> Option<Self::GlUniformId<'_>> {
        for (gl_id, u) in &self.uniforms {
            if *u == uniform_id {
                return Some(gl_id);
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
                return Some((uniform, *unit));
            }
        }
        None
    }
}
