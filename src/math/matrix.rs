use crate::math::{Cos, One, Sin, ToRadians, Vector3, Vector4, Zero};

pub type Matrix4x4<T> = Matrix<T, 4, 4>;

pub struct Matrix<T, const R: usize, const C: usize> {
    elements: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(elements: [[T; C]; R]) -> Self {
        Self { elements }
    }

    pub fn identity() -> Self
    where
        T: Zero + One,
    {
        Self {
            elements: std::array::from_fn(|row| {
                std::array::from_fn(|column| if row == column { T::one() } else { T::zero() })
            }),
        }
    }

    pub fn transpose(&self) -> Matrix<T, C, R>
    where
        T: Clone,
    {
        Matrix {
            elements: std::array::from_fn(|i| std::array::from_fn(|j| self[j][i].clone())),
        }
    }
}

impl<T> Matrix4x4<T> {
    pub fn scale(scale: Vector3<T>) -> Self
    where
        T: Zero + One,
    {
        Self {
            elements: [
                [scale.x, T::zero(), T::zero(), T::zero()],
                [T::zero(), scale.y, T::zero(), T::zero()],
                [T::zero(), T::zero(), scale.z, T::zero()],
                [T::zero(), T::zero(), T::zero(), T::one()],
            ],
        }
    }

    pub fn translation(offset: Vector3<T>) -> Self
    where
        T: Zero + One,
    {
        Self {
            elements: [
                [T::one(), T::zero(), T::zero(), T::zero()],
                [T::zero(), T::one(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::one(), T::zero()],
                [offset.x, offset.y, offset.z, T::one()],
            ],
        }
    }

    pub fn rotation_x(degrees: T) -> Self
    where
        T: Clone + Zero + One + ToRadians + Sin + Cos + std::ops::Neg<Output = T>,
    {
        let radians = degrees.to_radians();
        Self {
            elements: [
                [T::one(), T::zero(), T::zero(), T::zero()],
                [
                    T::zero(),
                    radians.clone().cos(),
                    -radians.clone().sin(),
                    T::zero(),
                ],
                [T::zero(), radians.clone().sin(), radians.cos(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::one()],
            ],
        }
    }

    pub fn rotation_y(degrees: T) -> Self
    where
        T: Clone + Zero + One + ToRadians + Sin + Cos + std::ops::Neg<Output = T>,
    {
        let radians = -degrees.to_radians();
        Self {
            elements: [
                [
                    radians.clone().cos(),
                    T::zero(),
                    radians.clone().sin(),
                    T::zero(),
                ],
                [T::zero(), T::one(), T::zero(), T::zero()],
                [-radians.clone().sin(), T::zero(), radians.cos(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::one()],
            ],
        }
    }

    pub fn rotation_z(degrees: T) -> Self
    where
        T: Clone + Zero + One + ToRadians + Sin + Cos + std::ops::Neg<Output = T>,
    {
        let radians = -degrees.to_radians();
        Self {
            elements: [
                [
                    radians.clone().cos(),
                    -radians.clone().sin(),
                    T::zero(),
                    T::zero(),
                ],
                [radians.clone().sin(), radians.cos(), T::zero(), T::zero()],
                [T::zero(), T::zero(), T::one(), T::zero()],
                [T::zero(), T::zero(), T::zero(), T::one()],
            ],
        }
    }

    pub fn rotation(degrees: Vector3<T>) -> Self
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
        Self::rotation_z(degrees.z) * Self::rotation_x(degrees.x) * Self::rotation_y(degrees.y)
    }
}

impl<T, const R1: usize, const C1R2: usize, const C2: usize> std::ops::Mul<Matrix<T, C1R2, C2>>
    for Matrix<T, R1, C1R2>
where
    T: Clone + Zero + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    type Output = Matrix<T, R1, C2>;

    fn mul(self, other: Matrix<T, C1R2, C2>) -> Self::Output {
        Matrix::new(std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                let mut sum = T::zero();
                for k in 0..C1R2 {
                    sum = sum + self[i][k].clone() * other[k][j].clone();
                }
                sum
            })
        }))
    }
}

impl<T> std::ops::Mul<Vector4<T>> for Matrix<T, 4, 4>
where
    T: Clone + Zero + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    type Output = Vector4<T>;

    fn mul(self, other: Vector4<T>) -> Self::Output {
        let matrix = Matrix::new([[other.x], [other.y], [other.z], [other.w]]);
        let result = self.transpose() * matrix;
        Self::Output {
            x: result[0][0].clone(),
            y: result[1][0].clone(),
            z: result[2][0].clone(),
            w: result[3][0].clone(),
        }
    }
}

impl<T, const R: usize, const C: usize> std::ops::Index<usize> for Matrix<T, R, C> {
    type Output = [T; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T, const R: usize, const C: usize> std::ops::IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T, const R: usize, const C: usize> Clone for Matrix<T, R, C>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            elements: self.elements.clone(),
        }
    }
}

impl<T, const R: usize, const C: usize> Copy for Matrix<T, R, C> where T: Copy {}

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            elements: std::array::from_fn(|_| std::array::from_fn(|_| Default::default())),
        }
    }
}
