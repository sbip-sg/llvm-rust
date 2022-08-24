//! Module handling successor blocks of an LLVM `BasicBlock`.

use crate::values::{BasicBlock, BasicValueEnum};

use super::PathCondition;

/// Data structure representing a successor block and its path condition from
/// the current `BasicBlock`.
#[derive(Debug)]
pub struct SuccessorBlock<'ctx> {
    /// Path condition leading from this successor block.
    pub condition: PathCondition<'ctx>,

    /// The successor block.
    pub block: BasicBlock<'ctx>,
}

/// Implement functionalities handling successor blocks.
impl<'ctx> SuccessorBlock<'ctx> {
    /// Constructor
    pub fn new(
        condition: PathCondition<'ctx>,
        successor: BasicBlock<'ctx>,
    ) -> SuccessorBlock<'ctx> {
        SuccessorBlock {
            condition,
            block: successor,
        }
    }

    /// Get name of the current successor block or return a default name.
    pub fn get_name_or_default(&self) -> String {
        self.block.get_name_or_default()
    }
}
