use crate::block::{Block, BlockReuseFn, DataPointer, ObjectHeader, UserData, WordSize};
use crate::heap::{HEAP_START, HEAP_TOP};
use std::{mem, ptr};

#[cfg(test)]
mod tests;

/// Allocates a block of memory of (at least) `size` bytes.
pub fn alloc(size: usize, find_block: BlockReuseFn) -> DataPointer {
    let size = align(size);

    if let Some(block) = find_block(size) {
        return from(block);
    }

    let mut block = match request_from_os(size) {
        Some(block) => block,
        None => panic!("OOM!"),
    };

    unsafe {
        let header = ObjectHeader {
            size,
            used: true,
            next: ptr::null_mut(),
        };
        (*block).header = header;

        // Init heap
        if HEAP_START.is_null() {
            HEAP_START = block;
        }

        // Chain the blocks
        if !HEAP_TOP.is_null() {
            (*HEAP_TOP).header.next = block;
        }

        HEAP_TOP = block;
    }
    // User payload
    from(block)
}

/// Frees a previously allocated block.
pub fn free(data: DataPointer) {
    let block = get_header(data);
    unsafe { (*block).header.used = false };
}

/// Get a pointer to data of a block.
fn from(block: *mut Block) -> DataPointer {
    unsafe { &mut (*block).data[0] }
}

/// Aligns the size by the machine word.
#[inline]
fn align(size: usize) -> usize {
    (size + mem::size_of::<WordSize>() - 1) & !(mem::size_of::<WordSize>() - 1)
}

/// Returns total allocation size, reserving in addition the space for
/// the Block structure (object header + first data word).
///
/// Since the UserData already allocates one byte inside the Block
/// structure, we decrease it from the size request: if a user allocates
/// only one byte, it's fully in the Block struct.
#[inline]
fn alloc_size(size: usize) -> libc::intptr_t {
    (size + mem::size_of::<Block>() - mem::size_of::<UserData>()) as libc::intptr_t
}

/// Requests (maps) memory from OS.
fn request_from_os(size: usize) -> Option<*mut Block> {
    let block = unsafe { libc::sbrk(0) as *mut Block };

    if unsafe { libc::sbrk(alloc_size(size)) } == -1_isize as *mut libc::c_void {
        return None;
    }

    Some(block)
}

/// Returns the object header.
fn get_header(data: DataPointer) -> *mut Block {
    (data as usize - mem::size_of::<ObjectHeader>()) as *mut Block
}
