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

mod alloc;
mod block;
mod heap;
mod rust_allocator;

extern crate libc;

pub use alloc::{alloc, free};
pub use block::Block;
pub use rust_allocator::RustAllocator;
