//a Imports
use std::marker::PhantomData;

use mod3d_base::{BufferAccessor, BufferElementType, VertexAttr};

use crate::{Gl, GlProgram};

//a VertexBuffer
//tp VertexBuffer
///
/// A subset of a data buffer for use with OpenGL vertex data.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  A [VertexBuffer] is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// OpenGL will have one copy of the data for all the [VertexBuffer]
#[derive(Debug)]
pub struct VertexBuffer<G>
where
    G: Gl,
{
    /// Ref-counted buffer
    gl_buffer: <G as Gl>::Buffer,
    /// For attributes: number of elements per vertex (1 to 4, or 4, 9 or 16)
    /// For indices: number of indices in the buffer
    pub elements_per_data: u32,
    /// The type of each element
    pub ele_type: BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset: u32,
    /// Stride of data in the buffer - 0 for elements_per_data*sizeof(ele_type)
    pub stride: u32,
}

//ip VertexBuffer
impl<G> VertexBuffer<G>
where
    G: Gl,
{
    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> &<G as Gl>::Buffer {
        &self.gl_buffer
    }

    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(&mut self, view: &BufferAccessor<G>, render_context: &mut G) {
        view.data.create_client(render_context);
        self.elements_per_data = view.elements_per_data;
        self.ele_type = view.ele_type;
        self.byte_offset = view.byte_offset;
        self.stride = view.stride;
        self.gl_buffer = view.data.borrow_client().clone();
    }

    //fp bind_to_vao_attr
    /// Bind the buffer as a vertex attribute to the current VAO
    pub fn bind_to_vao_attr(
        &self,
        context: &mut G,
        attr_id: &<<G as Gl>::Program as GlProgram>::GlAttrId,
    ) {
        context.buffer_bind_to_vao_attr(
            &self.gl_buffer,
            attr_id,
            self.elements_per_data,
            self.ele_type,
            self.byte_offset,
            self.stride,
        );
    }

    //zz All done
}

//ip Default for VertexBuffer
impl<G> Default for VertexBuffer<G>
where
    G: Gl,
{
    fn default() -> Self {
        let gl_buffer = <G as Gl>::Buffer::default();
        let elements_per_data = 0;
        let ele_type = BufferElementType::Float32;
        let byte_offset = 0;
        let stride = 0;
        Self {
            gl_buffer,
            elements_per_data,
            ele_type,
            byte_offset,
            stride,
        }
    }
}

//ip Clone for VertexBuffer
impl<G> Clone for VertexBuffer<G>
where
    G: Gl,
{
    fn clone(&self) -> Self {
        let gl_buffer = self.gl_buffer.clone();
        let elements_per_data = self.elements_per_data;
        let ele_type = self.ele_type;
        let byte_offset = self.byte_offset;
        let stride = self.stride;
        Self {
            gl_buffer,
            elements_per_data,
            ele_type,
            byte_offset,
            stride,
        }
    }
}

//ip Display for VertexBuffer
impl<G> std::fmt::Display for VertexBuffer<G>
where
    G: Gl,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Vert({:?}+{}:#{} {:?} @{})",
            self.gl_buffer, self.byte_offset, self.elements_per_data, self.ele_type, self.stride
        )
    }
}

//ip DefaultIndentedDisplay for VertexBuffer
impl<G> indent_display::DefaultIndentedDisplay for VertexBuffer<G> where G: Gl {}

//a IndexBuffer
//tp IndexBuffer
///
/// A subset of a data buffer for use with OpenGL index data.
///
/// An IndexBuffer directly owns the OpenGL buffer which is an
/// ElementArray rather than vertex data
#[derive(Debug)]
pub struct IndexBuffer<G>
where
    G: Gl,
{
    /// Ref-counted buffer
    gl_buffer: <G as Gl>::Buffer,
    /// Number of indices in the buffer
    pub count: u32,
    /// The type of each element
    pub ele_type: BufferElementType,
}

//ip Default for IndexBuffer
impl<G> Default for IndexBuffer<G>
where
    G: Gl,
{
    fn default() -> Self {
        let gl_buffer = <G as Gl>::Buffer::default();
        let count = 0;
        let ele_type = BufferElementType::Int8;
        Self {
            gl_buffer,
            count,
            ele_type,
        }
    }
}

//ip Clone for IndexBuffer
impl<G> Clone for IndexBuffer<G>
where
    G: Gl,
{
    fn clone(&self) -> Self {
        let gl_buffer = self.gl_buffer.clone();
        let count = self.count;
        let ele_type = self.ele_type;
        Self {
            gl_buffer,
            count,
            ele_type,
        }
    }
}

//ip IndexBuffer
impl<G> IndexBuffer<G>
where
    G: Gl,
{
    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(view: &BufferAccessor<G>, render_context: &mut G) -> Self {
        let mut gl_buffer = <G as Gl>::Buffer::default();
        render_context.init_buffer_of_indices(&mut gl_buffer, view);
        let count = view.elements_per_data;
        let ele_type = view.ele_type;
        println!(
            "Create indices buffer {} of view {:?}#{}",
            gl_buffer, view.ele_type, view.elements_per_data
        );
        Self {
            gl_buffer,
            count,
            ele_type,
        }
    }
    //ap gl_buffer
    pub fn gl_buffer(&self) -> &<G as Gl>::Buffer {
        &self.gl_buffer
    }

    //zz All done
}

