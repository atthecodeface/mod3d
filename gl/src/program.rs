use crate::{Gl, GlShaderType, TextureId, UniformId};

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg(feature = "serde")]
mod deserialize {
    use crate::{TextureId, UniformId};
    use serde::Deserializer;
    use std::collections::HashMap;

    //fi map_name_to_attr
    /// Map an array of attribute name/value pairs to a Vec of
    /// tuples of named and mod3d_base::VertexAttr
    pub fn map_name_to_attr<'de, D>(
        de: D,
    ) -> std::result::Result<Vec<(String, mod3d_base::VertexAttr)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m: HashMap<String, String> = serde::de::Deserialize::deserialize(de)?;
        let mut r = vec![];
        for (k, v) in m.into_iter() {
            use mod3d_base::VertexAttr::*;
            let v = match v.as_ref() {
                "Position" => Position,
                "Normal" => Normal,
                "Color" => Color,
                "Tangent" => Tangent,
                "Joints" => Joints,
                "Weights" => Weights,
                "TexCoords0" => TexCoords0,
                "TexCoords1" => TexCoords1,
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "Unknown attribute name {k}"
                    )));
                }
            };
            r.push((k, v.into()));
        }
        Ok(r)
    }

    //fi map_name_to_uniform
    /// Map an array of attribute name/value pairs to a Vec of
    /// tuples of named and mod3d_base::VertexAttr
    pub fn map_name_to_uniform<'de, D>(
        de: D,
    ) -> std::result::Result<Vec<(String, UniformId)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m: HashMap<String, String> = serde::de::Deserialize::deserialize(de)?;
        let mut r = vec![];
        for (k, v) in m.into_iter() {
            let v = v.parse().map_err(serde::de::Error::custom)?;
            r.push((k, v));
        }
        Ok(r)
    }

    //fi map_name_to_texture_unit
    /// Map an array of attribute name/value pairs to a Vec of
    /// tuples of named and mod3d_base::VertexAttr
    pub fn map_name_to_texture_unit<'de, D>(
        de: D,
    ) -> std::result::Result<Vec<(String, TextureId, usize)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m: HashMap<String, (String, usize)> = serde::de::Deserialize::deserialize(de)?;
        let mut r = vec![];
        for (k, (name, unit)) in m.into_iter() {
            let t = name.parse().map_err(serde::de::Error::custom)?;
            r.push((k, t, unit));
        }
        Ok(r)
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct ShaderProgramDesc {
    /// The vertex shader path name
    vertex_src: String,

    /// The fragment shader path name
    fragment_src: String,

    /// The map from shader attribute names to the mod3d_base names
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::map_name_to_attr")
    )]
    attribute_map: Vec<(String, mod3d_base::VertexAttr)>,

    /// The map from shader uniform names to the UniformId names
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::map_name_to_uniform")
    )]
    uniform_map: Vec<(String, UniformId)>,

    /// The map from shader uniform names to the UniformId names
    uniform_buffer_map: HashMap<String, usize>,

    /// The map from shader uniform names to the UniformId names
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize::map_name_to_texture_unit")
    )]
    texture_map: Vec<(String, TextureId, usize)>,
}

impl ShaderProgramDesc {
    pub fn compile<F, G>(&self, gl: &G, read_src: &F) -> Result<<G as Gl>::Program, String>
    where
        F: Fn(&str) -> Result<String, String>,
        G: Gl,
    {
        let frag_src = read_src(&self.fragment_src)?;
        let vert_src = read_src(&self.vertex_src)?;

        let frag_shader = gl.compile_shader(GlShaderType::Fragment, &frag_src)?;
        let vert_shader = gl.compile_shader(GlShaderType::Vertex, &vert_src)?;

        let named_attrs: Vec<(&str, mod3d_base::VertexAttr)> = self
            .attribute_map
            .iter()
            .map(|(s, a)| (s.as_str(), *a))
            .collect();
        let named_uniforms: Vec<(&str, UniformId)> = self
            .uniform_map
            .iter()
            .map(|(s, a)| (s.as_str(), *a))
            .collect();
        let named_uniform_buffers: Vec<(&str, usize)> = self
            .uniform_buffer_map
            .iter()
            .map(|(s, a)| (s.as_str(), *a))
            .collect();
        let named_textures: Vec<(&str, TextureId, usize)> = self
            .texture_map
            .iter()
            .map(|(s, t, u)| (s.as_str(), *t, *u))
            .collect();
        let program = gl.link_program(
            &[&vert_shader, &frag_shader],
            &named_attrs,
            &named_uniforms,
            &named_uniform_buffers,
            &named_textures,
        )?;
        Ok(program)
    }
}
