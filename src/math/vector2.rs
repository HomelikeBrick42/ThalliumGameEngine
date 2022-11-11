use crate::math::{One, Recip, Sqrt, Zero};

pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> T
    where
        T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T>,
    {
        self.x * other.x + self.y * other.y
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
        self.clone() - Vector2::from(T::from(2) * self.dot(normal.clone())) * normal
    }
}

impl<T, U> std::ops::Add<Vector2<U>> for Vector2<T>
where
    T: std::ops::Add<U>,
{
    type Output = Vector2<<T as std::ops::Add<U>>::Output>;

    fn add(self, other: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T, U> std::ops::Sub<Vector2<U>> for Vector2<T>
where
    T: std::ops::Sub<U>,
{
    type Output = Vector2<<T as std::ops::Sub<U>>::Output>;

    fn sub(self, other: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, U> std::ops::Mul<Vector2<U>> for Vector2<T>
where
    T: std::ops::Mul<U>,
{
    type Output = Vector2<<T as std::ops::Mul<U>>::Output>;

    fn mul(self, other: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T, U> std::ops::Div<Vector2<U>> for Vector2<T>
where
    T: std::ops::Div<U>,
{
    type Output = Vector2<<T as std::ops::Div<U>>::Output>;

    fn div(self, other: Vector2<U>) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T, U> std::ops::AddAssign<Vector2<U>> for Vector2<T>
where
    T: std::ops::AddAssign<U>,
{
    fn add_assign(&mut self, other: Vector2<U>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T, U> std::ops::SubAssign<Vector2<U>> for Vector2<T>
where
    T: std::ops::SubAssign<U>,
{
    fn sub_assign(&mut self, other: Vector2<U>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T, U> std::ops::MulAssign<Vector2<U>> for Vector2<T>
where
    T: std::ops::MulAssign<U>,
{
    fn mul_assign(&mut self, other: Vector2<U>) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T, U> std::ops::DivAssign<Vector2<U>> for Vector2<T>
where
    T: std::ops::DivAssign<U>,
{
    fn div_assign(&mut self, other: Vector2<U>) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<T> std::ops::Neg for Vector2<T>
where
    T: std::ops::Neg,
{
    type Output = Vector2<<T as std::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Default for Vector2<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Zero for Vector2<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> One for Vector2<T>
where
    T: One,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T> Clone for Vector2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Copy for Vector2<T> where T: Copy {}

impl<T> From<T> for Vector2<T>
where
    T: Clone,
{
    fn from(scalar: T) -> Self {
        Self {
            x: scalar.clone(),
            y: scalar.clone(),
        }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vector2 { x, y }
    }
}

impl<T> Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T, U> PartialEq<Vector2<U>> for Vector2<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vector2<U>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Vector2<T> where T: Eq {}

impl<T> std::fmt::Debug for Vector2<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T> std::fmt::Display for Vector2<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
