mod math;
mod opengl_renderer;
mod opengl_shader;
mod renderer;
mod shader;
mod vector2;
mod vector3;
mod window;

pub use math::*;
pub use renderer::*;
pub use shader::*;
pub use vector2::*;
pub use vector3::*;
pub use window::*;

use std::{cell::Cell, marker::PhantomData, sync::MutexGuard};

pub type PhantomUnsync = PhantomData<Cell<()>>;
pub type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
