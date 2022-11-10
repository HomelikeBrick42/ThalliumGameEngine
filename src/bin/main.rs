use std::{mem::size_of, mem::size_of_val};

use game_engine::*;

fn main() {
    let mut renderer = Window::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    let _shader = renderer.create_shader(
        include_str!("./basic.vert.glsl"),
        include_str!("./basic.frag.glsl"),
    );

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
    }

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
