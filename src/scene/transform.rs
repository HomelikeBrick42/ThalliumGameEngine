use crate::math::{Cos, Matrix4x4, One, Sin, ToRadians, Vector3, Vector4, Zero};

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

    pub fn forward(self) -> Vector3<T>
    where
        T: Clone
            + Zero
            + One
            + ToRadians
            + Sin
            + Cos
            + std::ops::Mul<T, Output = T>
            + std::ops::Add<T, Output = T>
            + std::ops::Neg<Output = T>,
    {
        let (x, y, z, _) = (Matrix4x4::rotation(self.rotation)
            * Vector4::new(T::zero(), T::zero(), T::one(), T::one()))
        .into();
        (x, y, z).into()
    }

    pub fn right(self) -> Vector3<T>
    where
        T: Clone
            + Zero
            + One
            + ToRadians
            + Sin
            + Cos
            + std::ops::Mul<T, Output = T>
            + std::ops::Add<T, Output = T>
            + std::ops::Neg<Output = T>,
    {
        let (x, y, z, _) = (Matrix4x4::rotation(self.rotation)
            * Vector4::new(T::one(), T::zero(), T::zero(), T::one()))
        .into();
        (x, y, z).into()
    }

    pub fn up(self) -> Vector3<T>
    where
        T: Clone
            + Zero
            + One
            + ToRadians
            + Sin
            + Cos
            + std::ops::Mul<T, Output = T>
            + std::ops::Add<T, Output = T>
            + std::ops::Neg<Output = T>,
    {
        let (x, y, z, _) = (Matrix4x4::rotation(self.rotation)
            * Vector4::new(T::zero(), T::one(), T::zero(), T::one()))
        .into();
        (x, y, z).into()
    }
}

impl<T> Into<Matrix4x4<T>> for Transform<T>
where
    T: Clone
        + Zero
        + One
        + ToRadians
        + Sin
        + Cos
        + std::ops::Mul<T, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Neg<Output = T>,
{
    fn into(self) -> Matrix4x4<T> {
        // TODO: rotation
        Matrix4x4::scale(self.scale)
            * Matrix4x4::rotation(self.rotation)
            * Matrix4x4::translation(self.position)
    }
}

impl<T> Clone for Transform<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
        }
    }
}

impl<T> Copy for Transform<T> where T: Copy {}

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
