use std::mem;
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "we are not ready handle ZSTs");
        MyVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }
}
