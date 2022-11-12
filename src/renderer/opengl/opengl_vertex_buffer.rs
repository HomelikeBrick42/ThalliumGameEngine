use std::{marker::PhantomData, mem::size_of, sync::atomic::AtomicUsize};

use gl::types::GLuint;

use crate::{
    renderer::{VertexBuffer, VertexBufferElement, VertexBufferID},
    PhantomUnsend, PhantomUnsync,
};

pub(crate) struct OpenGLVertexBuffer {
    id: VertexBufferID,
    opengl_vertex_array_id: GLuint,
    opengl_id: GLuint,
    stride: usize,
    count: usize,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLVertexBuffer {
    pub(crate) fn new(layout: &[VertexBufferElement], data: &[u8]) -> OpenGLVertexBuffer {
        let mut vertex_buffer = unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);
            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);

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
                stride: 0,
                count: 0,
                _send: PhantomData,
                _sync: PhantomData,
            }
        };
        vertex_buffer.set_layout(layout, data);
        vertex_buffer
    }

    pub(crate) fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.opengl_vertex_array_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.opengl_id);
        }
    }

    pub(crate) fn unbind(&mut self) {
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

    fn set_layout(&mut self, layout: &[VertexBufferElement], data: &[u8]) {
        fn get_element_size(element: &VertexBufferElement) -> usize {
            match element {
                VertexBufferElement::Float => 1 * size_of::<f32>(),
                VertexBufferElement::Float2 => 2 * size_of::<f32>(),
                VertexBufferElement::Float3 => 3 * size_of::<f32>(),
                VertexBufferElement::Float4 => 4 * size_of::<f32>(),
            }
        }

        self.stride = layout.iter().map(get_element_size).sum();

        self.bind();
        let mut offset = 0;
        for (i, element) in layout.iter().enumerate() {
            unsafe {
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
                    self.stride as _,
                    offset as _,
                );
            }
            offset += get_element_size(&element);
        }
        self.unbind();

        self.set_data(data);
    }

    fn set_data(&mut self, data: &[u8]) {
        if self.stride > 0 {
            assert_eq!(data.len() % self.stride, 0);
            self.count = data.len() / self.stride;
        } else {
            assert_eq!(data.len(), 0);
            self.count = 0;
        }
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.len() as _,
                data.as_ptr().cast(),
                gl::DYNAMIC_DRAW,
            )
        };
        self.unbind();
    }
}
