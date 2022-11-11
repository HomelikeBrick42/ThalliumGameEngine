use game_engine::*;

fn main() {
    let mut renderer = Window::new((640, 480).into(), "Test").into_renderer(RendererAPI::OpenGL);

    let shader = renderer.create_shader(
        include_str!("./basic.vert.glsl"),
        include_str!("./basic.frag.glsl"),
    );

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
            }
        }

        renderer.clear((0.2, 0.4, 0.8).into());
        renderer.draw(shader, vertex_buffer);
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
