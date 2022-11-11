use crate::math::{One, Recip, Sqrt, Zero};

pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> T
    where
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sqr_length(self) -> T
    where
        Self: Clone,
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T>,
    {
        self.clone().dot(self)
    }

    pub fn length(self) -> T
    where
        Self: Clone,
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Sqrt,
    {
        self.sqr_length().sqrt()
    }

    pub fn normalized(self) -> Self
    where
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Sqrt + Recip + Clone,
    {
        self.clone() * self.length().recip().into()
    }

    pub fn reflect(self, normal: Self) -> Self
    where
        Self: std::ops::Sub<Self, Output = Self>,
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Clone + From<u8>,
    {
        self.clone() - Vector3::from(T::from(2) * self.dot(normal.clone())) * normal
    }

    pub fn cross(self, other: Self) -> Self
    where
        T: std::ops::Sub<T, Output = T> + std::ops::Mul<T, Output = T> + Clone,
    {
        Self {
            x: self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            y: self.z * other.x.clone() - self.x.clone() * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T, U> std::ops::Add<Vector3<U>> for Vector3<T>
where
    T: std::ops::Add<U>,
{
    type Output = Vector3<<T as std::ops::Add<U>>::Output>;

    fn add(self, other: Vector3<U>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T, U> std::ops::Sub<Vector3<U>> for Vector3<T>
where
    T: std::ops::Sub<U>,
{
    type Output = Vector3<<T as std::ops::Sub<U>>::Output>;

    fn sub(self, other: Vector3<U>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T, U> std::ops::Mul<Vector3<U>> for Vector3<T>
where
    T: std::ops::Mul<U>,
{
    type Output = Vector3<<T as std::ops::Mul<U>>::Output>;

    fn mul(self, other: Vector3<U>) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T, U> std::ops::Div<Vector3<U>> for Vector3<T>
where
    T: std::ops::Div<U>,
{
    type Output = Vector3<<T as std::ops::Div<U>>::Output>;

    fn div(self, other: Vector3<U>) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T, U> std::ops::AddAssign<Vector3<U>> for Vector3<T>
where
    T: std::ops::AddAssign<U>,
{
    fn add_assign(&mut self, other: Vector3<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T, U> std::ops::SubAssign<Vector3<U>> for Vector3<T>
where
    T: std::ops::SubAssign<U>,
{
    fn sub_assign(&mut self, other: Vector3<U>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T, U> std::ops::MulAssign<Vector3<U>> for Vector3<T>
where
    T: std::ops::MulAssign<U>,
{
    fn mul_assign(&mut self, other: Vector3<U>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T, U> std::ops::DivAssign<Vector3<U>> for Vector3<T>
where
    T: std::ops::DivAssign<U>,
{
    fn div_assign(&mut self, other: Vector3<U>) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> std::ops::Neg for Vector3<T>
where
    T: std::ops::Neg,
{
    type Output = Vector3<<T as std::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Default for Vector3<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> Zero for Vector3<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> One for Vector3<T>
where
    T: One,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }
}

impl<T> Clone for Vector3<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T> Copy for Vector3<T> where T: Copy {}

impl<T> From<T> for Vector3<T>
where
    T: Clone,
{
    fn from(scalar: T) -> Self {
        Self {
            x: scalar.clone(),
            y: scalar.clone(),
            z: scalar,
        }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Vector3 { x, y, z }
    }
}

impl<T> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T, U> PartialEq<Vector3<U>> for Vector3<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vector3<U>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Eq for Vector3<T> where T: Eq {}

impl<T> std::fmt::Debug for Vector3<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T> std::fmt::Display for Vector3<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
