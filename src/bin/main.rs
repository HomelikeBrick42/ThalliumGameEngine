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
        position: Vector3<f32>,
    }
    let vertices: &[Vertex] = &[
        Vertex {
            position: (-0.5, 0.5, 0.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, 0.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, 0.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, 0.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, 0.0).into(),
        },
    ];
    let vertex_buffer =
        renderer.create_vertex_buffer(&[VertexBufferElement::Float3], slice_to_bytes(vertices));

    let mut camera = Camera {
        transform: Transform::default(),
        projection_type: CameraProjectionType::Perspective {
            fov: 60.0,
            aspect: {
                let (width, height) = renderer.get_window().get_size().into();
                width as f32 / height as f32
            },
            near: 0.001,
            far: 1000.0,
        },
    };

    const FIXED_UPDATE_INTERVAL: f32 = 1.0 / 60.0;

    renderer.get_window_mut().show();
    let mut fixed_update_time = 0.0;
    let mut last_now = std::time::Instant::now();
    'main_loop: loop {
        let now = std::time::Instant::now();
        let ts = now.duration_since(last_now).as_secs_f32();
        last_now = now;

        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(size) => {
                    renderer.resize(size);
                    let aspect_ratio = size.x as f32 / size.y as f32;
                    match &mut camera.projection_type {
                        CameraProjectionType::None => {}
                        CameraProjectionType::Orthographic {
                            left,
                            right,
                            top,
                            bottom,
                            ..
                        } => {
                            *left = -aspect_ratio * *bottom;
                            *right = aspect_ratio * *top;
                        }
                        CameraProjectionType::Perspective { aspect, .. } => *aspect = aspect_ratio,
                    }
                }
                WindowEvent::KeyPressed(_) => {}
                WindowEvent::KeyReleased(_) => {}
            }
        }

        fixed_update_time += ts;
        while fixed_update_time > FIXED_UPDATE_INTERVAL {
            fixed_update_time -= FIXED_UPDATE_INTERVAL;
            let ts = FIXED_UPDATE_INTERVAL;

            let window = renderer.get_window();
            if window.get_key_state(Keycode::W) {
                camera.transform.position.z += 2.0 * ts;
            }
            if window.get_key_state(Keycode::S) {
                camera.transform.position.z -= 2.0 * ts;
            }
            if window.get_key_state(Keycode::A) {
                camera.transform.position.x -= 2.0 * ts;
            }
            if window.get_key_state(Keycode::D) {
                camera.transform.position.x += 2.0 * ts;
            }
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        {
            let mut draw_context = renderer.drawing_context(camera);
            draw_context.draw(
                shader,
                vertex_buffer,
                Transform {
                    position: (0.0, 0.0, 3.0).into(),
                    ..Default::default()
                }
                .into(),
                (0.8, 0.2, 0.1).into(),
            );
        }
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
