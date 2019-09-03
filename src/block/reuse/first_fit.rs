use crate::block::{reuse::BlockReuseFn, Block};
use crate::heap::HEAP_START;

/// First-fit algorithm allocator.
pub static FIRST_FIT: BlockReuseFn = find_block;

fn find_block(size: usize) -> Option<*mut Block> {
    let mut block: *mut Block = unsafe { HEAP_START };
    while !block.is_null() {
        unsafe {
            // O(n) search
            if (*block).header.used || (*block).header.size < size {
                block = (*block).header.next;
                continue;
            }
        }
        // Block found
        return Some(block);
    }
    None
}
