pub mod first_fit;

use crate::Block;

/// Allocator algorithms interface
pub(crate) trait Algorithm {
    unsafe fn find_block(&self, size: usize) -> Option<*mut Block>;
}
