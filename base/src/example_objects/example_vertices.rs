//a Imports
use std::cell::RefCell;
use std::pin::Pin;

use crate::{
    BufferData, BufferDataAccessor, BufferDescriptor, BufferElementType, BufferIndexAccessor,
    ByteBuffer, Renderable, ShortIndex, VertexDesc, Vertices,
};

//a ExampleBuffers
//tp ExampleBuffers
/// This is a monotonically increasing [Vec] of buffers, which are immutable once added to the struct
///
/// It allows the buffers to be borrowed (immutably) for the lifetime
/// of the structure, even if later more buffers are added to the Vec
pub struct Buffers<'buffers> {
    buffers: RefCell<Vec<Pin<Box<dyn ByteBuffer + 'buffers>>>>,
}

//ip Buffers
impl<'buffers> Buffers<'buffers> {
    //fp new
    /// Create a new empty [Buffers]
    pub fn new() -> Self {
        let buffers = Vec::new().into();
        Self { buffers }
    }

    //mp push
    /// Push a new [ByteBuffer] implementation and return its index
    pub fn push(&self, buffer: Box<dyn ByteBuffer>) -> usize {
        let mut buffers = self.buffers.borrow_mut();
        let n = buffers.len();
        buffers.push(buffer.into());
        n
    }

    //ap Borrow a buffer
    /// Get a reference to one of the buffers held by self;
    pub fn buffer(&self, n: usize) -> &'buffers dyn ByteBuffer {
        let buffers = self.buffers.borrow();
        assert!(n < buffers.len(), "Buffer index out of range");
        let buffer = buffers[n].as_ref();
        // Safety:
        //
        // Extending the lifetime of the reference to the ByteBuffer to
        // that of self is safe as each element of self.buffers is not
        // able to move (they are Pinned) and this reference cannot
        // outlive self.buffers, and the references are guaranteed not to
        // be used during the drop of Self
        unsafe { std::mem::transmute::<&'_ dyn ByteBuffer, &'buffers dyn ByteBuffer>(&*buffer) }
    }
}

//a DataAccessors
//tp DataAccessors
/// This structure helps for objects; the data is
pub struct DataAccessors<'buffers, R: Renderable> {
    data: Vec<Pin<Box<BufferData<'buffers, R>>>>,
    descriptors: Vec<Pin<Box<BufferDescriptor<'buffers, R>>>>,
    index_accessors: Vec<Pin<Box<BufferIndexAccessor<'buffers, R>>>>,
    data_accessors: Vec<Pin<Box<BufferDataAccessor<'buffers, R>>>>,
}

//ip DataAccessors
impl<'buffers, R: Renderable> DataAccessors<'buffers, R> {
    //fp new
    /// Create a new [DataAccessors]
    pub fn new() -> Self {
        let data = Vec::new();
        let index_accessors = Vec::new();
        let descriptors = Vec::new();
        let data_accessors = Vec::new();
        Self {
            data,
            descriptors,
            data_accessors,
            index_accessors,
        }
    }

    //fp push_buffer_data
    /// Push a new [BufferData] that is a portion of a Buffer
    pub fn push_buffer_data(
        &mut self,
        buffers: &Buffers<'buffers>,
        buffer_n: usize,
        byte_offset: u32,
        byte_length: u32,
    ) -> usize {
        let n = self.data.len();
        let b = buffers.buffer(buffer_n);
        let data = Box::new(BufferData::new(b, byte_offset, byte_length));
        self.data.push(data.into());
        n
    }

