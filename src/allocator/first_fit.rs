use crate::{alloc, block::FIRST_FIT, free, Block};
use std::alloc::{GlobalAlloc, Layout};

/// Rust memory allocator using the mem alloc
#[derive(Debug)]
pub struct FirstFit;

unsafe impl GlobalAlloc for FirstFit {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        alloc(size, FIRST_FIT)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        #[allow(clippy::cast_ptr_alignment)]
        let block = ptr as *mut Block;
        free(&mut (*block).data[0]);
    }
}
