use game_engine::*;

fn main() {
    let mut window = Window::new(640, 480, "Test");

    window.show();
    'main_loop: loop {
        for event in window.events() {
            match event {
                WindowEvent::Close => break 'main_loop,
                WindowEvent::Resize(width, height) => println!("Resize: {width}, {height}"),
            }
        }
    }
    window.hide();
}
