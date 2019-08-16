//! A learning purpose memory allocator that follows 
//! "[writing a memory allocator](http://dmitrysoshnikov.com/compilers/writing-a-memory-allocator/)" exercise.

#![deny(missing_docs,
        missing_debug_implementations, 
        trivial_casts, 
        trivial_numeric_casts,
        unstable_features,
        unused_import_braces, 
        unused_qualifications)]

#[cfg(test)]
mod tests;

extern crate libc;

use std::{mem, ptr};

/// Allocated block of memory. Contains the object structure,
/// and the actual payload pointer.
#[repr(C)]
#[derive(Debug)]
pub struct Block {
// -------- Object Header -----------
    /// Block size
    pub size: libc::size_t,

    /// Whether this block is currently used
    pub used: bool,

    /// Next block in the list
    pub next: *mut Block,

// --------- User Data --------------
    /// Payload pointer
    pub data: DataPointer,
}

/// Heap start. Initialized on first allocation.
static mut HEAP_START: *mut Block = ptr::null_mut();
/// Current top. Updated on each allocation.
static mut HEAP_TOP: *mut Block = unsafe { HEAP_START };

/// Machine word size. Depending on the architecture,
/// can be 4 or 8 bytes.
pub type WordSize = libc::intptr_t;
/// Data pointer type.
pub type DataPointer = [WordSize; 1];

/// Allocates a block of memory of (at least) `size` bytes.
pub fn alloc(size: usize) -> *mut WordSize {
    let size = align(size);

    if let Some(block) = find_block(size) {
        return from(block);
    }

    let mut block = match request_from_os(size) {
        Some(block) => block,
        None => panic!("OOM!"),
    };

    unsafe {
        (*block).size = size;
        (*block).used = true;

        // Init heap
        if HEAP_START.is_null() {
            HEAP_START = block;    
        }

        // Chain the blocks
        if !HEAP_TOP.is_null() {
            (*HEAP_TOP).next = block;
        }

        HEAP_TOP = block;
    }

    // User payload
    from(block)
}

/// Frees a previously allocated block.
pub fn free(data: *mut WordSize) {
    let block = get_header(data);
    unsafe { (*block).used = false };
}

/// Tries to find a block of a needed size.
fn find_block(_size: usize) -> Option<*mut Block> {
    None
}

/// Get a pointer to data of a block.
fn from(block: *mut Block) -> *mut WordSize {
    let p: *mut WordSize = unsafe { &mut (*block).data[0] };
    p
}

/// Aligns the size by the machine word.
#[inline]
fn align(n: libc::size_t) -> libc::size_t {
    (n + mem::size_of::<WordSize>() - 1) & !(mem::size_of::<WordSize>() - 1)
}

/// Returns total allocation size, reserving in addition the space for
/// the Block structure (object header + first data word).
/// 
/// Since the `data: DataPointer` already allocates one word inside the Block
/// structure, we decrease it from the size request: if a user allocates
/// only one word, it's fully in the Block struct.
#[inline]
fn alloc_size(size: libc::size_t) -> libc::intptr_t {
    (size + mem::size_of::<Block>() - mem::size_of::<DataPointer>()) as libc::intptr_t
}

/// Requests (maps) memory from OS.
fn request_from_os(size: usize) -> Option<*mut Block> {
    let block = unsafe { libc::sbrk(0) as (*mut Block) };

    if unsafe { libc::sbrk(alloc_size(size)) } == -1_isize as *mut libc::c_void {
        return None;
    } 

    Some(block)
}

/// Returns the object header.
pub fn get_header(data: *mut WordSize) -> *mut Block {
    (data as usize + mem::size_of::<DataPointer>() - mem::size_of::<Block>()) as *mut Block
}