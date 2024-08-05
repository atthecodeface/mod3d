//a Imports
use crate::{GlShader, GlShaderType};
use web_sys::{WebGl2RenderingContext, WebGlShader};

//a Shader
//tp Shader
/// A WebGL shader, of any kind, which can be created from source.
///
/// A number of shaders are linked together to make a program; once
/// the program has been linked, the shader can be dropped.
pub struct Shader(WebGlShader);

//ip Shader
impl Shader {
    //fp compile
    /// Create a shader of a particular kind from source
    pub fn compile(
        context: &WebGl2RenderingContext,
        source: &str,
        shader_type: GlShaderType,
    ) -> Result<Self, String> {
        let (shader_type, shader_kind) = match shader_type {
            GlShaderType::Fragment => (WebGl2RenderingContext::FRAGMENT_SHADER, "fragment"),
            GlShaderType::Vertex => (WebGl2RenderingContext::VERTEX_SHADER, "vertex"),
        };
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| format!("Unable to create {} shader object", shader_kind))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Self(shader))
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| format!("Unknown error compiling {} shader", shader_kind)))
        }
    }
}

//ip GlShader for Shader
impl GlShader for Shader {
    type Id<'a> = &'a WebGlShader;
    //fp id
    /// Get the shader program id
    fn id(&self) -> &WebGlShader {
        &self.0
    }
}
