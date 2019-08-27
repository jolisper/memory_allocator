use crate::{alloc, free, Block};
use std::alloc::{GlobalAlloc, Layout};

/// Rust memory allocator using the mem alloc
#[derive(Debug)]
pub struct RustAllocator;

unsafe impl GlobalAlloc for RustAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        alloc(size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        #[allow(clippy::cast_ptr_alignment)]
        let block = ptr as *mut Block;
        free(&mut (*block).data[0]);
    }
}
