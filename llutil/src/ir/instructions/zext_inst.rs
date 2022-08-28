//! Module handling to the `zext` instruction of LLVM.

use super::{AnyCast, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `zext` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ZExtInst<'ctx> {
    /// Instruction value corresponding to the `ZExtInst`.
    zext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `ZExtInst`.
impl<'ctx> ZExtInst<'ctx> {
    /// Constructor of a `ZExtInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_zext_inst());
        ZExtInst { zext_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `ZExtInst`.
impl<'ctx> AsInstructionValue<'ctx> for ZExtInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.zext_inst
    }
}

/// Implement the `AsValueRef` trait for `ZExtInst`.
impl<'ctx> AsValueRef for ZExtInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.zext_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `ZExtInst`.
impl<'ctx> AnyInstruction<'ctx> for ZExtInst<'ctx> {}

/// Implement the `AnyCast` trait for `ZExtInst`.
impl<'ctx> AnyCast<'ctx> for ZExtInst<'ctx> {}

/// Implement the `AnyValue` trait for `ZExtInst`.
impl<'ctx> AnyValue<'ctx> for ZExtInst<'ctx> {}

/// Implement the `Display` trait for `ZExtInst`.
impl<'ctx> Display for ZExtInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `ZExtInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for ZExtInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_zext_inst() {
            Ok(ZExtInst::new(inst))
        } else {
            Err(())
        }
    }
}
