//! Module handling to the `br` instruction of LLVM.

use super::{AnyCondition, AnyInstruction, AnyTerminator, AsInstructionValue};
use crate::cfg::{PathCondition, SuccessorBlock};
use crate::values::{AnyValue, AsValueRef, BasicBlock, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `br` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct BranchInst<'ctx> {
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

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::BranchInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

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
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::BranchInst;
    use crate::values::instructions::{
        AnyCondition, AnyInstruction, AnyTerminator,
    };

    /// Implement the `AnyTerminator` trait for `BranchInst`.
    impl<'ctx> AnyTerminator<'ctx> for BranchInst<'ctx> {}

    /// Implement the `AnyCondition` trait for `BranchInst`.
    impl<'ctx> AnyCondition<'ctx> for BranchInst<'ctx> {}

    /// Implement the `AnyInstruction` trait for `BranchInst`.
    impl<'ctx> AnyInstruction<'ctx> for BranchInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::BranchInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `BranchInst`.
    impl<'ctx> Display for BranchInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `BranchInst`.
    impl<'ctx> Clone for BranchInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                branch_inst: self.branch_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `BranchInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for BranchInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_branch_inst() {
                unsafe { Ok(BranchInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
