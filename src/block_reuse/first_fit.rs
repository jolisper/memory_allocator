use crate::block_reuse::Algorithm;
use crate::{Block, HEAP_START};

/// First-fit algorithm allocator
#[derive(Default)]
pub struct FirstFit;

impl Algorithm for FirstFit {
    unsafe fn find_block(&self, size: usize) -> Option<*mut Block> {
        let mut block: *mut Block = HEAP_START;

        while !block.is_null() {
            // O(n) search
            if (*block).used || (*block).size < size {
                block = (*block).next;
                continue;
            }
            // Block found
            return Some(block);
        }

        None
    }
}
