use std::{marker::PhantomData, mem::size_of, sync::atomic::AtomicUsize};

use gl::types::GLuint;

use crate::{
    renderer::{IndexBuffer, IndexBufferID},
    PhantomUnsend, PhantomUnsync,
};

pub(crate) struct OpenGLIndexBuffer {
    id: IndexBufferID,
    opengl_id: GLuint,
    count: usize,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLIndexBuffer {
    pub(crate) fn new(indices: &[u32]) -> OpenGLIndexBuffer {
        unsafe {
            let mut index_buffer = 0;
            gl::GenBuffers(1, &mut index_buffer);

            let mut index_buffer = OpenGLIndexBuffer {
                id: {
                    static ID: AtomicUsize = AtomicUsize::new(1);
                    IndexBufferID(
                        ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
                        PhantomData,
                        PhantomData,
                    )
                },
                opengl_id: index_buffer,
                count: 0,
                _send: PhantomData,
                _sync: PhantomData,
            };
            index_buffer.set_indices(indices);
            index_buffer
        }
    }

    pub(crate) fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.opengl_id) };
    }

    pub(crate) fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    }
}

impl Drop for OpenGLIndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.opengl_id) };
    }
}

impl IndexBuffer for OpenGLIndexBuffer {
    fn get_id(&self) -> IndexBufferID {
        self.id
    }

    fn get_count(&self) -> usize {
        self.count
    }

    fn set_indices(&mut self, indices: &[u32]) {
        self.bind();
        self.count = indices.len();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as _,
                indices.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            )
        };
        self.unbind();
    }
}
