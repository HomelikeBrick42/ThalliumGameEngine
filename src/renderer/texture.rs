use crate::{
    math::{Vector2, Vector3, Vector4},
    PhantomUnsend, PhantomUnsync,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureID(
    pub(crate) usize,
    pub(crate) PhantomUnsend,
    pub(crate) PhantomUnsync,
);

pub enum Pixels<'a> {
    RGB(&'a [Vector3<u8>]),
    RGBA(&'a [Vector4<u8>]),
    RGBF(&'a [Vector3<f32>]),
    RGBAF(&'a [Vector4<f32>]),
}

pub trait Texture {
    fn get_id(&self) -> TextureID;
    fn get_size(&self) -> Vector2<usize>;
    fn set_pixels(&mut self, size: Vector2<usize>, pixels: Pixels);
    fn get_pixels(&self) -> Vec<Vector4<f32>>;
}
