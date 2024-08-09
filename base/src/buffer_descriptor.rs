//a Imports
use std::cell::RefCell;

use crate::{BufferData, BufferElementType, Renderable, VertexAttr, VertexDesc};

//a BufferDescriptor
//tp BufferDescriptor
/// A desccriptor of a subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferDescriptor` is used for a single attribute of a set of data, such as
/// Position or Normal.
pub struct BufferDescriptor<'a, R: Renderable + ?Sized> {
    /// The `BufferData` that contains the actual vertex attribute data
    pub data: &'a BufferData<'a, R>,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    /// Unused for indices
    pub stride: u32,
    /// Byte offset to first data inside 'data'
    pub byte_offset: u32,
    /// Description of the layout of the elements of the actual portion of buffer data
    ///
    /// This could become a reference to a struct that is borrowed here, with its own client ref
    pub elements: Vec<VertexDesc>,
    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferDescriptor`s using
    /// it - which then have shared references to the daata - but the
    /// client is created afterwards
    rc_client: RefCell<R::Descriptor>,
}

//ip Display for Object
impl<'a, R: Renderable> std::fmt::Debug for BufferDescriptor<'a, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "BufferDescriptor{{ {:?} @{}+*{}}}",
            self.data,
            self.byte_offset,
            self.stride,
            // self.ele_type,
            // self.elements_per_data,
            //  self.rc_client
        )
    }
}

//ip BufferDescriptor
impl<'a, R: Renderable> BufferDescriptor<'a, R> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        byte_offset: u32, // offset in bytes from start of data
        stride: u32,      /* stride between elements
                           * (0->count*sizeof(ele_type)) */
        elements: Vec<VertexDesc>,
    ) -> Self {
        let rc_client = RefCell::new(R::Descriptor::default());
        Self {
            data,
            byte_offset,
            stride,
            elements,
            rc_client,
        }
    }

    //mp create_client
    /// Create the render buffer required by the BufferDescriptor
    pub fn create_client(&self, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_buffer_desc_client(self.rc_client.borrow_mut().deref_mut(), self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::Descriptor> {
        self.rc_client.borrow()
    }

    //zz All done
}

//ip Display for BufferDescriptor
impl<'a, R: Renderable> std::fmt::Display for BufferDescriptor<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, f)
    }
}

//ip DefaultIndentedDisplay for BufferDescriptor
impl<'a, R: Renderable> indent_display::DefaultIndentedDisplay for BufferDescriptor<'a, R> {}