    //fp push_index_accessor
    /// Create a new [BufferIndexAccessor] on a particular [BufferData] instance that has already been pushed
    pub fn push_index_accessor(
        &mut self,
        data: usize,
        num: u32,
        et: BufferElementType,
        ofs: u32,
    ) -> usize {
        let n = self.index_accessors.len();
        // Safety:
        //
        // Extending the lifetime of the reference to BufferIndexAccessor to
        // that of self is safe as each element of self.descriptors is not
        // able to move (they are Pinned) and this reference cannot
        // outlive self.index_accessors, and the references are guaranteed not to
        // be used during the drop of Self
        let d = unsafe {
            std::mem::transmute::<&BufferData<'_, R>, &'buffers BufferData<'buffers, R>>(
                &self.data[data],
            )
        };
        let accessor = Box::new(BufferIndexAccessor::new(d, num, et, ofs));
        self.index_accessors.push(accessor.into());
        n
    }

    //fp push_descriptor
    /// Create a new [BufferDescriptor<] on a particular [BufferData]
    /// instance that has already been pushed
    pub fn push_descriptor(
        &mut self,
        data: usize,
        byte_offset: u32,
        mut byte_length: u32,
        stride: u32,
    ) -> usize {
        let n = self.data_accessors.len();
        // Safety:
        //
        // Extending the lifetime of the reference to BufferData to
        // that of self is safe as each element of self.data is not
        // able to move (they are Pinned) and this reference cannot
        // outlive self.data, and the references are guaranteed not to
        // be used during the drop of Self
        let d = unsafe {
            std::mem::transmute::<&BufferData<'_, R>, &'buffers BufferData<'buffers, R>>(
                &self.data[data],
            )
        };
        if byte_length == 0 {
            byte_length = d.byte_length() - byte_offset;
        }
        let desc = Box::new(BufferDescriptor::new(
            d,
            byte_offset,
            byte_length,
            stride,
            vec![],
        ));
        self.descriptors.push(desc.into());
        n
    }

    //fp push_data_accessor
    /// Create a new [BufferAccessor] on a particular [BufferData] instance that has already been pushed
    pub fn push_data_accessor(&mut self, desc: usize, vertex_desc: VertexDesc) -> usize
    where
        <R as Renderable>::Descriptor: Unpin,
    {
        let n = self.data_accessors.len();
        let desc_n = self.descriptors[desc].add_vertex_desc(vertex_desc);
        // Safety:
        //
        // Extending the lifetime of the reference to BufferDescriptor to
        // that of self is safe as each element of self.descriptors is not
        // able to move (they are Pinned) and this reference cannot
        // outlive self.descriptors, and the references are guaranteed not to
        // be used during the drop of Self
        let desc = unsafe {
            std::mem::transmute::<&BufferDescriptor<'_, R>, &'buffers BufferDescriptor<'buffers, R>>(
                &self.descriptors[desc],
            )
        };
        let accessor = Box::new(BufferDataAccessor::new(desc, desc_n));
        self.data_accessors.push(accessor.into());
        n
    }

    //ap indices
    /// Get a buffer-lifetime reference to a buffer-lifetime [BufferIndexAccessor]
    pub fn indices(&self, n: Option<usize>) -> Option<&'buffers BufferIndexAccessor<'buffers, R>> {
        if let Some(n) = n {
            let buffer = self.index_accessors[n].as_ref();
            // Safety:
            //
            // Extending the lifetime of the reference to BufferIndexAccessor to
            // that of self is safe as each element of self.index_accessors is not
            // able to move (they are Pinned) and this reference cannot
            // outlive self.index_accessors, and the references are guaranteed not to
            // be used during the drop of Self
            Some(unsafe {
                std::mem::transmute::<
                    &BufferIndexAccessor<'_, R>,
                    &'buffers BufferIndexAccessor<'buffers, R>,
                >(&*buffer)
            })
        } else {
            None
        }
    }
    //ap data_accessor
    /// Get a buffer-lifetime reference to a buffer-lifetime [BufferDataAccessor]
    pub fn data_accessor(&self, n: usize) -> &'buffers BufferDataAccessor<'buffers, R> {
        assert!(
            n < self.data_accessors.len(),
            "Data accessor index out of range"
        );
        let buffer = self.data_accessors[n].as_ref();
        // Safety:
        //
        // Extending the lifetime of the reference to BufferDataAccessor to
        // that of self is safe as each element of self.data_accessors is not
        // able to move (they are Pinned) and this reference cannot
        // outlive self.data_accessors, and the references are guaranteed not to
        // be used during the drop of Self
        unsafe {
            std::mem::transmute::<
                &BufferDataAccessor<'_, R>,
                &'buffers BufferDataAccessor<'buffers, R>,
            >(&*buffer)
        }
    }
}

