//a Imports
use std::cell::{Ref, RefCell};

use crate::{BufferDataAccessor, BufferIndexAccessor};
use crate::{Renderable, VertexAttr};

//a Vertices
//tp Vertices
/// A set of vertices using one or more [crate::BufferData] through [BufferAccessor]s.
///
/// A number of [Vertices] is used by an `Object`, its components and
/// their meshes; one is used for each primitive within a mesh for its
/// elements.  The actual elements will be sets of triangles (as
/// stripes or whatever) which use these vertices.
///
/// A [Vertices] object includes a lot of options for vertices, and
/// different renderers (or different render stages) may require
/// different subsets of these indices. As such, in OpenGL for
/// example, a [Vertices] object may end up with more than one
/// `VAO`. This data is part of the [VerticesClient] struct associated
/// with the [Vertices]. In WebGPU there may more than one render
/// pipeline for different shader pipelines for the same set of
/// vertices.
///
/// When it comes to creating an instance of a mesh, that instance
/// will have specific transformations and materials for each of its
/// primitives; rendering the instance with a shader will require
/// enabling the [Vertices] client for that shader, setting
/// appropriate render options (uniforms in OpenGL)
#[derive(Debug)]
pub struct Vertices<'vertices, R: Renderable + ?Sized> {
    /// Indices related to primitives that use these vertices; if none
    /// then a draw call is not indexed but uses a range of elements
    indices: Option<&'vertices BufferIndexAccessor<'vertices, R>>,
    /// Attributes of the vertices, which must include position, sorted by VertexAttr
    attrs: Vec<(VertexAttr, &'vertices BufferDataAccessor<'vertices, R>)>,
    /// Client handle for this set of Vertices, updated when 'create_client' is invoked
    rc_client: RefCell<R::Vertices>,
}

//ip Display for Vertices
impl<'vertices, R: Renderable> std::fmt::Display for Vertices<'vertices, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "Vertices:")?;
        writeln!(fmt, "  indices: {:?}", self.indices)?;
        writeln!(fmt, "  position: {:?}", self.position)?;
        for (n, a) in &self.attrs {
            writeln!(fmt, "  {n:?}: {a:?}")?;
        }
        Ok(())
    }
}

///ip Vertices
impl<'vertices, R: Renderable> Vertices<'vertices, R> {
    //fp new
    /// Create a new [Vertices] object with no additional attributes
    pub fn new(
        indices: Option<&'vertices BufferIndexAccessor<'vertices, R>>,
        position: &'vertices BufferDataAccessor<'vertices, R>,
    ) -> Self {
        let attrs = vec![(VertexAttr::Position, position)];
        let rc_client = RefCell::new(R::Vertices::default());
        Self {
            indices,
            attrs,
            rc_client,
        }
    }

    //mp add_attr
    /// Add a [BufferAccessor] for a particular [VertexAttr]
    ///
    /// On creation the [Vertices] will have views for indices and
    /// positions; this provides a means to add views for things such
    /// as normal, tex coords, etc
    pub fn add_attr(
        &mut self,
        attr: VertexAttr,
        accessor: &'vertices BufferDataAccessor<'vertices, R>,
    ) {
        match self.attrs.binary_search_by(|(a, _)| a.cmp(&attr)) {
            Ok(index) => {
                self.attrs[index] = (attr, accessor);
            }
            Err(index) => {
                self.attrs.insert(index, (attr, accessor));
            }
        }
    }

    //mp borrow_indices
    /// Borrow the indices [BufferAccessor]
    pub fn borrow_indices<'a>(&'a self) -> Option<&'a BufferIndexAccessor<'vertices, R>> {
        self.indices
    }

    //mp borrow_attr
    /// Borrow an attribute [BufferAccessor] if the [Vertices] has one
    pub fn borrow_attr<'a>(
        &'a self,
        attr: VertexAttr,
    ) -> Option<&'a BufferDataAccessor<'vertices, R>> {
        for i in 0..self.attrs.len() {
            if self.attrs[i].0 == attr {
                return Some(self.attrs[i].1);
            }
        }
        None
    }

    //mp iter_attrs
    /// Iterate through attributes
    pub fn iter_attrs(&self) -> std::slice::Iter<(VertexAttr, &BufferDataAccessor<'vertices, R>)> {
        self.attrs.iter()
    }

    //mp create_client
    /// Create the render buffer required by the BufferAccessor
    pub fn create_client(&self, renderer: &mut R) {
        self.indices.create_client(renderer);
        for (attr, view) in self.iter_attrs() {
            view.create_client(*attr, renderer);
        }
        *(self.rc_client.borrow_mut()) = renderer.create_vertices_client(self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> Ref<R::Vertices> {
        self.rc_client.borrow()
    }

    //zz All done
}
