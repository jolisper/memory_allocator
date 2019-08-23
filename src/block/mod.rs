pub(crate) mod reuse;

pub(crate) use reuse::{ReuseAlgorithm, ReuseAlgorithmBuilder};

/// Machine word size. Depending on the architecture,
/// can be 4 or 8 bytes.
pub type WordSize = libc::intptr_t;
/// Data pointer type.
pub type DataPointer = [WordSize; 1];

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
