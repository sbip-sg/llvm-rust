//! Module handling to the `terminator` instructions of LLVM.
//!
//! A `terminator` instruction is one of the following instructions: `ret`,
//! `br`, `switch`, `indirectbr`, `invoke`, `callbr` `resume`, `catchswitch`,
//! `catchret`, `cleanupret`, and `unreachable`.

use std::convert::TryFrom;
use std::fmt::{self, Display};

use crate::cfg::SuccessorBlock;
use crate::values::{AnyValue, AsValueRef, BasicValueEnum, InstructionValue};
use llvm_sys::{
    core::{
        LLVMGetCondition, LLVMGetNumSuccessors, LLVMGetSuccessor,
        LLVMIsConditional,
    },
    prelude::LLVMValueRef,
};

use super::{
    AnyCondition, AnyInstruction, AnyTerminator, AsInstructionValue,
    BranchInst, IndirectBrInst, InvokeInst, ReturnInst, SwitchInst,
    UnreachableInst,
};

/// Data structure modelling a `terminator` instruction.
///
/// A `terminator` instruction is one of the following instructions: `ret`,
/// `br`, `switch`, `indirectbr`, `invoke`, `callbr` `resume`, `catchswitch`,
/// `catchret`, `cleanupret`, and `unreachable`.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct TerminatorInst<'ctx> {
    terminator_inst: InstructionValue<'ctx>,
}

/// Implement methods for `TerminatorInst`.
impl<'ctx> TerminatorInst<'ctx> {
    /// Constructor of a `TerminatorInst`
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_terminator_inst());
        TerminatorInst {
            terminator_inst: inst,
        }
    }

    /// Convert to `BranchInst`.
    pub fn as_branch_inst(&self) -> Option<BranchInst<'ctx>> {
        self.terminator_inst.try_into_branch_inst()
    }

    /// Convert to `IndirectBrInst`.
    pub fn as_indirectbr_inst(&self) -> Option<IndirectBrInst<'ctx>> {
        self.terminator_inst.try_into_indirectbr_inst()
    }

    /// Convert to `InvokeInst`.
    pub fn as_invoke_inst(&self) -> Option<InvokeInst<'ctx>> {
        self.terminator_inst.try_into_invoke_inst()
    }

    /// Convert to `ReturnInst`.
    pub fn as_return_inst(&self) -> Option<ReturnInst<'ctx>> {
        self.terminator_inst.try_into_return_inst()
    }

    /// Convert to `SwitchInst`.
    pub fn as_switch_inst(&self) -> Option<SwitchInst<'ctx>> {
        self.terminator_inst.try_into_switch_inst()
    }

    /// Convert to `UnreachableInst`.
    pub fn as_unreachable_inst(&self) -> Option<UnreachableInst<'ctx>> {
        self.terminator_inst.try_into_unreachable_inst()
    }

    /// Get successor block
    pub fn get_conditioned_successors(&self) -> Vec<SuccessorBlock<'ctx>> {
        if let Some(branch_inst) = self.as_branch_inst() {
            branch_inst.get_conditioned_successors()
        } else if let Some(indirectbr_inst) = self.as_indirectbr_inst() {
            indirectbr_inst.get_conditioned_successors()
        } else if let Some(switch_inst) = self.as_switch_inst() {
            switch_inst.get_conditioned_successors()
        } else {
            vec![]
        }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::TerminatorInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `TerminatorInst`.
    impl<'ctx> AsInstructionValue<'ctx> for TerminatorInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.terminator_inst
        }
    }

    /// Implement the `AsValueRef` trait for `TerminatorInst`.
    impl<'ctx> AsValueRef for TerminatorInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.terminator_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::TerminatorInst;
    use crate::values::instructions::{
        AnyCondition, AnyInstruction, AnyTerminator,
    };

    /// Implement the `AnyInstruction` trait for `TerminatorInst`.
    impl<'ctx> AnyInstruction<'ctx> for TerminatorInst<'ctx> {}

    /// Implement the `AnyTerminator` trait for `TerminatorInst`.
    impl<'ctx> AnyTerminator<'ctx> for TerminatorInst<'ctx> {}

    /// Implement the `AnyCondition` trait for `TerminatorInst`.
    impl<'ctx> AnyCondition<'ctx> for TerminatorInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::TerminatorInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `TerminatorInst`.
    impl<'ctx> Display for TerminatorInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `TerminatorInst`.
    impl<'ctx> Clone for TerminatorInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                terminator_inst: self.terminator_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `TerminatorInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for TerminatorInst<'ctx> {
        type Error = ();

        fn try_from(
            value: InstructionValue<'ctx>,
        ) -> Result<Self, Self::Error> {
            if value.is_a_terminator_inst() {
                unsafe { Ok(TerminatorInst::new(value)) }
            } else {
                Err(())
            }
        }
    }
}
