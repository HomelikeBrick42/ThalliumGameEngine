pub mod math;
pub mod renderer;

mod window;

pub use window::*;

pub type PhantomUnsync = std::marker::PhantomData<std::cell::Cell<()>>;
pub type PhantomUnsend = std::marker::PhantomData<std::sync::MutexGuard<'static, ()>>;

pub fn slice_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr().cast(),
            slice.len() * std::mem::size_of::<T>(),
        )
    }
}
