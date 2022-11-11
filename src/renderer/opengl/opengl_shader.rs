use std::{marker::PhantomData, sync::atomic::AtomicUsize};

use gl::types::{GLenum, GLuint};

use crate::{
    renderer::{Shader, ShaderID},
    PhantomUnsend, PhantomUnsync,
};

pub(crate) struct OpenGLShader {
    id: ShaderID,
    opengl_id: GLuint,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLShader {
    pub(crate) fn new(
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<OpenGLShader, String> {
        unsafe fn compile_shader(typ: GLenum, source: &str) -> Result<GLuint, String> {
            let shader = gl::CreateShader(typ);
            let ptr = source.as_ptr();
            let length = source.len() as i32;
            gl::ShaderSource(shader, 1, &ptr as *const _ as _, &length);
            gl::CompileShader(shader);

            let mut compiled = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compiled);
            if compiled == 0 {
                let mut length = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
                let mut vec = vec![0u8; length as usize];

                let mut actual_length = 0;
                gl::GetShaderInfoLog(shader, length, &mut actual_length, vec.as_mut_ptr().cast());
                let info_log = std::str::from_utf8(&vec.as_mut_slice()[..actual_length as usize])
                    .unwrap()
                    .to_string();

                gl::DeleteShader(shader);

                Err(info_log)
            } else {
                Ok(shader)
            }
        }

        unsafe {
            let vertex_shader = compile_shader(gl::VERTEX_SHADER, vertex_shader_source)?;
            let fragment_shader = match compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source)
            {
                Ok(fragment_shader) => fragment_shader,
                Err(message) => {
                    gl::DeleteShader(vertex_shader);
                    return Err(message);
                }
            };

            let shader = gl::CreateProgram();
            gl::AttachShader(shader, vertex_shader);
            gl::AttachShader(shader, fragment_shader);
            gl::LinkProgram(shader);

            let mut linked = 0;
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut linked);
            if linked == 0 {
                let mut length = 0;
                gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut length);
                let mut vec = vec![0u8; length as usize];

                let mut actual_length = 0;
                gl::GetProgramInfoLog(shader, length, &mut actual_length, vec.as_mut_ptr().cast());
                let info_log = std::str::from_utf8(&vec.as_mut_slice()[..actual_length as usize])
                    .unwrap()
                    .to_string();

                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
                gl::DeleteProgram(shader);

                return Err(info_log);
            }

            gl::DetachShader(shader, vertex_shader);
            gl::DetachShader(shader, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(OpenGLShader {
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
            })
        }
    }

    pub(crate) fn bind(&mut self) {
        unsafe { gl::UseProgram(self.opengl_id) }
    }

    pub(crate) fn unbind(&mut self) {
        unsafe { gl::UseProgram(0) }
    }
}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.opengl_id);
        }
    }
}

impl Shader for OpenGLShader {
    fn get_id(&self) -> ShaderID {
        self.id
    }
}
