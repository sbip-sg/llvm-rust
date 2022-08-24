//! Module handling predecessor blocks of an LLVM `BasicBlock`.

use crate::values::{BasicBlock, BasicValueEnum};

use super::PathCondition;

/// Data structure representing a predecessor block and its path condition of
/// the current `BasicBlock`.
#[derive(Debug)]
pub struct PredecessorBlock<'ctx> {
    /// Path condition leading from this predecessor block.
    pub condition: PathCondition<'ctx>,

    /// The predecessor block.
    pub block: BasicBlock<'ctx>,
}

/// Implement functionalities handling predecessor blocks.
impl<'ctx> PredecessorBlock<'ctx> {
    /// Constructor
    pub fn new(
        condition: PathCondition<'ctx>,
        predecessor: BasicBlock<'ctx>,
    ) -> PredecessorBlock<'ctx> {
        PredecessorBlock {
            condition,
            block: predecessor,
        }
    }

    /// Get name of the current predecessor block or return a default name.
    pub fn get_name_or_default(&self) -> String {
        self.block.get_name_or_default()
    }
}
