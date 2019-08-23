use crate::block::{reuse::ReuseAlgorithm, Block};
use crate::heap::HEAP_START;

/// First-fit algorithm allocator
#[derive(Default)]
pub struct FirstFit;

impl ReuseAlgorithm for FirstFit {
    fn find_block(&self, size: usize) -> Option<*mut Block> {
        let mut block: *mut Block = unsafe { HEAP_START };
        while !block.is_null() {
            unsafe {
                // O(n) search
                if (*block).used || (*block).size < size {
                    block = (*block).next;
                    continue;
                }
            }
            // Block found
            return Some(block);
        }
        None
    }
}
