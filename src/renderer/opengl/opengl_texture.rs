use std::{marker::PhantomData, sync::atomic::AtomicUsize};

use gl::types::GLuint;

use crate::{
    math::{Vector2, Vector4, Zero},
    renderer::{Pixels, Texture, TextureID},
    PhantomUnsend, PhantomUnsync,
};

pub(crate) struct OpenGLTexture {
    id: TextureID,
    opengl_id: GLuint,
    size: Vector2<usize>,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl OpenGLTexture {
    pub(crate) fn new(size: Vector2<usize>, pixels: Pixels) -> OpenGLTexture {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);

            let mut texture = OpenGLTexture {
                id: {
                    static ID: AtomicUsize = AtomicUsize::new(1);
                    TextureID(
                        ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
                        PhantomData,
                        PhantomData,
                    )
                },
                opengl_id: texture,
                size: 0.into(),
                _send: PhantomData,
                _sync: PhantomData,
            };

            texture.bind(0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            texture.unbind();

            texture.set_pixels(size, pixels);
            texture
        }
    }

    pub(crate) fn bind(&mut self, unit: u32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.opengl_id);
            gl::BindTextureUnit(unit, self.opengl_id);
        }
    }

    pub(crate) fn unbind(&mut self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) };
    }
}

impl Drop for OpenGLTexture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.opengl_id) };
    }
}

impl Texture for OpenGLTexture {
    fn get_id(&self) -> TextureID {
        self.id
    }

    fn get_size(&self) -> Vector2<usize> {
        self.size
    }

    fn set_pixels(&mut self, size: Vector2<usize>, pixels: Pixels) {
        unsafe {
            self.bind(0);
            self.size = size;
            let (format, typ, pixels) = match pixels {
                Pixels::RGB(pixels) => {
                    assert_eq!(size.x * size.y, pixels.len());
                    (gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr().cast())
                }
                Pixels::RGBA(pixels) => {
                    assert_eq!(size.x * size.y, pixels.len());
                    (gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr().cast())
                }
                Pixels::RGBF(pixels) => {
                    assert_eq!(size.x * size.y, pixels.len());
                    (gl::RGB, gl::FLOAT, pixels.as_ptr().cast())
                }
                Pixels::RGBAF(pixels) => {
                    assert_eq!(size.x * size.y, pixels.len());
                    (gl::RGBA, gl::FLOAT, pixels.as_ptr().cast())
                }
            };
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as _,
                size.x as _,
                size.y as _,
                0,
                format,
                typ,
                pixels,
            );
            self.unbind();
        }
    }

    fn get_pixels(&self) -> Vec<Vector4<f32>> {
        unsafe {
            let mut pixels = vec![Vector4::zero(); self.size.x * self.size.y];
            gl::BindTexture(gl::TEXTURE_2D, self.opengl_id);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGBA,
                gl::FLOAT,
                pixels.as_mut_ptr().cast(),
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
            pixels
        }
    }
}
