mod first_fit;

pub use first_fit::FIRST_FIT;

use crate::block::Block;

/// Allocator algorithms interface.
pub type BlockReuseFn = fn(usize) -> Option<*mut Block>;
