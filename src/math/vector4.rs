use crate::math::{One, Recip, Sqrt, Two, Zero};

pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, other: Self) -> T
    where
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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
        T: Two + std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Clone,
    {
        self.clone() - Vector4::from(T::two() * self.dot(normal.clone())) * normal
    }
}

impl<T, U> std::ops::Add<Vector4<U>> for Vector4<T>
where
    T: std::ops::Add<U>,
{
    type Output = Vector4<<T as std::ops::Add<U>>::Output>;

    fn add(self, other: Vector4<U>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T, U> std::ops::Sub<Vector4<U>> for Vector4<T>
where
    T: std::ops::Sub<U>,
{
    type Output = Vector4<<T as std::ops::Sub<U>>::Output>;

    fn sub(self, other: Vector4<U>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T, U> std::ops::Mul<Vector4<U>> for Vector4<T>
where
    T: std::ops::Mul<U>,
{
    type Output = Vector4<<T as std::ops::Mul<U>>::Output>;

    fn mul(self, other: Vector4<U>) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T, U> std::ops::Div<Vector4<U>> for Vector4<T>
where
    T: std::ops::Div<U>,
{
    type Output = Vector4<<T as std::ops::Div<U>>::Output>;

    fn div(self, other: Vector4<U>) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T, U> std::ops::AddAssign<Vector4<U>> for Vector4<T>
where
    T: std::ops::AddAssign<U>,
{
    fn add_assign(&mut self, other: Vector4<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T, U> std::ops::SubAssign<Vector4<U>> for Vector4<T>
where
    T: std::ops::SubAssign<U>,
{
    fn sub_assign(&mut self, other: Vector4<U>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T, U> std::ops::MulAssign<Vector4<U>> for Vector4<T>
where
    T: std::ops::MulAssign<U>,
{
    fn mul_assign(&mut self, other: Vector4<U>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl<T, U> std::ops::DivAssign<Vector4<U>> for Vector4<T>
where
    T: std::ops::DivAssign<U>,
{
    fn div_assign(&mut self, other: Vector4<U>) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

impl<T> std::ops::Neg for Vector4<T>
where
    T: std::ops::Neg,
{
    type Output = Vector4<<T as std::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> Default for Vector4<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T> Zero for Vector4<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }
}

impl<T> One for Vector4<T>
where
    T: One,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
            w: T::one(),
        }
    }
}

impl<T> Two for Vector4<T>
where
    T: Two,
{
    fn two() -> Self {
        Self {
            x: T::two(),
            y: T::two(),
            z: T::two(),
            w: T::two(),
        }
    }
}

impl<T> Clone for Vector4<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            w: self.w.clone(),
        }
    }
}

impl<T> Copy for Vector4<T> where T: Copy {}

impl<T> From<T> for Vector4<T>
where
    T: Clone,
{
    fn from(scalar: T) -> Self {
        Self {
            x: scalar.clone(),
            y: scalar.clone(),
            z: scalar.clone(),
            w: scalar,
        }
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Vector4 { x, y, z, w }
    }
}

impl<T> Into<(T, T, T, T)> for Vector4<T> {
    fn into(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T, U> PartialEq<Vector4<U>> for Vector4<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vector4<U>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T> Eq for Vector4<T> where T: Eq {}

impl<T> std::fmt::Debug for Vector4<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl<T> std::fmt::Display for Vector4<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
