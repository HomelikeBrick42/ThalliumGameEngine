#![allow(dead_code)]

use thallium::{math::*, platform::*, renderer::*, scene::*, *};

fn main() {
    let mut renderer = Surface::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    let shader = renderer
        .create_shader(
            include_str!("./basic.vert.glsl"),
            include_str!("./basic.frag.glsl"),
        )
        .unwrap();

    struct Vertex {
        position: Vector3<f32>,
        normal: Vector3<f32>,
        tex_coord: Vector2<f32>,
    }
    let vertices: &[Vertex] = &[
        Vertex {
            position: (-0.5, 0.5, 0.5).into(),
            normal: (0.0, 0.0, 1.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, 0.5).into(),
            normal: (0.0, 0.0, 1.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, 0.5).into(),
            normal: (0.0, 0.0, 1.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, 0.5).into(),
            normal: (0.0, 0.0, 1.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, -0.5).into(),
            normal: (0.0, 0.0, -1.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, -0.5).into(),
            normal: (0.0, 0.0, -1.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, -0.5).into(),
            normal: (0.0, 0.0, -1.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, -0.5).into(),
            normal: (0.0, 0.0, -1.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, -0.5).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, 0.5).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, 0.5).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, -0.5).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, -0.5).into(),
            normal: (1.0, 0.0, 0.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, 0.5).into(),
            normal: (1.0, 0.0, 0.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, 0.5).into(),
            normal: (1.0, 0.0, 0.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, -0.5).into(),
            normal: (1.0, 0.0, 0.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, 0.5).into(),
            normal: (0.0, 1.0, 0.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, 0.5).into(),
            normal: (0.0, 1.0, 0.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, 0.5, -0.5).into(),
            normal: (0.0, 1.0, 0.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, 0.5, -0.5).into(),
            normal: (0.0, 1.0, 0.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, 0.5).into(),
            normal: (0.0, -1.0, 0.0).into(),
            tex_coord: (0.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, 0.5).into(),
            normal: (0.0, -1.0, 0.0).into(),
            tex_coord: (1.0, 1.0).into(),
        },
        Vertex {
            position: (0.5, -0.5, -0.5).into(),
            normal: (0.0, -1.0, 0.0).into(),
            tex_coord: (1.0, 0.0).into(),
        },
        Vertex {
            position: (-0.5, -0.5, -0.5).into(),
            normal: (0.0, -1.0, 0.0).into(),
            tex_coord: (0.0, 0.0).into(),
        },
    ];
    let vertex_buffer = renderer.create_vertex_buffer(
        &[
            VertexBufferElement::Float3,
            VertexBufferElement::Float3,
            VertexBufferElement::Float2,
        ],
        slice_to_bytes(vertices),
    );

    let index_buffer = renderer.create_index_buffer(&[
        0, 1, 2, 0, 2, 3, // front face
        6, 5, 4, 7, 6, 4, // back face
        8, 9, 10, 8, 10, 11, // left face
        14, 13, 12, 15, 14, 12, // right face
        16, 17, 18, 16, 18, 19, // top face
        22, 21, 20, 23, 22, 20, // bottom face
    ]);

    let stars_texture = match stb_image::image::load_from_memory_with_depth(
        include_bytes!("./stars.png"),
        4,
        false,
    ) {
        stb_image::image::LoadResult::Error(error) => panic!("{error}"),
        stb_image::image::LoadResult::ImageU8(image) => {
            let pixels = Pixels::RGBA(unsafe { slice_data_cast(&image.data) });
            renderer.create_texture((image.width, image.height).into(), pixels)
        }
        stb_image::image::LoadResult::ImageF32(image) => {
            let pixels = Pixels::RGBAF(unsafe { slice_data_cast(&image.data) });
            renderer.create_texture((image.width, image.height).into(), pixels)
        }
    };

    let mut camera = Camera {
        transform: Transform::default(),
        projection_type: CameraProjectionType::Perspective {
            fov: 60.0,
            aspect: {
                let (width, height) = renderer.get_surface().get_size().into();
                width as f32 / height as f32
            },
            near: 0.001,
            far: 1000.0,
        },
    };

    let mut cube_transform = Transform {
        position: (0.0, 0.0, 3.0).into(),
        ..Default::default()
    };

    renderer.get_surface_mut().show();
    let mut fixed_update_time = 0.0;
    let mut last_now = std::time::Instant::now();
    'main_loop: loop {
        let now = std::time::Instant::now();
        let ts = now.duration_since(last_now).as_secs_f32();
        last_now = now;

        for event in renderer.get_surface_mut().events() {
            match event {
                SurfaceEvent::Close => break 'main_loop,
                SurfaceEvent::Resize(size) => {
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
                SurfaceEvent::KeyPressed(key) => println!("{key:?} was pressed"),
                SurfaceEvent::KeyReleased(key) => println!("{key:?} was released"),
                SurfaceEvent::MousePressed(button, coord) => {
                    println!("{button:?} mouse button was pressed at {coord}")
                }
                SurfaceEvent::MouseReleased(button, coord) => {
                    println!("{button:?} mouse button was released at {coord}")
                }
                SurfaceEvent::MouseMoved(coord) => println!("Mouse was moved to {coord}"),
            }
        }

        const FIXED_UPDATE_INTERVAL: f32 = 1.0 / 60.0;
        fixed_update_time += ts;
        while fixed_update_time > FIXED_UPDATE_INTERVAL {
            fixed_update_time -= FIXED_UPDATE_INTERVAL;
            let ts = FIXED_UPDATE_INTERVAL;

            let surface = renderer.get_surface();
            if surface.get_key_state(Keycode::W) {
                camera.transform.position += camera.transform.forward() * Vector3::from(2.0 * ts);
            }
            if surface.get_key_state(Keycode::S) {
                camera.transform.position -= camera.transform.forward() * Vector3::from(2.0 * ts);
            }
            if surface.get_key_state(Keycode::A) {
                camera.transform.position -= camera.transform.right() * Vector3::from(2.0 * ts);
            }
            if surface.get_key_state(Keycode::D) {
                camera.transform.position += camera.transform.right() * Vector3::from(2.0 * ts);
            }
            if surface.get_key_state(Keycode::Q) {
                camera.transform.position -= camera.transform.up() * Vector3::from(2.0 * ts);
            }
            if surface.get_key_state(Keycode::E) {
                camera.transform.position += camera.transform.up() * Vector3::from(2.0 * ts);
            }

            if surface.get_key_state(Keycode::Left) {
                camera.transform.rotation.y -= 90.0 * ts;
            }
            if surface.get_key_state(Keycode::Right) {
                camera.transform.rotation.y += 90.0 * ts;
            }
            if surface.get_key_state(Keycode::Up) {
                camera.transform.rotation.x += 90.0 * ts;
            }
            if surface.get_key_state(Keycode::Down) {
                camera.transform.rotation.x -= 90.0 * ts;
            }

            cube_transform.rotation.x += 30.0 * ts;
            cube_transform.rotation.y += 40.0 * ts;
            cube_transform.rotation.z += 25.0 * ts;
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        {
            let mut draw_context = renderer.drawing_context(camera, true);
            draw_context.draw_indexed(
                PrimitiveType::Triangle,
                shader,
                vertex_buffer,
                index_buffer,
                Some(stars_texture),
                cube_transform.into(),
                Vector3::one(),
            );
        }
        renderer.present();
    }
    renderer.get_surface_mut().hide();
}
