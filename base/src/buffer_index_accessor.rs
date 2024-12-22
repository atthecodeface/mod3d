//a Imports
use std::cell::RefCell;

use crate::{BufferData, BufferElementType, Renderable, VertexAttr};

//a BufferIndexAccessor
//tp BufferIndexAccessor
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferIndexAccessor` is used for a single attribute of a set of data, such as
/// Position or Normal.
pub struct BufferIndexAccessor<'a, R: Renderable + ?Sized> {
    /// The `BufferData` that contains the actual index data
    pub data: &'a BufferData<'a, R>,
    /// Number of indices in the buffer
    pub number_indices: u32,
    /// The type of each element
    ///
    /// For indices this must be UInt8, UInt16 or UInt32
    pub ele_type: BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset: u32,
    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferIndexAccessor`s using
    /// it - which then have shared references to the daata - but the
    /// client is created afterwards
    rc_client: RefCell<R::IndexAccessor>,
}

//ip Display for BufferIndexAccessor
impl<'a, R: Renderable> std::fmt::Debug for BufferIndexAccessor<'a, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "BufferIndexAccessor{{ {:?}:{:?} #{}@{}}}",
            self.data,
            self.ele_type,
            self.number_indices,
            self.byte_offset,
            //  self.rc_client
        )
    }
}

//ip BufferIndexAccessor
impl<'a, R: Renderable> BufferIndexAccessor<'a, R> {
    //fp new
    /// Create a new index accessor of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        number_indices: u32,
        ele_type: BufferElementType,
        byte_offset: u32, // offset in bytes from start of data
    ) -> Self {
        let rc_client = RefCell::new(R::IndxAccessor::default());
        Self {
            data,
            number_indices,
            ele_type,
            byte_offset,
            rc_client,
        }
    }

    //mp create_client
    /// Create the render buffer required by the BufferIndexAccessor
    pub fn create_client(&self, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_index_accessor_client(self.rc_client.borrow_mut().deref_mut(), self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::IndexAccessor> {
        self.rc_client.borrow()
    }

    //zz All done
}

//ip Display for BufferIndexAccessor
impl<'a, R: Renderable> std::fmt::Display for BufferIndexAccessor<'a, R> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, fmt)
    }
}

//ip DefaultIndentedDisplay for BufferIndexAccessor
impl<'a, R: Renderable> indent_display::DefaultIndentedDisplay for BufferIndexAccessor<'a, R> {}
