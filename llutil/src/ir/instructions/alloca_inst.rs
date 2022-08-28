//! Module handling to the `callbr` instruction of LLVM.

use super::{AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display, Formatter};

/// Data structure modelling a `alloca` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct AllocaInst<'ctx> {
    /// Instruction value corresponding to the `AllocaInst`.
    alloca_inst: InstructionValue<'ctx>,
}

impl<'ctx> AllocaInst<'ctx> {
    /// Constructor of a `AllocaInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_alloca_inst());
        AllocaInst { alloca_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `AllocaInst`.
impl<'ctx> AsInstructionValue<'ctx> for AllocaInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.alloca_inst
    }
}

/// Implement the `AsValueRef` trait for `AllocaInst`.
impl<'ctx> AsValueRef for AllocaInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.alloca_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `AllocaInst`.
impl<'ctx> AnyInstruction<'ctx> for AllocaInst<'ctx> {}

/// Implement the `AnyValue` trait for `AllocaInst`.
impl<'ctx> AnyValue<'ctx> for AllocaInst<'ctx> {}

/// Implement the `Display` trait for `AllocaInst`.
impl<'ctx> Display for AllocaInst<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `AllocaInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for AllocaInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_alloca_inst() {
            Ok(AllocaInst::new(inst))
        } else {
            Err(())
        }
    }
}
