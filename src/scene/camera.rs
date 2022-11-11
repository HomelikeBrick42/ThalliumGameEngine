use crate::math::{Matrix4x4, One, Two, Zero};

use super::Transform;

pub enum CameraProjectionType<T> {
    Orthographic {
        left: T,
        right: T,
        top: T,
        bottom: T,
        near: T,
        far: T,
    },
}

impl<T> CameraProjectionType<T> {
    pub fn to_matrix(self) -> Matrix4x4<T>
    where
        T: Clone
            + Zero
            + One
            + Two
            + std::ops::Add<T, Output = T>
            + std::ops::Sub<T, Output = T>
            + std::ops::Div<T, Output = T>
            + std::ops::Neg<Output = T>,
    {
        match self {
            CameraProjectionType::Orthographic {
                left,
                right,
                top,
                bottom,
                near,
                far,
            } => Matrix4x4::new([
                [
                    T::two() / (right.clone() - left.clone()),
                    T::zero(),
                    T::zero(),
                    T::zero(),
                ],
                [
                    T::zero(),
                    T::two() / (top.clone() - bottom.clone()),
                    T::zero(),
                    T::zero(),
                ],
                [
                    T::zero(),
                    T::zero(),
                    T::two() / (far.clone() - near.clone()),
                    T::zero(),
                ],
                [
                    -(right.clone() + left.clone()) / (right - left),
                    -(top.clone() + bottom.clone()) / (top - bottom),
                    -(far.clone() + near.clone()) / (far - near),
                    T::one(),
                ],
            ]),
        }
    }
}

impl<T> Clone for CameraProjectionType<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Orthographic {
                left,
                right,
                top,
                bottom,
                near,
                far,
            } => Self::Orthographic {
                left: left.clone(),
                right: right.clone(),
                top: top.clone(),
                bottom: bottom.clone(),
                near: near.clone(),
                far: far.clone(),
            },
        }
    }
}

impl<T> Copy for CameraProjectionType<T> where T: Copy {}

pub struct Camera<T> {
    pub transform: Transform<T>,
    pub projection_type: CameraProjectionType<T>,
}

impl<T> Clone for Camera<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            transform: self.transform.clone(),
            projection_type: self.projection_type.clone(),
        }
    }
}

impl<T> Copy for Camera<T> where T: Copy {}
