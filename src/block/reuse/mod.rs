mod first_fit;

use crate::block::Block;

/// Allocator algorithms interface
pub trait ReuseAlgorithm {
    fn find_block(&self, size: usize) -> Option<*mut Block>;
}

pub enum ReuseAlgorithmBuilder {
    FirstFit,
}

impl ReuseAlgorithmBuilder {
    pub fn build(variant: ReuseAlgorithmBuilder) -> impl ReuseAlgorithm {
        match variant {
            ReuseAlgorithmBuilder::FirstFit => first_fit::FirstFit::default(),
        }   
    }
}
