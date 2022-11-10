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
