use crate::math::{One, Zero};

use super::Vector4;

pub type Matrix4x4<T> = Matrix<T, 4, 4>;

pub struct Matrix<T, const R: usize, const C: usize> {
    elements: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(elements: [[T; C]; R]) -> Self {
        Self { elements }
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
        let result = self * matrix;
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

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            elements: std::array::from_fn(|row| {
                std::array::from_fn(|column| if row == column { T::one() } else { T::zero() })
            }),
        }
    }
}