//a ExampleVertices
//tp ExampleVertices
/// This structure provides for creating example objects, particularly with regard to their vertices
///
/// It uses arrays of [Pin]ned data structures so that the data can be safely self-referential
pub struct ExampleVertices<'buffers, R: Renderable> {
    buffers: Buffers<'buffers>,
    accessors: DataAccessors<'buffers, R>,
    vertices: Vec<Vertices<'buffers, R>>,
}

//ip Default for ExampleVertices
impl<'a, R: Renderable> Default for ExampleVertices<'a, R> {
    fn default() -> Self {
        Self::new()
    }
}

//ip ExampleVertices
impl<'a, R: Renderable> ExampleVertices<'a, R> {
    //fp new
    /// Create a new [ExampleVertices]
    ///
    /// This should probably not be Pin<Box<>>
    pub fn new() -> Self {
        let buffers = Buffers::new();
        let accessors = DataAccessors::new();
        let vertices = Vec::new();
        Self {
            buffers,
            accessors,
            vertices,
        }
    }

    //fp push_byte_buffer
    /// Push a new [ByteBuffer] implementation and return its index
    pub fn push_byte_buffer(&mut self, buffer: Box<dyn ByteBuffer>) -> usize {
        let buffer_n = self.buffers.push(buffer);
        self.accessors
            .push_buffer_data(&self.buffers, buffer_n, 0, 0)
    }

    //fp push_index_accessor
    /// Create a new [BufferAccessor] on a particular [ByteBuffer] instance that has already been pushed
    pub fn push_index_accessor(
        &mut self,
        data: usize,
        num: u32,
        et: BufferElementType,
        ofs: u32,
    ) -> usize {
        self.accessors.push_index_accessor(data, num, et, ofs)
    }

    //fp push_descriptor
    /// Create a new [BufferDescriptor<] on a particular [BufferData]
    /// instance that has already been pushed
    pub fn push_descriptor(
        &mut self,
        data: usize,
        byte_offset: u32,
        byte_length: u32,
        stride: u32,
    ) -> usize {
        self.accessors
            .push_descriptor(data, byte_offset, byte_length, stride)
    }

    //fp push_data_accessor
    /// Create a new [BufferAccessor] on a particular [ByteBuffer] instance that has already been pushed
    pub fn push_data_accessor(&mut self, desc: usize, vertex_desc: VertexDesc) -> usize
    where
        <R as Renderable>::Descriptor: Unpin,
    {
        self.accessors.push_data_accessor(desc, vertex_desc)
    }

    //fp push_vertices
    /// Create a new [Vertices] using a set of indices and positions
    ///
    /// This extends the life of the BufferAccessor to that of the ExampleVertices
    ///
    /// This is safe as the BufferAccessor's are in the Vec for ExampleVertices
    pub fn push_vertices(
        &mut self,
        indices: Option<usize>,
        positions: usize,
        attrs: &[usize],
    ) -> ShortIndex {
        let n = self.vertices.len();
        let i = self.accessors.indices(indices);
        let v = self.accessors.data_accessor(positions);
        let mut vertices = Vertices::new(i, v);
        for view_id in attrs {
            let v = self.accessors.data_accessor(*view_id);
            vertices.add_attr(v);
        }
        self.vertices.push(vertices);
        n.into()
    }

    //fp borrow_vertices
    /// Borrow a set of vertices; this would allow (if mut!) the vertices to have attributes added
    pub fn borrow_vertices(&self, vertices: ShortIndex) -> &Vertices<R> {
        &self.vertices[vertices.as_usize()]
    }
}
