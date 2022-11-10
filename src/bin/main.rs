use game_engine::*;

fn main() {
    let mut renderer = Window::new(640, 480, "Test").into_renderer(RendererAPI::OpenGL);

    renderer.get_window_mut().show();
    'main_loop: loop {
        for event in renderer.get_window_mut().events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(width, height) => println!("Resize: {width}, {height}"),
            }
        }
    }
    renderer.get_window_mut().hide();
}
