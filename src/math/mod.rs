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

pub trait ToDegrees {
    fn to_degrees(self) -> Self;
}

impl ToDegrees for f32 {
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }
}

impl ToDegrees for f64 {
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }
}

pub trait ToRadians {
    fn to_radians(self) -> Self;
}

impl ToRadians for f32 {
    fn to_radians(self) -> Self {
        self.to_radians()
    }
}

impl ToRadians for f64 {
    fn to_radians(self) -> Self {
        self.to_radians()
    }
}

pub trait Sin {
    fn sin(self) -> Self;
}

impl Sin for f32 {
    fn sin(self) -> Self {
        self.sin()
    }
}

impl Sin for f64 {
    fn sin(self) -> Self {
        self.sin()
    }
}

pub trait Cos {
    fn cos(self) -> Self;
}

impl Cos for f32 {
    fn cos(self) -> Self {
        self.cos()
    }
}

impl Cos for f64 {
    fn cos(self) -> Self {
        self.cos()
    }
}

pub trait Tan {
    fn tan(self) -> Self;
}

impl Tan for f32 {
    fn tan(self) -> Self {
        self.tan()
    }
}

impl Tan for f64 {
    fn tan(self) -> Self {
        self.tan()
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

pub trait Two {
    fn two() -> Self;
}

macro_rules! two_integer_like {
    ($type:ty) => {
        impl Two for $type {
            fn two() -> Self {
                2
            }
        }
    };
}

two_integer_like!(u8);
two_integer_like!(u16);
two_integer_like!(u32);
two_integer_like!(u64);
two_integer_like!(usize);
two_integer_like!(i8);
two_integer_like!(i16);
two_integer_like!(i32);
two_integer_like!(i64);
two_integer_like!(isize);

impl Two for f32 {
    fn two() -> Self {
        2.0
    }
}

impl Two for f64 {
    fn two() -> Self {
        2.0
    }
}
