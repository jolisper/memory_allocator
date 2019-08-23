use crate::block::Block;
use std::ptr;

/// Heap start. Initialized on first allocation.
pub(crate) static mut HEAP_START: *mut Block = ptr::null_mut();
/// Current top. Updated on each allocation.
pub(crate) static mut HEAP_TOP: *mut Block = unsafe { HEAP_START };
