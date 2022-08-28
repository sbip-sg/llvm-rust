//! Module handling to the `br` instruction of LLVM.

use std::fmt::{self, Display};

use crate::ir::{PathCondition, SuccessorBlock};

use super::{AnyCondition, AnyInstruction, AnyTerminator, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, BasicBlock, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;

/// Data structure modelling a `br` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BranchInst<'ctx> {
    /// Instruction value corresponding to the `BranchInst`.
    branch_inst: InstructionValue<'ctx>,
}

/// Implement methods for `BranchInst`.
impl<'ctx> BranchInst<'ctx> {
    /// Constructor of a `BranchInst` instruction.
    pub(crate) fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_branch_inst());
        BranchInst { branch_inst: inst }
    }

    /// Get the first successor
    pub fn get_first_successor(&self) -> BasicBlock<'ctx> {
        self.get_successor(0).unwrap()
    }

    /// Get the second successor if this is a conditional `BranchInst`.
    pub fn get_second_successor(&self) -> Option<BasicBlock<'ctx>> {
        self.get_successor(1)
    }

    /// Get all successor blocks with path conditions.
    pub fn get_conditioned_successors(&self) -> Vec<SuccessorBlock<'ctx>> {
        let mut successors = vec![];

        if self.has_condition() {
            let condition = self.get_condition();
            let sblk1 = SuccessorBlock::new(
                PathCondition::Boolean(condition, true),
                self.get_first_successor(),
            );
            let sblk2 = SuccessorBlock::new(
                PathCondition::Boolean(condition, false),
                self.get_second_successor().unwrap(),
            );
            successors.push(sblk1);
            successors.push(sblk2);
        } else {
            let path_cond = PathCondition::None;
            let blk = self.get_successor(0).unwrap();
            let sblk = SuccessorBlock::new(path_cond, blk);
            successors.push(sblk);
        }

        successors
    }
}

/// Implement the `AsInstructionValue` trait for `BranchInst`.
impl<'ctx> AsInstructionValue<'ctx> for BranchInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.branch_inst
    }
}

/// Implement the `AsValueRef` trait for `BranchInst`.
impl<'ctx> AsValueRef for BranchInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.branch_inst.as_value_ref()
    }
}

/// Implement the `AnyTerminator` trait for `BranchInst`.
impl<'ctx> AnyTerminator<'ctx> for BranchInst<'ctx> {}

/// Implement the `AnyCondition` trait for `BranchInst`.
impl<'ctx> AnyCondition<'ctx> for BranchInst<'ctx> {}

/// Implement the `AnyInstruction` trait for `BranchInst`.
impl<'ctx> AnyInstruction<'ctx> for BranchInst<'ctx> {}

/// Implement the `AnyValue` trait for `BranchInst`.
impl<'ctx> AnyValue<'ctx> for BranchInst<'ctx> {}

/// Implement the `Display` trait for `BranchInst`.
impl<'ctx> Display for BranchInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `BranchInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for BranchInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_branch_inst() {
            Ok(BranchInst::new(inst))
        } else {
            Err(())
        }
    }
}
