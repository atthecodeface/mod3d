//a Imports
use std::path::Path;

pub fn read_file(shader_paths: &[&Path], filename: &str) -> Result<String, anyhow::Error> {
    if let Ok(x) = std::fs::read_to_string(filename) {
        Ok(x)
    } else {
        for p in shader_paths {
            let pb = p.join(filename);
            if let Ok(x) = std::fs::read_to_string(&pb) {
                println!("Shader: {x}");
                return Ok(x);
            }
        }
        Err(anyhow::anyhow!("Failed to read shader program {filename}"))
    }
}
