//! Module handling to the `indirectbr` instruction of LLVM.

use crate::ir::{PathCondition, SuccessorBlock};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `indirectbr` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct IndirectBrInst<'ctx> {
    /// Instruction value corresponding to the `IndirectBrInst`.
    indirectbr_inst: InstructionValue<'ctx>,
}

/// Implement methods for `IndirectBrInst`.
impl<'ctx> IndirectBrInst<'ctx> {
    /// Constructor of a `IndirectBrInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_indirectbr_inst());
        IndirectBrInst {
            indirectbr_inst: inst,
        }
    }

    /// Get all successor blocks with path conditions.
    pub fn get_conditioned_successors(&self) -> Vec<SuccessorBlock<'ctx>> {
        let mut successors = vec![];

        for blk in self.get_successors() {
            // FIXME: check if this condition is correct?
            let path_cond = PathCondition::None;
            let sblk = SuccessorBlock::new(path_cond, blk);
            successors.push(sblk);
        }

        successors
    }
}

/// Implement the `AsInstructionValue` trait for `IndirectBrInst`.
impl<'ctx> AsInstructionValue<'ctx> for IndirectBrInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.indirectbr_inst
    }
}

/// Implement the `AsValueRef` trait for `IndirectBrInst`.
impl<'ctx> AsValueRef for IndirectBrInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.indirectbr_inst.as_value_ref()
    }
}

/// Implement the `AnyTerminator` trait for `IndirectBrInst`.
impl<'ctx> AnyTerminator<'ctx> for IndirectBrInst<'ctx> {}

/// Implement the `AnyInstruction` trait for `IndirectBrInst`.
impl<'ctx> AnyInstruction<'ctx> for IndirectBrInst<'ctx> {}

/// Implement the `AnyValue` trait for `IndirectBrInst`.
impl<'ctx> AnyValue<'ctx> for IndirectBrInst<'ctx> {}

/// Implement the `Display` trait for `IndirectBrInst`.
impl<'ctx> Display for IndirectBrInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `IndirectBrInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for IndirectBrInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_indirectbr_inst() {
            Ok(IndirectBrInst::new(inst))
        } else {
            Err(())
        }
    }
}
