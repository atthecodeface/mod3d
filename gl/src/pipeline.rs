use crate::{TextureId, UniformId};

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
pub struct PipelineDesc {
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

impl PipelineDesc {
    pub fn compile<F, CP, P>(
        self: Box<Self>,
        read_src: &F,
        compile_and_link_program: &CP,
    ) -> Result<P, String>
    where
        F: Fn(&str) -> Result<String, String>,
        CP: Fn(
            String,
            String,
            Vec<(String, mod3d_base::VertexAttr)>,
            Vec<(String, crate::UniformId)>,
            HashMap<String, usize>,
            Vec<(String, crate::TextureId, usize)>,
        ) -> Result<P, String>,
    {
        let frag_src = read_src(&self.fragment_src)?;
        let vert_src = read_src(&self.vertex_src)?;

        compile_and_link_program(
            vert_src,
            frag_src,
            self.attribute_map,
            self.uniform_map,
            self.uniform_buffer_map,
            self.texture_map,
        )
    }
}
