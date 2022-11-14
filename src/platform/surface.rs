use enum_map::Enum;

use crate::math::Vector2;

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