//ip Display for IndexBuffer
impl<G> std::fmt::Display for IndexBuffer<G>
where
    G: Gl,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Ind({:?}#{} {:?})",
            self.gl_buffer, self.count, self.ele_type,
        )
    }
}

//ip DefaultIndentedDisplay for IndexBuffer
impl<G> indent_display::DefaultIndentedDisplay for IndexBuffer<G> where G: Gl {}

//a BufferView
//tp BufferView
///
/// A view of data with either vertices of indices
#[derive(Debug)]
pub enum BufferView<G>
where
    G: Gl,
{
    /// Vertex buffer
    VertexBuffer(VertexBuffer<G>),
    /// Index buffer
    IndexBuffer(IndexBuffer<G>),
}

//ip Default for BufferView<G>
impl<G> Default for BufferView<G>
where
    G: Gl,
{
    fn default() -> Self {
        Self::VertexBuffer(VertexBuffer::default())
    }
}

//ip Clone for BufferView<G>
impl<G> Clone for BufferView<G>
where
    G: Gl,
{
    fn clone(&self) -> Self {
        use BufferView::*;
        match self {
            VertexBuffer(b) => Self::VertexBuffer(b.clone()),
            IndexBuffer(b) => Self::IndexBuffer(b.clone()),
        }
    }
}

//ip BufferView
impl<G> BufferView<G>
where
    G: Gl,
{
    //fp as_index_buffer
    /// Return the [IndexBuffer] that this [BufferView] is of - if it
    /// is not a view of indices then panic
    pub fn as_index_buffer(&self) -> &IndexBuffer<G> {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer,
            _ => panic!("Attempt to borrow a VertexBuffer as an IndexBuffer"),
        }
    }

    //fp as_vertex_buffer
    /// Return the [VertexBuffer] that this [BufferView] is of - if it
    /// is not a view of vertex attributess then panic
    pub fn as_vertex_buffer(&self) -> &VertexBuffer<G> {
        match self {
            Self::VertexBuffer(vertex_buffer) => vertex_buffer,
            _ => panic!("Attempt to borrow an IndexBuffer as an VertexBuffer"),
        }
    }
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn init_buffer_view_client(
        &mut self,
        view: &BufferAccessor<G>,
        attr: VertexAttr,
        renderer: &mut G,
    ) {
        if attr == VertexAttr::Indices {
            let index_buffer = IndexBuffer::of_view(view, renderer);
            *self = BufferView::IndexBuffer(index_buffer);
        } else {
            match self {
                BufferView::IndexBuffer(_) => panic!("Vertex buffer is already an index buffer"),
                BufferView::VertexBuffer(vb) => {
                    vb.of_view(view, renderer);
                }
            }
        }
    }
}

//ip AccessorClient for BufferView
impl<G> mod3d_base::AccessorClient for BufferView<G> where G: Gl {}

//ip Display for BufferView
impl<G> std::fmt::Display for BufferView<G>
where
    G: Gl,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer.fmt(f),
            Self::VertexBuffer(vertex_buffer) => vertex_buffer.fmt(f),
        }
    }
}

//ip DefaultIndentedDisplay for BufferView
impl<G> indent_display::DefaultIndentedDisplay for BufferView<G> where G: Gl {}

//a UniformBuffer
//tp UniformBuffer
/// Creates a UniformBuffer that may contain the data for a number of program Uniforms
///
/// For simplicity in OpenGl/WebGl this also creates the backing GlBuffer
///
/// A program's uniform is bound to a *range* of one of these
#[derive(Debug)]
pub struct UniformBuffer<G>
where
    G: Gl,
{
    /// Ref-cotunted GPU gl buffer to use
    gl_buffer: <G as Gl>::Buffer,
    /// This is the size of the buffer (so that length=0 can map to the whole buffer)
    byte_length: usize,
    phantom: PhantomData<G>,
}

//ip UniformBuffer
impl<G> UniformBuffer<G>
where
    G: Gl,
{
    //fp of_data
    pub fn of_data<F: Sized>(context: &mut G, data: &[F], is_dynamic: bool) -> Result<Self, ()> {
        G::uniform_buffer_create(context, data, is_dynamic)
    }

    //fp new
    pub fn new(gl_buffer: <G as Gl>::Buffer, byte_length: usize) -> Self {
        Self {
            gl_buffer,
            byte_length,
            phantom: PhantomData,
        }
    }

    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> &<G as Gl>::Buffer {
        &self.gl_buffer
    }

    //ap byte_length
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    //ap offset_and_length
    pub fn offset_and_length(&self, byte_offset: usize, byte_length: usize) -> (usize, usize) {
        if byte_length == 0 {
            (0, self.byte_length)
        } else {
            (byte_offset, byte_length)
        }
    }
}
