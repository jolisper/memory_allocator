pub(crate) mod reuse;

pub(crate) use reuse::{ReuseAlgorithm, ReuseAlgorithmBuilder};

/// Machine word size. Depending on the architecture,
/// can be 4 or 8 bytes.
pub type WordSize = u8;
/// Data pointer type.
pub type DataPointer = [WordSize; 1];

/// Headers.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    /// Block size
    pub size: libc::size_t,

    /// Whether this block is currently used
    pub used: bool,

    /// Next block in the list
    pub next: *mut Block,
}

/// Allocated block of memory. Contains the object structure,
/// and the actual payload pointer.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Block {
    // -------- Object Header -----------
    /// Object header
    pub header: ObjectHeader,

    // --------- User Data --------------
    /// Payload pointer
    pub data: DataPointer,
}
