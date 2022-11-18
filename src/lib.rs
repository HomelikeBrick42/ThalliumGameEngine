pub mod math;
pub mod platform;
pub mod renderer;
pub mod scene;

pub type PhantomUnsync = std::marker::PhantomData<std::cell::Cell<()>>;
pub type PhantomUnsend = std::marker::PhantomData<std::sync::MutexGuard<'static, ()>>;

pub fn slice_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe { slice_data_cast(slice) }
}

pub unsafe fn slice_data_cast<U, T>(slice: &[T]) -> &[U] {
    let (start, result, end) = slice.align_to();
    assert_eq!((start.len(), end.len()), (0, 0));
    result
}
