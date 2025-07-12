//a Imports
use std::cell::RefCell;

use crate::{ByteBuffer, Renderable};

//a BufferData
//tp BufferData
/// A data buffer for use with vertex data. It may be indices
/// or vertex coordinates etc.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  a GPU `BufferView` on the data is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// The data buffer may, indeed, contain data for more than one object
/// - and the objects may have different data per vertex.
///
/// A data buffer may then be used by many GPU `BufferView`s. Each
/// `BufferView` may be used by many primitives for a single model;
/// alternatively, primitives may have their own individual
/// `BufferViews`.
///
/// To allow a [Renderable] to use the [BufferData] for multiple
/// views, it supports a 'client' field that can be initialized using
/// the 'init_buffer_data_client' method of the Renderable, and then
/// borrowed as required during render programming.
pub struct BufferData<'a, R: Renderable> {
    /// Data buffer itself
    data: &'a [u8],

    ///
    /// byte_offset..(byte_offset+byte_length) is guaranteed to be within the data field
    ///
    /// This value cannot be public without breaking the validity
    byte_offset: u32,

    /// Length of data used in the buffer
    ///
    /// byte_offset..(byte_offset+byte_length) is guaranteed to be within the data field
    ///
    /// This value cannot be public without breaking the validity
    byte_length: u32,

    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferView`s using
    /// it - which then have shared references to the data - but the
    /// client is created afterwards
    rc_client: RefCell<R::Buffer>,
}

//ip Debug for BufferData
impl<'a, R: Renderable> std::fmt::Debug for BufferData<'a, R> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let (data, cont) = {
            if self.data.len() < 8 {
                (self.data, "")
            } else {
                (&self.data[0..8], "...")
            }
        };
        write!(
            fmt,
            "BufferData {{{0:?}{cont}#{4}, byte_offset:{1}, byte_length:{2}, client:{3:?}}}",
            data,
            self.byte_offset,
            self.byte_length,
            self.rc_client,
            self.data.len(),
        )
    }
}

//ip BufferData
impl<'a, R: Renderable> BufferData<'a, R> {
    //ap byte_length
    /// Get the byte length of the [BufferData]
    #[inline]
    pub fn byte_length(&self) -> u32 {
        self.byte_length
    }

    //ap byte_offset
    /// Get the byte offset within the underlying data of the [BufferData]
    #[inline]
    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    //fp new
    /// Create a new [BufferData] given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    pub fn new<B: ByteBuffer + ?Sized>(data: &'a B, byte_offset: u32, byte_length: u32) -> Self {
        let byte_length = {
            if byte_length == 0 {
                (data.byte_length() as u32) - byte_offset
            } else {
                byte_length
            }
        };
        let rc_client = RefCell::new(R::Buffer::default());
        let data = data.borrow_bytes();
        assert!(
            byte_offset + byte_length <= data.len() as u32,
            "Buffer is not large enough for data {} + #{} [ got {}]",
            byte_offset,
            byte_length,
            data.len()
        );
        Self {
            data,
            byte_offset,
            byte_length,
            rc_client,
        }
    }

    //mp create_client
    /// Replace the client data with one of this data
    pub fn create_client(&self, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_buffer_data_client(self.rc_client.borrow_mut().deref_mut(), self);
    }

    //ap borrow_client
    /// Borrow the client immutably
    pub fn borrow_client(&self) -> std::cell::Ref<R::Buffer> {
        self.rc_client.borrow()
    }

    //zz All done
}

//ip AsRef<[u8]> for BufferData
impl<'a, R> AsRef<[u8]> for BufferData<'a, R>
where
    R: Renderable,
{
    fn as_ref(&self) -> &[u8] {
        let start = self.byte_offset as usize;
        let end = (self.byte_offset + self.byte_length) as usize;
        &self.data[start..end]
    }
}

//ip Display for BufferData
impl<'a, R: Renderable + ?Sized> std::fmt::Display for BufferData<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let data_ptr = self.data.as_ptr();
        write!(
            f,
            "BufferData[{:?}+{}#{}]:GL({})",
            data_ptr,
            self.byte_offset,
            self.byte_length,
            self.rc_client.borrow()
        )
    }
}

//ip DefaultIndentedDisplay for BufferData
impl<'a, R: Renderable + ?Sized> indent_display::DefaultIndentedDisplay for BufferData<'a, R> {}
