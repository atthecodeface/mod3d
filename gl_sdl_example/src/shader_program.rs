//a Imports
use std::path::Path;

use mod3d_gl::Gl;

use crate::utils::read_file;

pub fn create_shader_program<G: Gl>(
    gl: &mut G,
    shader_filename: &str,
    shader_paths: &[&Path],
) -> Result<G::Program, anyhow::Error> {
    let shader = read_file(shader_paths, shader_filename)?;
    let shader: mod3d_gl::ShaderProgramDesc = serde_json::from_str(&shader)?;
    let shader_program = shader
        .compile(gl, &|filename| {
            read_file(shader_paths, filename).map_err(|e| e.to_string())
        })
        .map_err(|e| anyhow::anyhow!("Failed to read shader program {e}"))?;

    Ok(shader_program)
}