//! A learning purpose memory allocator that follows
//! "[writing a memory allocator](http://dmitrysoshnikov.com/compilers/writing-a-memory-allocator/)" exercise.

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod allocator;
mod block;
mod heap;
mod malloc;

extern crate libc;

pub use allocator::FirstFit;
pub use block::Block;
pub use malloc::{alloc, free};
