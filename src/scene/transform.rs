use crate::math::{One, Vector3, Zero};

pub struct Transform<T> {
    pub position: Vector3<T>,
    pub rotation: Vector3<T>,
    pub scale: Vector3<T>,
}

impl<T> Transform<T> {
    pub fn new(position: Vector3<T>, rotation: Vector3<T>, scale: Vector3<T>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl<T> Default for Transform<T>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            position: (T::zero(), T::zero(), T::zero()).into(),
            rotation: (T::zero(), T::zero(), T::zero()).into(),
            scale: (T::one(), T::one(), T::one()).into(),
        }
    }
}
