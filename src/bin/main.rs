#![allow(dead_code)]

use game_engine::{math::*, renderer::*, scene::*, *};

fn main() {
    let mut renderer = Window::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    let shader = renderer
        .create_shader(
            include_str!("./basic.vert.glsl"),
            include_str!("./basic.frag.glsl"),
        )
        .unwrap();

    struct Vertex {
        position: Vector2<f32>,
        color: Vector3<f32>,
    }
    let vertices: &[Vertex] = &[
        Vertex {
            position: (0.0, 0.5).into(),
            color: (1.0, 0.0, 0.0).into(),
        },
        Vertex {
            position: (0.5, -0.5).into(),
            color: (0.0, 0.0, 1.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5).into(),
            color: (0.0, 1.0, 0.0).into(),
        },
    ];
    let vertex_buffer = renderer.create_vertex_buffer(
        &[VertexBufferElement::Float2, VertexBufferElement::Float3],
        slice_to_bytes(vertices),
    );

    let mut camera = Camera {
        transform: Transform::default(),
        projection_type: {
            let (width, height) = renderer.get_window().get_size().into();
            CameraProjectionType::Orthographic {
                left: -(width as f32 / height as f32),
                right: (width as f32 / height as f32),
                top: 1.0,
                bottom: -1.0,
                near: -1.0,
                far: 1.0,
            }
        },
    };

    renderer.get_window_mut().show();
    'main_loop: loop {
        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(size) => {
                    renderer.resize(size);
                    camera.projection_type = {
                        let (width, height) = renderer.get_window().get_size().into();
                        CameraProjectionType::Orthographic {
                            left: -(width as f32 / height as f32),
                            right: (width as f32 / height as f32),
                            top: 1.0,
                            bottom: -1.0,
                            near: -1.0,
                            far: 1.0,
                        }
                    }
                }
            }
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        {
            let mut draw_context = renderer.drawing_context(camera);
            draw_context.draw(shader, vertex_buffer, Matrix::identity());
        }
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
