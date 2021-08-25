//! At the time, this is set up so that a VertexArray generally
//! has one fixed vertex buffer, and one fixed index buffer, and
//! fixed attribute layout.
//!
//! If would be sensible to at least separate the index buffer,
//! so the parts of the same mesh can be rendered in separate
//! draw calls by changing index buffer.

use std::os::raw::c_void;
use std::mem;
use gl::types::*;

pub trait Vertex {
    unsafe fn define_vertex_attrib_pointers();
}

/// Abstract vertex buffer object for generic vertex type.
/// Encapsulates generating and deleting VBOs and uploading
/// vertex data to the GPU.
pub struct VertexArray<V: Vertex> {
    vao: GLuint,
    vbo: GLuint,
    ibo: GLuint,
    marker: std::marker::PhantomData<V>
}

impl<V: Vertex> VertexArray<V> {
    pub fn create(
        vertices: impl AsRef<[V]>,
        indices:  impl AsRef<[u32]>
    ) -> VertexArray<V> {
        /* Vertex Array Object. */
        let vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        };

        /* Vertex buffer. */
        let vbo = unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            vbo
        };

        /* Index buffer. */
        let ibo = unsafe {
            let mut ibo = 0;
            gl::GenBuffers(1, &mut ibo);
            ibo
        };

        let vertices = vertices.as_ref();
        let indices = indices.as_ref();

        unsafe {
            /* Bind the relevant buffers. */
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

            /* OpenGL needs the # of bytes in the buffers, and ptrs to them. */
            let vb_n_bytes = vertices.len() * mem::size_of::<V>();
            let vb_data = vertices.as_ptr();

            let ib_n_bytes = indices.len() * mem::size_of::<u32>();
            let ib_data = indices.as_ptr();

            /* Upload the data. */

            gl::BufferData(gl::ARRAY_BUFFER,
                           vb_n_bytes as GLsizeiptr,
                           vb_data    as *const c_void,
                           gl::STATIC_DRAW);

            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           ib_n_bytes as GLsizeiptr,
                           ib_data    as *const c_void,
                           gl::STATIC_DRAW);

            /* Vertex attribute pointers.
             * This associates the vertex buffer with this vertex array object.
             * Later, we need only to bind the vertex array and the index buffer.
             */
            V::define_vertex_attrib_pointers();

            /* Unbind. */
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        

        VertexArray {
            vao, vbo, ibo, marker: std::marker::PhantomData
        }
    }

    pub unsafe fn select(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
    }

}


impl<V: Vertex> Drop for VertexArray<V> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao as *const GLuint);
            gl::DeleteBuffers(1, &self.vbo as *const GLuint);
            gl::DeleteBuffers(1, &self.ibo as *const GLuint);
        }
    }
}
