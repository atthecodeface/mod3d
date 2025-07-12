//a Imports
use std::cell::RefCell;

use crate::{BufferData, BufferDescriptor, BufferElementType, Renderable, VertexAttr, VertexDesc};

//a BufferDataAccessor
//tp BufferDataAccessor
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferDataAccessor` is used for a single attribute of a set of data, such as
/// Position or Normal.
///
/// FIXME - change to using borrowed BufferDescriptor...
pub struct BufferDataAccessor<'a, R: Renderable> {
    /// The `BufferData` that contains the actual vertex attribute data
    desc: BufferDescriptor<'a, R>,

    /// Element index in [BufferDescriptor]
    desc_index: u8,

    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferDataAccessor`s using
    /// it - which then have shared references to the daata - but the
    /// client is created afterwards
    rc_client: RefCell<R::DataAccessor>,
}

//ip Display for Object
impl<'a, R: Renderable> std::fmt::Debug for BufferDataAccessor<'a, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "BufferDataAccessor{{{:?}}}",
            self.desc.element(self.desc_index as usize),
        )
    }
}

//ip BufferDataAccessor
impl<'a, R: Renderable> BufferDataAccessor<'a, R> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        count: u32, // count is number of ele_type in an attribute
        ele_type: BufferElementType,
        byte_offset: u32, // offset in bytes from start of data
        stride: u32,      /* stride between elements
                           * (0->count*sizeof(ele_type)) */
    ) -> Self {
        let rc_client = RefCell::new(R::DataAccessor::default());
        let desc = BufferDescriptor::new(
            data,
            byte_offset,
            stride,
            vec![VertexDesc::vec(
                VertexAttr::Position,
                ele_type,
                count as u8,
                0,
            )],
        );
        Self {
            desc,
            desc_index: 0,
            rc_client,
        }
    }

    //mp create_client
    /// Create the render buffer required by the BufferDataAccessor
    pub fn create_client(&self, attr: VertexAttr, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_buffer_view_client(self.rc_client.borrow_mut().deref_mut(), self, attr);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::DataAccessor> {
        self.rc_client.borrow()
    }

    //ap desc
    /// desc
    pub fn desc(&self) -> &BufferDescriptor<'a, R> {
        &self.desc
    }

    //ap vertex_desc
    /// Retrieve the vertex attribute this field is for
    #[inline]
    pub fn vertex_desc(&self) -> &VertexDesc {
        self.desc.element(self.desc_index as usize)
    }

    //ap vertex_attr
    /// Retrieve the vertex attribute this field is for
    #[inline]
    pub fn vertex_attr(&self) -> VertexAttr {
        self.vertex_desc().vertex_attr()
    }

    //ap byte_offset
    /// Retrieve the byte_offset within the [BufferData] for this field
    #[inline]
    pub fn byte_offset(&self) -> u32 {
        self.vertex_desc().byte_offset() as u32 + self.desc.byte_offset()
    }

    //ap ele_type
    /// Retrieve the [BufferElementType] of the field
    #[inline]
    pub fn ele_type(&self) -> BufferElementType {
        self.vertex_desc().ele_type()
    }

    //ap count
    /// Get the count of the number of elements in the field
    #[inline]
    pub fn count(&self) -> u32 {
        self.vertex_desc().count()
    }

    //ap byte_length
    /// Get the byte length of the field
    pub fn byte_length(&self) -> u32 {
        self.vertex_desc().byte_length()
    }

    //zz All done
}

//ip Display for BufferDataAccessor
impl<'a, R: Renderable> std::fmt::Display for BufferDataAccessor<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

//ip DefaultIndentedDisplay for BufferDataAccessor
impl<'a, R: Renderable> indent_display::DefaultIndentedDisplay for BufferDataAccessor<'a, R> {}
