use std::{marker::PhantomData, mem::size_of, sync::atomic::AtomicUsize};

use gl::types::GLuint;

use crate::{PhantomUnsend, PhantomUnsync, VertexBuffer, VertexBufferElement, VertexBufferID};

pub(crate) struct OpenGLVertexBuffer {
    id: VertexBufferID,
    opengl_vertex_array_id: GLuint,
    opengl_id: GLuint,
    count: usize,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLVertexBuffer {
    pub(crate) fn new(
        vertex_layout: &[VertexBufferElement],
        data: Option<&[u8]>,
    ) -> OpenGLVertexBuffer {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);

            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            if let Some(data) = data {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    data.len() as _,
                    data.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );
            }

            fn get_element_size(element: &VertexBufferElement) -> usize {
                match element {
                    VertexBufferElement::Float => 1 * size_of::<f32>(),
                    VertexBufferElement::Float2 => 2 * size_of::<f32>(),
                    VertexBufferElement::Float3 => 3 * size_of::<f32>(),
                    VertexBufferElement::Float4 => 4 * size_of::<f32>(),
                }
            }

            let stride: usize = vertex_layout.iter().map(get_element_size).sum();

            let mut offset = 0;
            for (i, element) in vertex_layout.iter().enumerate() {
                gl::EnableVertexAttribArray(i as _);
                gl::VertexAttribPointer(
                    i as _,
                    match element {
                        VertexBufferElement::Float => 1,
                        VertexBufferElement::Float2 => 2,
                        VertexBufferElement::Float3 => 3,
                        VertexBufferElement::Float4 => 4,
                    },
                    match element {
                        VertexBufferElement::Float
                        | VertexBufferElement::Float2
                        | VertexBufferElement::Float3
                        | VertexBufferElement::Float4 => gl::FLOAT,
                    },
                    false as _,
                    stride as _,
                    offset as _,
                );
                offset += get_element_size(element);
            }

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            OpenGLVertexBuffer {
                id: {
                    static ID: AtomicUsize = AtomicUsize::new(1);
                    VertexBufferID(
                        ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
                        PhantomData,
                        PhantomData,
                    )
                },
                opengl_vertex_array_id: vertex_array,
                opengl_id: vertex_buffer,
                count: data.map(|data| data.len()).unwrap_or(0) / stride,
                _send: PhantomData,
                _sync: PhantomData,
            }
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.opengl_vertex_array_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.opengl_id);
        }
    }

    pub(crate) fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for OpenGLVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.opengl_id);
            gl::DeleteVertexArrays(1, &self.opengl_vertex_array_id);
        }
    }
}

impl VertexBuffer for OpenGLVertexBuffer {
    fn get_id(&self) -> VertexBufferID {
        self.id
    }

    fn get_count(&self) -> usize {
        self.count
    }
}
