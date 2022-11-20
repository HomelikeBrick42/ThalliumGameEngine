#[allow(unused_imports)]
use std::{marker::PhantomData, pin::Pin};

use enum_map::Enum;

#[allow(unused_imports)]
use crate::{
    math::Vector2,
    renderer::{new_renderer, Renderer, RendererAPI},
};

pub enum SurfaceEvent {
    Close,
    Resize(Vector2<usize>),
    KeyPressed(Keycode),
    KeyReleased(Keycode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
pub enum Keycode {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Left,
    Up,
    Right,
    Down,
    Control,
    Shift,
    Alt,
}

#[cfg(target_os = "windows")]
pub use crate::platform::windows::*;

#[cfg(not(target_os = "windows"))]
pub struct Surface(PhantomData<()>);

#[cfg(not(target_os = "windows"))]
impl Surface {
    pub fn new(_size: Vector2<usize>, _title: &str) -> Pin<Box<Surface>> {
        unimplemented!()
    }

    pub fn show(&mut self) {
        unimplemented!()
    }
    pub fn hide(&mut self) {
        unimplemented!()
    }

    pub fn get_size(&self) -> Vector2<usize> {
        unimplemented!()
    }

    pub fn get_key_state(&self, _key: Keycode) -> bool {
        unimplemented!()
    }

    pub fn events(&mut self) -> impl Iterator<Item = SurfaceEvent> {
        std::iter::empty()
    }

    pub fn into_renderer(self: Pin<Box<Surface>>, api: RendererAPI) -> Box<dyn Renderer> {
        new_renderer(self, api)
    }
}
