use std::pin::Pin;

use crate::{opengl_renderer::OpenGLRenderer, Shader, ShaderID, Vector2, Vector3, Window};

pub enum RendererAPI {
    OpenGL,
}

pub trait Renderer {
    fn get_window(&self) -> &Window;
    fn get_window_mut(&mut self) -> &mut Window;
    fn take_window(self) -> Pin<Box<Window>>;

    fn create_shader(
        &mut self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> ShaderID;
    fn destroy_shader(&mut self, id: ShaderID);
    fn get_shader(&self, id: ShaderID) -> &dyn Shader;
    fn get_shader_mut(&mut self, id: ShaderID) -> &mut dyn Shader;

    fn resize(&mut self, size: Vector2<usize>);
    fn present(&mut self);

    fn clear(&mut self, color: Vector3<f32>);
}

pub(crate) fn new_renderer(window: Pin<Box<Window>>, api: RendererAPI) -> Box<dyn Renderer> {
    match api {
        RendererAPI::OpenGL => Box::new(OpenGLRenderer::new(window)),
    }
}
