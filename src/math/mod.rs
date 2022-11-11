mod matrix;
mod vector2;
mod vector3;
mod vector4;

pub use matrix::*;
pub use vector2::*;
pub use vector3::*;
pub use vector4::*;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Recip {
    fn recip(self) -> Self;
}

impl Recip for f32 {
    fn recip(self) -> Self {
        self.recip()
    }
}

impl Recip for f64 {
    fn recip(self) -> Self {
        self.recip()
    }
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! zero_integer_like {
    ($type:ty) => {
        impl Zero for $type {
            fn zero() -> Self {
                0
            }
        }
    };
}

zero_integer_like!(u8);
zero_integer_like!(u16);
zero_integer_like!(u32);
zero_integer_like!(u64);
zero_integer_like!(usize);
zero_integer_like!(i8);
zero_integer_like!(i16);
zero_integer_like!(i32);
zero_integer_like!(i64);
zero_integer_like!(isize);

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

pub trait One {
    fn one() -> Self;
}

macro_rules! one_integer_like {
    ($type:ty) => {
        impl One for $type {
            fn one() -> Self {
                1
            }
        }
    };
}

one_integer_like!(u8);
one_integer_like!(u16);
one_integer_like!(u32);
one_integer_like!(u64);
one_integer_like!(usize);
one_integer_like!(i8);
one_integer_like!(i16);
one_integer_like!(i32);
one_integer_like!(i64);
one_integer_like!(isize);

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}
