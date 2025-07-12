//a Imports
use std::cell::RefCell;

use crate::{BufferData, Renderable, VertexDesc};

//a BufferDescriptor
//tp BufferDescriptor
/// A descriptor of a subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A [BufferDescriptor] allows portion of a [BufferData] to contain
/// an array of structs with multiple fields for, e.g., Vertex, Normal
/// and Color.
///
/// A [BufferDescriptor] is used within a [crate::BufferDataAccessor]
/// to describe *just* an individual field element.
///
/// TODO: Add a byte_length field
pub struct BufferDescriptor<'a, R: Renderable> {
    /// The `BufferData` that contains the actual vertex attribute data
    data: &'a BufferData<'a, R>,

    /// Byte offset to first data inside 'data'
    byte_offset: u32,

    // Indexed by instance - if true, instance 'n' vertex 'v' use the
    // data from index 'n'; if false then instance 'n' vertex 'v' use
    // the data from index 'v'.
    //
    // index_by_instance: bool,
    /// Stride of data in the buffer
    ///
    /// This is always at least the maximum of the elements[].byte_offset() + byte_length()
    stride: u32,

    /// Description of the layout of the elements of the actual portion of buffer data
    ///
    /// This could become a reference to a struct that is borrowed here, with its own client ref
    elements: Vec<VertexDesc>,

    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferDescriptor] is
    /// created early in the process, prior to any `BufferDataAccessor`s using
    /// it - which then have shared references to the descriptor - but the
    /// client is created afterwards
    rc_client: RefCell<R::Descriptor>,
}

//ip Display for BufferDescriptor
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
    //ap data
    /// Get a reference to the underlying [BufferData]
    pub fn data(&self) -> &BufferData<'a, R> {
        self.data
    }

    //ap byte_offset
    /// Get the byte offset within the underlying [BufferData] for
    /// this descriptor
    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    //ap stride
    /// Get the byte stride between different indices for the instances for
    /// this descriptor
    pub fn stride(&self) -> u32 {
        self.stride
    }

    //ap element
    /// Get a reference to the n'th element
    pub fn element(&self, n: usize) -> &VertexDesc {
        &self.elements[n]
    }

    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        byte_offset: u32,
        mut stride: u32,
        elements: Vec<VertexDesc>,
    ) -> Self {
        let rc_client = RefCell::new(R::Descriptor::default());
        for e in elements.iter() {
            stride = stride.max(e.byte_offset() as u32 + e.byte_length());
        }
        Self {
            data,
            byte_offset,
            stride,
            elements,
            rc_client,
        }
    }

    //mp add_vertex_desc
    pub fn add_vertex_desc(&mut self, vertex_desc: VertexDesc) -> u8 {
        let n = self.elements.len() as u8;
        self.stride = self
            .stride
            .max(vertex_desc.byte_offset() as u32 + vertex_desc.byte_length());
        self.elements.push(vertex_desc);
        n
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
