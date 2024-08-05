// a Imports
use model3d_gl::{Gl, GlShaderType, Model3DWebGL};
type GlProgram = <Model3DWebGL as model3d_gl::Gl>::Program;
pub fn compile_shader_program(model3d: &Model3DWebGL) -> Result<GlProgram, String> {
    let vert_shader = model3d.compile_shader(
        GlShaderType::Vertex,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
        
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = model3d.compile_shader(
        GlShaderType::Fragment,
        r##"#version 300 es
    
        precision highp float;
        out vec4 outColor;
        
        void main() {
            outColor = vec4(1, 1, 1, 1);
        }
        "##,
    )?;
    model3d.link_program(
        &[&vert_shader, &frag_shader],
        &[("position", model3d_base::VertexAttr::Position)],
        &[],
        &[],
    )
}
