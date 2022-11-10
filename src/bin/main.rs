use game_engine::*;

fn main() {
    let mut renderer = Window::new(640, 480, "Test").into_renderer(RendererAPI::OpenGL);

    renderer.get_window_mut().show();
    'main_loop: loop {
        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(size) => renderer.resize(size),
            }
        }

        renderer.clear(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        renderer.present();
    }
    renderer.get_window_mut().hide();
}
