use std::ptr::NonNull;

use crate::mem_intern::{leak_as_nonnull, reclaim_as_boxed};

pub trait BoxedExt<T: ?Sized> {
    fn leak_as_nonnull(self) -> NonNull<T>;
    unsafe fn reclaim(raw_ptr: NonNull<T>) -> Self;
}

impl<T: ?Sized> BoxedExt<T> for Box<T> {
    #[inline] fn leak_as_nonnull(self) -> NonNull<T> {
        leak_as_nonnull(self)
    }

    #[inline] unsafe fn reclaim(raw_ptr: NonNull<T>) -> Self {
        reclaim_as_boxed(raw_ptr)
    }
}

pub trait VecExt<T> {
    fn into_slice_ptr(self) -> NonNull<[T]>;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline] fn into_slice_ptr(self) -> NonNull<[T]> {
        self.into_boxed_slice().leak_as_nonnull()
    }
}
