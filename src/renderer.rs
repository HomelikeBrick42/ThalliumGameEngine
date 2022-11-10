use std::pin::Pin;

use crate::{opengl_renderer::OpenGLRenderer, Vector3, Window};

pub enum RendererAPI {
    OpenGL,
}

pub trait Renderer {
    fn get_window(&self) -> &Window;
    fn get_window_mut(&mut self) -> &mut Window;
    fn take_window(self) -> Pin<Box<Window>>;

    fn clear(&mut self, color: Vector3<f32>);
    fn present(&mut self);
}

pub(crate) fn new_renderer(window: Pin<Box<Window>>, api: RendererAPI) -> Box<dyn Renderer> {
    match api {
        RendererAPI::OpenGL => Box::new(OpenGLRenderer::new(window)),
    }
}
