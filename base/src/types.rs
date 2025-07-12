//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

//a Basic types
//tp Vec3
/// 3-dimensional vector
pub type Vec3 = [f32; 3];

//tp Vec4
/// 3-dimensional vector with extra coord (1 for position, 0 for direction)
pub type Vec4 = [f32; 4];

//tp Mat3
/// 3-by-3 matrix for transformation of Vec3
pub type Mat3 = [f32; 9];

//tp Mat4
/// 4-by-4 matrix for transformation of Vec4
pub type Mat4 = [f32; 16];

//tp Quat - Quaternion
/// Quaternion
pub type Quat = [f32; 4];

//a Buffer
//tp BufferElementType
/// The type of an element in a buffer
///
/// This deliberately does not implement Default
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum BufferElementType {
    /// 32-bit floating point
    Float32,
    /// 16-bit floating point
    Float16,
    /// Signed 8-bit integers
    SInt8,
    /// Signed 16-bit integers
    SInt16,
    /// Signed 32-bit integers
    SInt32,
    /// Unsigned 8-bit integers
    UInt8,
    /// Unsigned 16-bit integers
    UInt16,
    /// Unsigned 32-bit integers
    UInt32,
}

//ip BufferElementType
impl BufferElementType {
    /// Create a 16-bit float type
    pub const fn float16() -> Self {
        Self::Float16
    }
    /// Create a 32-bit float type
    pub const fn float32() -> Self {
        Self::Float32
    }
    /// Create a signed/unsigned int type
    pub const fn new_int(signed: bool, bits: usize) -> Self {
        match bits {
            8 => {
                if signed {
                    Self::SInt8
                } else {
                    Self::UInt8
                }
            }
            16 => {
                if signed {
                    Self::SInt16
                } else {
                    Self::UInt16
                }
            }
            32 => {
                if signed {
                    Self::SInt32
                } else {
                    Self::UInt32
                }
            }
            _ => {
                panic!("An int value must be 8, 16 or 32 bits");
            }
        }
    }

    /// Get the length in bytes of the element type
    pub fn byte_length(self) -> u32 {
        use BufferElementType::*;
        match self {
            Float32 => 4,
            Float16 => 2,
            SInt8 => 1,
            SInt16 => 2,
            SInt32 => 4,
            UInt8 => 1,
            UInt16 => 2,
            UInt32 => 4,
        }
    }
}

//tp VertexDesc
/// A descriptor of the contents of one aspect of data for a Vertex;
/// in essence a 'field' of a Vertex structure
///
/// Such an field is a scalar, vector, or matrix of
/// [BufferElementType], starting at a byte offset within the parent
/// structure, and it has a target [VertexAttr] usage
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VertexDesc {
    /// Vertex attribute that this describes, e.g. Position, TexCoordsN, Joints
    attr: VertexAttr,

    /// Byte offset within the parent to the first element
    byte_offset: u16,

    /// Dimensions, e.g. [0, 0] for a scalar, [3,0] for a Vec3, [4,4] for a Mat4
    dims: [u8; 2],

    /// Type of each element, e.g. Float32, SInt16, UInt8
    ele_type: BufferElementType,
}

//ip Display for VertexDesc
impl std::fmt::Debug for VertexDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.dims[0] == 0 {
            write!(
                fmt,
                "VertexDesc{{{:?}: {:?} @ {}}}",
                self.attr, self.ele_type, self.byte_offset
            )
        } else if self.dims[1] == 0 {
            write!(
                fmt,
                "VertexDesc{{{:?}: {:?}[{}] @ {}}}",
                self.attr, self.ele_type, self.dims[0], self.byte_offset
            )
        } else {
            write!(
                fmt,
                "VertexDesc{{{:?}: {:?}[{}, {}] @ {}}}",
                self.attr, self.ele_type, self.dims[0], self.dims[1], self.byte_offset
            )
        }
    }
}

//ip VertexDesc
impl VertexDesc {
    //cp scalar
    /// Create a scalar [VertexDescr] for a [VertexAttr] of a given
    /// [BufferElementType], at an offset within its parent data.
    pub fn scalar(attr: VertexAttr, ele_type: BufferElementType, byte_offset: u16) -> Self {
        Self {
            attr,
            byte_offset,
            dims: [0, 0],
            ele_type,
        }
    }

    //cp vec
    /// Create a vector of 'len' [VertexDescr] for a [VertexAttr] of a given
    /// [BufferElementType], at an offset within its parent data.
    pub fn vec(attr: VertexAttr, ele_type: BufferElementType, len: u8, byte_offset: u16) -> Self {
        Self {
            attr,
            byte_offset,
            dims: [len, 0],
            ele_type,
        }
    }

    //cp mat
    /// Create a matrix of 'dims' [VertexDescr] for a [VertexAttr] of a given
    /// [BufferElementType], at an offset within its parent data.
    pub fn mat(
        attr: VertexAttr,
        ele_type: BufferElementType,
        dims: [u8; 2],
        byte_offset: u16,
    ) -> Self {
        Self {
            attr,
            byte_offset,
            dims,
            ele_type,
        }
    }

    //ap vertex_attr
    /// Retrieve the vertex attribute this field is for
    #[inline]
    pub fn vertex_attr(&self) -> VertexAttr {
        self.attr
    }

