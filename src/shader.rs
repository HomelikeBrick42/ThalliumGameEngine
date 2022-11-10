use crate::{PhantomUnsend, PhantomUnsync};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderID(
    pub(crate) usize,
    pub(crate) PhantomUnsend,
    pub(crate) PhantomUnsync,
);

pub trait Shader {
    fn get_id(&self) -> ShaderID;
}
