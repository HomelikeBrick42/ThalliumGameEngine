use crate::{PhantomUnsend, PhantomUnsync};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexBufferID(
    pub(crate) usize,
    pub(crate) PhantomUnsend,
    pub(crate) PhantomUnsync,
);

pub trait IndexBuffer {
    fn get_id(&self) -> IndexBufferID;
    fn get_count(&self) -> usize;
    fn set_indices(&mut self, indices: &[u32]);
}
