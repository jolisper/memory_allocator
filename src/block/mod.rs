mod reuse;

pub use reuse::{BlockReuseFn, FIRST_FIT};

/// Machine word size. Depending on the architecture,
/// can be 4 or 8 bytes.
pub type WordSize = isize;
/// Data type.
pub type UserData = [u8; 1];
/// Data pointer.
pub type DataPointer = *mut u8;

/// Headers.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ObjectHeader {
    /// Block size.
    pub size: usize,

    /// Whether this block is currently used.
    pub used: bool,

    /// Next block in the list.
    pub next: *mut Block,
}

/// Allocated block of memory. Contains the object structure,
/// and the actual user data.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Block {
    // -------- Object Header -----------
    /// Object header part.
    pub header: ObjectHeader,

    // --------- User Data --------------
    /// User data part.
    pub data: UserData,
}
