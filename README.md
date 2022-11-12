# `thallium`

[![Latest Version](https://img.shields.io/crates/v/thallium.svg)](https://crates.io/crates/thallium)
[![Rust Documentation](https://docs.rs/thallium/badge.svg)](https://docs.rs/thallium)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/HomelikeBrick42/thallium/master/LICENSE)

This is a 2d and 3d game engine.
It is sill very early in development so expect changes in the api.

## Example code

How to render a red traingle

```rust
use thallium::{math::*, renderer::*, scene::*, *};

fn main() {
    let mut renderer = Window::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    let shader = renderer.create_shader(
        r"#version 330 core

in vec4 a_Position;

void main() {
    gl_Position = a_Position;
}
",
        r"#version 330 core

out vec4 o_Color;

void main() {
    o_Color = vec4(1.0, 0.0, 0.0, 1.0);
}
",
    ).unwrap();

    let vertices: &[Vector2<f32>] = &[(0.0, 0.5).into(), (0.5, -0.5).into(), (-0.5, -0.5).into()];
    let vertex_buffer = renderer.create_vertex_buffer(
        &[VertexBufferElement::Float2],
        slice_to_bytes(vertices),
    );

    renderer.get_window_mut().show();
    'main_loop: loop {
        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(size) => renderer.resize(size),
                _ => {}
            }
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        {
            let mut draw_context = renderer.drawing_context(Camera::default(), false);            draw_context.draw(shader, vertex_buffer, Matrix4x4::default(), Vector3::zero());
        }
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
```
