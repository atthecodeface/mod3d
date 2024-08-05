//tp UniformId
/// An enumeration of uniforms - that this crate particularly cares about
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UniformId {
    /// The view matrix uniform - once per framebuffer render
    ViewMatrix,
    /// The model matrix uniform - once per model instance
    ModelMatrix,
    /// The mesh matrix uniform - once per model mesh
    MeshMatrix,
    /// The Bone data uniform - once per model
    BoneScale,
    /// The Bone data uniform - once per model
    BoneMatrices,
    /// The Material data uniform - once per model, and it may have
    /// many forms, but it must start with ShaderMaterialBaseData
    Material,
    /// Texure uniform - dependent on the program.
    Texture(TextureId),
    /// User uniform - dependent on the program.
    User(u8),
    /// User uniform buffer - dependent on the program.
    Buffer(u8),
}

impl std::str::FromStr for UniformId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UniformId::*;
        let v = match s {
            "ViewMatrix" => ViewMatrix,
            "ModelMatrix" => ModelMatrix,
            "MeshMatrix" => MeshMatrix,
            "BoneScale" => BoneScale,
            "BoneMatrices" => BoneMatrices,
            "Material" => Material,
            _ => Err(format!("Cannot interpret {s} as a UniformID"))?,
        };
        Ok(v)
    }
}

//tp TextureId
/// An enumeration of texures - that this crate particularly cares about
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum TextureId {
    /// None
    #[default]
    None,
    /// The base color texture
    BaseColor,
    /// A normal texture
    Normal,
    /// The occlusion texture (as per Gltf)
    Occlusion,
    /// The emission texture (as per Gltf)
    Emission,
    /// The metallic-roughness texture (as per Gltf)
    MetallicRoughness,
    /// User 0
    User0,
}

impl std::str::FromStr for TextureId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TextureId::*;
        let v = match s {
            "BaseColor" => BaseColor,
            "Normal" => Normal,
            "Occlusion" => Occlusion,
            "Emission" => Emission,
            "MetallicRoughness" => MetallicRoughness,
            "User0" => User0,
            _ => Err(format!("Cannot interpret {s} as a TextureId"))?,
        };
        Ok(v)
    }
}

impl TextureId {
    pub fn of_material_aspect(m: mod3d_base::MaterialAspect) -> Self {
        use mod3d_base::MaterialAspect::*;
        #[allow(unreachable_patterns)]
        match m {
            Color => Self::BaseColor,
            Normal => Self::Normal,
            MetallicRoughness => Self::MetallicRoughness,
            Occlusion => Self::Occlusion,
            Emission => Self::Emission,
            _ => Self::None,
        }
    }
}
