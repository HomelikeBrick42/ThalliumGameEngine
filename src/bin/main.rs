use std::{mem::size_of, mem::size_of_val};

use game_engine::*;
use gl::types::{GLenum, GLuint};

fn main() {
    let mut renderer = Window::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    unsafe {
        unsafe fn compile_shader(typ: GLenum, source: &str) -> GLuint {
            let shader = gl::CreateShader(typ);
            let ptr = source.as_ptr();
            let length = source.len() as i32;
            gl::ShaderSource(shader, 1, &ptr as *const _ as _, &length);
            gl::CompileShader(shader);
            shader
        }

        let vertex_shader = compile_shader(
            gl::VERTEX_SHADER,
            r##"#version 440 core

layout(location = 0) in vec4 a_Position;

void main() {
    gl_Position = a_Position;
}
"##,
        );
        let fragment_shader = compile_shader(
            gl::FRAGMENT_SHADER,
            r##"#version 400 core

layout(location = 0) out vec4 o_Color;

void main() {
    o_Color = vec4(1.0);
}
"##,
        );

        let shader = gl::CreateProgram();
        gl::AttachShader(shader, vertex_shader);
        gl::AttachShader(shader, fragment_shader);
        gl::LinkProgram(shader);
        gl::UseProgram(shader);
    }

    unsafe {
        let mut vertex_array = 0;
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);
    }

    unsafe {
        let mut vertex_buffer = 0;
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        let vertices = [0.0_f32, 0.5_f32, 0.5_f32, -0.5_f32, -0.5_f32, -0.5_f32];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of_val(&vertices[0])) as _,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    };

    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            false as _,
            (2 * size_of::<f32>()) as _,
            std::ptr::null(),
        );
    }

    renderer.get_window_mut().show();
    'main_loop: loop {
        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(size) => renderer.resize(size),
            }
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
