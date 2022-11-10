mod math;
mod opengl_renderer;
mod opengl_shader;
mod opengl_vertex_buffer;
mod renderer;
mod shader;
mod vector2;
mod vector3;
mod vertex_buffer;
mod window;

pub use math::*;
pub use renderer::*;
pub use shader::*;
pub use vector2::*;
pub use vector3::*;
pub use vertex_buffer::*;
pub use window::*;

pub type PhantomUnsync = std::marker::PhantomData<std::cell::Cell<()>>;
pub type PhantomUnsend = std::marker::PhantomData<std::sync::MutexGuard<'static, ()>>;

pub fn slice_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr().cast(),
            slice.len() * std::mem::size_of::<T>(),
        )
    }
}
