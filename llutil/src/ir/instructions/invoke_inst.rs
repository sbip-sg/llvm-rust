//! Module handling to the `invoke` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `invoke` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct InvokeInst<'ctx> {
    /// Instruction value corresponding to the `InvokeInst`.
    invoke_inst: InstructionValue<'ctx>,
}

/// Implement methods for `InvokeInst`.
impl<'ctx> InvokeInst<'ctx> {
    /// Constructor of a `InvokeInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_invoke_inst());
        InvokeInst { invoke_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `InvokeInst`.
impl<'ctx> AsInstructionValue<'ctx> for InvokeInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.invoke_inst
    }
}

/// Implement the `AsValueRef` trait for `InvokeInst`.
impl<'ctx> AsValueRef for InvokeInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.invoke_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `InvokeInst`.
impl<'ctx> AnyInstruction<'ctx> for InvokeInst<'ctx> {}

/// Implement the `AnyCall` trait for `InvokeInst`.
impl<'ctx> AnyCall<'ctx> for InvokeInst<'ctx> {}

/// Implement the `AnyValue` trait for `InvokeInst`.
impl<'ctx> AnyValue<'ctx> for InvokeInst<'ctx> {}

/// Implement the `Display` trait for `InvokeInst`.
impl<'ctx> Display for InvokeInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `InvokeInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for InvokeInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_invoke_inst() {
            Ok(InvokeInst::new(inst))
        } else {
            Err(())
        }
    }
}