    //ap byte_offset
    /// Retrieve the byte_offset within the parent structure for this field
    #[inline]
    pub fn byte_offset(&self) -> u16 {
        self.byte_offset
    }

    //ap dims
    /// Retrieve the dimensions of this field - if scalar, for example, this is [0,0]
    #[inline]
    pub fn dims(&self) -> [u8; 2] {
        self.dims
    }

    //ap ele_type
    /// Retrieve the [BufferElementType] of the field
    #[inline]
    pub fn ele_type(&self) -> BufferElementType {
        self.ele_type
    }

    //ap count
    /// Get the count of the number of elements in the field
    #[inline]
    pub fn count(&self) -> u32 {
        if self.dims[0] == 0 {
            1
        } else if self.dims[1] == 0 {
            self.dims[0] as u32
        } else {
            (self.dims[0] as u32) * (self.dims[1] as u32)
        }
    }

    //ap byte_length
    /// Get the byte length of the field
    pub fn byte_length(&self) -> u32 {
        self.count() * self.ele_type.byte_length()
    }
}

//a Drawing - VertexAttr, PrimitiveType, MaterialAspect, etc
//tp VertexAttr
/// A [VertexAttr] is a possible vertex attribute that can be used by
/// a renderer; a vertex always has a position attribute, but
/// additional attributes may or maynot be provided by a model
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum VertexAttr {
    /// Position (3xf32) of the point
    Position,
    /// Normal (3xf32) at the point
    Normal,
    /// Color at the point (4xf32)
    Color,
    /// Tangent at the point (4xf32?)
    Tangent,
    /// A set of joints (n x int)
    Joints,
    /// Weights (n x f16?) to apply to each bone\[joint\[i\]\]
    Weights,
    /// Texture coordinates (2 x f32)
    TexCoords0,
    /// Texture coordinates (2 x f32)
    TexCoords1,
    /// Texture coordinates (2 x f32)
    TexCoords2,
}

//tp PrimitiveType
/// Type of a primitive
///
/// This is set to match the GLTF
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrimitiveType {
    /// Points (of an indeterminate size?)
    Points,
    /// Lines (n-1) (ab, cd, ef, ...)
    Lines,
    /// Close loop of (n) lines (ab, cd, ef, ..., za)
    LineLoop,
    /// Connected (n-1) lines (ab, bc, cd, de, ...)
    LineStrip,
    /// Individual (n/3) triangles (one for every three indices)
    #[default]
    Triangles,
    /// Strip of (n-2) triangles (abc, bcd, cde, def, ...)
    TriangleStrip,
    /// Fan of (n-2) triangles (abc, acd, ade, aef, ...)
    TriangleFan,
}

//tp MaterialAspect
/// The aspect of a material
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialAspect {
    /// Color (notionally RGBA as 4xf32)
    Color,
    /// Normal
    Normal,
    /// MetallicRoughness (notionally MR as 2xf32)
    MetallicRoughness,
    /// Occlusion (as f32)
    Occlusion,
    /// Emission (as f32)
    Emission,
}

//tp ShortIndex
/// An optional index used within the model system, that is up to 65000
///
/// It can be, effectively, 'None' or Some(usize less than 65000)
///
/// The purpose is to keep the size of indexed structures small and
/// permit the optional aspect; it is used to index Vec of textures,
/// vertices descriptor sets, etc
///
/// It has implementations of From<> to map a [usize] into a
/// [ShortIndex], and to map from [ShortIndex] to Option<usize>; plus
/// to map from Option<usize> (or anything that is Into<usize>) to a
/// ShortIndex, to ease use.
///
/// These extra implementations remove some of the type safety one
/// might have, but make it simpler to use the index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortIndex(u16);

//ip Default for ShortIndex
impl std::default::Default for ShortIndex {
    fn default() -> Self {
        Self(65535)
    }
}

//ip ShortIndex
impl ShortIndex {
    ///cp none
    /// Create a 'None' value
    #[inline]
    pub fn none() -> Self {
        Default::default()
    }

    ///ap as_usize
    /// Return the value - if it is effectively None, then panic
    #[inline]
    pub fn as_usize(self) -> usize {
        assert!(self.0 != 65535);
        self.0 as usize
    }

    ///ap is_none
    /// Return true if the index is None
    #[inline]
    pub fn is_none(self) -> bool {
        self.0 == 65535
    }

    ///ap is_some
    /// Return true if the index is not None
    #[inline]
    pub fn is_some(self) -> bool {
        self.0 != 65535
    }
}

//ip From<usize> for ShortIndex
impl From<usize> for ShortIndex {
    fn from(index: usize) -> Self {
        assert!(index < 65535);
        Self(index as u16)
    }
}

//ip From<ShortIndex> for Option<usize>
impl From<ShortIndex> for Option<usize> {
    fn from(index: ShortIndex) -> Option<usize> {
        if index.is_none() {
            None
        } else {
            Some(index.as_usize())
        }
    }
}

//ip From<Option<into usize >> for ShortIndex
impl<I: Into<usize>> From<Option<I>> for ShortIndex {
    fn from(opt_index: Option<I>) -> Self {
        if let Some(index) = opt_index {
            let index: usize = index.into();
            assert!(index < 65535);
            Self(index as u16)
        } else {
            Self(65535_u16)
        }
    }
}
