//a Imports

//a Texture
//tp Texture
/// A null texture for now
#[derive(Debug, Clone, Default)]
pub struct Texture(u32);
impl std::fmt::Display for Texture {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip TextureClient for Texture
impl mod3d_base::TextureClient for Texture {}
