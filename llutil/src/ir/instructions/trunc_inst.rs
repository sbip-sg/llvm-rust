//! Module handling to the `trunc` instruction of LLVM.

use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyCast, AnyInstruction, AsInstructionValue};

/// Data structure modelling a `trunc` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TruncInst<'ctx> {
    /// Instruction value corresponding to the `TruncInst`.
    zext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `TruncInst`.
impl<'ctx> TruncInst<'ctx> {
    /// Constructor of a `TruncInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_zext_inst());
        TruncInst { zext_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `TruncInst`.
impl<'ctx> AsInstructionValue<'ctx> for TruncInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.zext_inst
    }
}

/// Implement the `AsValueRef` trait for `TruncInst`.
impl<'ctx> AsValueRef for TruncInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.zext_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `TruncInst`.
impl<'ctx> AnyInstruction<'ctx> for TruncInst<'ctx> {}

/// Implement the `AnyCast` trait for `TruncInst`.
impl<'ctx> AnyCast<'ctx> for TruncInst<'ctx> {}

/// Implement the `AnyValue` trait for `TruncInst`.
impl<'ctx> AnyValue<'ctx> for TruncInst<'ctx> {}

/// Implement the `Display` trait for `TruncInst`.
impl<'ctx> Display for TruncInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `TruncInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for TruncInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_zext_inst() {
            Ok(TruncInst::new(inst))
        } else {
            Err(())
        }
    }
}
