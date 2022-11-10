use std::{marker::PhantomData, sync::atomic::AtomicUsize};

use gl::types::{GLenum, GLuint};

use crate::{PhantomUnsend, PhantomUnsync, Shader, ShaderID};

pub(crate) struct OpenGLShader {
    id: ShaderID,
    opengl_id: GLuint,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLShader {
    pub(crate) fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> OpenGLShader {
        unsafe fn compile_shader(typ: GLenum, source: &str) -> GLuint {
            let shader = gl::CreateShader(typ);
            let ptr = source.as_ptr();
            let length = source.len() as i32;
            gl::ShaderSource(shader, 1, &ptr as *const _ as _, &length);
            gl::CompileShader(shader);
            shader
        }

        unsafe {
            let vertex_shader = compile_shader(gl::VERTEX_SHADER, vertex_shader_source);
            let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source);

            let shader = gl::CreateProgram();
            gl::AttachShader(shader, vertex_shader);
            gl::AttachShader(shader, fragment_shader);
            gl::LinkProgram(shader);

            gl::DetachShader(shader, vertex_shader);
            gl::DetachShader(shader, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::UseProgram(shader); // TODO: just for testing purposes

            OpenGLShader {
                id: {
                    static ID: AtomicUsize = AtomicUsize::new(1);
                    ShaderID(
                        ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
                        PhantomData,
                        PhantomData,
                    )
                },
                opengl_id: shader,
                _send: PhantomData,
                _sync: PhantomData,
            }
        }
    }
}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.opengl_id);
        }
    }
}

impl Shader for OpenGLShader {
    fn get_id(&self) -> ShaderID {
        self.id
    }
}
