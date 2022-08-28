//! Module handling to the `unreachable` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `unreachable` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct UnreachableInst<'ctx> {
    /// Instruction value corresponding to the `UnreachableInst`.
    unreachable_inst: InstructionValue<'ctx>,
}

/// Implement methods for `UnreachableInst`.
impl<'ctx> UnreachableInst<'ctx> {
    /// Constructor of a `UnreachableInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_unreachable_inst());
        UnreachableInst {
            unreachable_inst: inst,
        }
    }
}

/// Implement the `AsInstructionValue` trait for `UnreachableInst`.
impl<'ctx> AsInstructionValue<'ctx> for UnreachableInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.unreachable_inst
    }
}

/// Implement the `AsValueRef` trait for `UnreachableInst`.
impl<'ctx> AsValueRef for UnreachableInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.unreachable_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `UnreachableInst`.
impl<'ctx> AnyInstruction<'ctx> for UnreachableInst<'ctx> {}

/// Implement the `AnyCall` trait for `UnreachableInst`.
impl<'ctx> AnyCall<'ctx> for UnreachableInst<'ctx> {}

/// Implement the `AnyValue` trait for `UnreachableInst`.
impl<'ctx> AnyValue<'ctx> for UnreachableInst<'ctx> {}

/// Implement the `Display` trait for `UnreachableInst`.
impl<'ctx> Display for UnreachableInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `UnreachableInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for UnreachableInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_unreachable_inst() {
            Ok(UnreachableInst::new(inst))
        } else {
            Err(())
        }
    }
}
