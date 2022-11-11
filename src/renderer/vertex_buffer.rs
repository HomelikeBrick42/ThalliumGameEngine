use crate::{PhantomUnsend, PhantomUnsync};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexBufferID(
    pub(crate) usize,
    pub(crate) PhantomUnsend,
    pub(crate) PhantomUnsync,
);

pub enum VertexBufferElement {
    Float,
    Float2,
    Float3,
    Float4,
}

pub trait VertexBuffer {
    fn get_id(&self) -> VertexBufferID;
    fn get_count(&self) -> usize;
    fn set_layout(&mut self, layout: &[VertexBufferElement], data: &[u8]);
    fn set_data(&mut self, data: &[u8]);
}
