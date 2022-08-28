//! Module handling to the `callbr` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display};

/// Data structure modelling a `callbr` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CallBrInst<'ctx> {
    /// Instruction value corresponding to the `CallBrInst`.
    callbr_inst: InstructionValue<'ctx>,
}

impl<'ctx> CallBrInst<'ctx> {
    /// Constructor of a `CallBrInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_callbr_inst());
        CallBrInst { callbr_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `CallBrInst`.
impl<'ctx> AsInstructionValue<'ctx> for CallBrInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.callbr_inst
    }
}

/// Implement the `AsValueRef` trait for `CallBrInst`.
impl<'ctx> AsValueRef for CallBrInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.callbr_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `CallBrInst`.
impl<'ctx> AnyInstruction<'ctx> for CallBrInst<'ctx> {}

/// Implement the `AnyCall` trait for `CallBrInst`.
impl<'ctx> AnyCall<'ctx> for CallBrInst<'ctx> {}

/// Implement the `AnyValue` trait for `CallBrInst`.
impl<'ctx> AnyValue<'ctx> for CallBrInst<'ctx> {}

/// Implement the `Display` trait for `CallBrInst`.
impl<'ctx> Display for CallBrInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `CallBrInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for CallBrInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_callbr_inst() {
            Ok(CallBrInst::new(inst))
        } else {
            Err(())
        }
    }
}
