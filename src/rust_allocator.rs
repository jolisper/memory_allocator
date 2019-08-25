use std::alloc::{GlobalAlloc, Layout};
use crate::{alloc, free, WordSize};

/// Rust memory allocator using the mem alloc
#[derive(Debug)]
pub struct RustAllocator;

unsafe impl GlobalAlloc for RustAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        alloc(size) as *mut u8
        //alloc(size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        #[allow(clippy::cast_ptr_alignment)]
        free(ptr as *mut WordSize);
        //free(ptr);
    }
}
