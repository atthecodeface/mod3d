//a Documentation
// #![warn(missing_docs)]
// Document code examples cannot be executed
// so don't require them right now
//
// #![warn(rustdoc::missing_doc_code_examples)]

/*!
# OpenGL/WeblGL Model / Shader Program abstraction library

This library provides structures for OpenGL shaders ...
!*/

//a Imports and exports
pub use mod3d_base::{Mat3, Mat4, Quat, Transformation, Vec3, Vec4};

mod types;
pub use types::{TextureId, UniformId};

mod traits;
pub use traits::{Gl, GlBuffer, GlProgram, GlShader, GlShaderType, GlVao};

//a Submodules
mod material;
mod texture;
pub use material::Material;
pub use texture::Texture;

mod buffer;
pub use buffer::{BufferView, IndexBuffer, UniformBuffer, VertexBuffer};

mod program;
pub use program::ShaderProgramDesc;

mod vertices;
pub use vertices::Vertices;

mod shader_instantiable;
pub use shader_instantiable::{ShaderInstantiable, ShaderMaterialBaseData};

//a Model3DWebGL
#[cfg(feature = "webgl")]
mod webgl;
#[cfg(feature = "webgl")]
mod webgl_log;
#[cfg(feature = "webgl")]
pub use webgl::Model3DWebGL;

//a Model3DOpenGL
#[cfg(feature = "opengl")]
mod opengl;
#[cfg(feature = "opengl")]
pub use opengl::utils as opengl_utils;
#[cfg(feature = "opengl")]
pub use opengl::Model3DOpenGL;
