use std::pin::Pin;

use crate::{
    math::{Matrix4x4, Vector2, Vector3},
    renderer::{
        opengl::OpenGLRenderer, IndexBuffer, IndexBufferID, Pixels, Shader, ShaderID, Texture,
        TextureID, VertexBuffer, VertexBufferElement, VertexBufferID,
    },
    scene::Camera,
    Window,
};

pub enum RendererAPI {
    OpenGL,
}

pub(crate) fn new_renderer(window: Pin<Box<Window>>, api: RendererAPI) -> Box<dyn Renderer> {
    match api {
        RendererAPI::OpenGL => Box::new(OpenGLRenderer::new(window)),
    }
}

pub trait Renderer {
    fn get_window(&self) -> &Window;
    fn get_window_mut(&mut self) -> &mut Window;
    fn take_window(self) -> Pin<Box<Window>>;

    fn create_shader(
        &mut self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<ShaderID, String>;
    fn destroy_shader(&mut self, id: ShaderID);
    fn get_shader(&self, id: ShaderID) -> Option<&dyn Shader>;
    fn get_shader_mut(&mut self, id: ShaderID) -> Option<&mut dyn Shader>;

    fn create_vertex_buffer(
        &mut self,
        layout: &[VertexBufferElement],
        data: &[u8],
    ) -> VertexBufferID;
    fn destroy_vertex_buffer(&mut self, id: VertexBufferID);
    fn get_vertex_buffer(&self, id: VertexBufferID) -> Option<&dyn VertexBuffer>;
    fn get_vertex_buffer_mut(&mut self, id: VertexBufferID) -> Option<&mut dyn VertexBuffer>;

    fn create_index_buffer(&mut self, indices: &[u32]) -> IndexBufferID;
    fn destroy_index_buffer(&mut self, id: IndexBufferID);
    fn get_index_buffer(&self, id: IndexBufferID) -> Option<&dyn IndexBuffer>;
    fn get_index_buffer_mut(&mut self, id: IndexBufferID) -> Option<&mut dyn IndexBuffer>;

    fn create_texture(&mut self, size: Vector2<usize>, pixels: Pixels) -> TextureID;
    fn destroy_texture(&mut self, id: TextureID);
    fn get_texture(&self, id: TextureID) -> Option<&dyn Texture>;
    fn get_texture_mut(&mut self, id: TextureID) -> Option<&mut dyn Texture>;

    fn resize(&mut self, size: Vector2<usize>);
    fn present(&mut self);

    fn clear(&mut self, color: Vector3<f32>);
    fn drawing_context<'a>(
        &'a mut self,
        camera: Camera<f32>,
        depth_testing: bool,
    ) -> Box<dyn RendererDrawContext + 'a>;
}

pub trait RendererDrawContext {
    /// If `None` is passed as `texture` then a default texture of a single white pixel is used
    fn draw(
        &mut self,
        shader: ShaderID,
        vertex_buffer: VertexBufferID,
        texture: Option<TextureID>,
        model_matrix: Matrix4x4<f32>,
        color: Vector3<f32>,
    );

    /// If `None` is passed as `texture` then a default texture of a single white pixel is used
    fn draw_indexed(
        &mut self,
        shader: ShaderID,
        vertex_buffer: VertexBufferID,
        index_buffer: IndexBufferID,
        texture: Option<TextureID>,
        model_matrix: Matrix4x4<f32>,
        color: Vector3<f32>,
    );
}
