//! Module handling to the `sext` instruction of LLVM.

use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyCast, AnyInstruction, AsInstructionValue};

/// Data structure modelling a `sext` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct SExtInst<'ctx> {
    /// Instruction value corresponding to the `SExtInst`.
    sext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `SExtInst`.
impl<'ctx> SExtInst<'ctx> {
    /// Constructor of a `SExtInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_sext_inst());
        SExtInst { sext_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `SExtInst`.
impl<'ctx> AsInstructionValue<'ctx> for SExtInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.sext_inst
    }
}

/// Implement the `AsValueRef` trait for `SExtInst`.
impl<'ctx> AsValueRef for SExtInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.sext_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `SExtInst`.
impl<'ctx> AnyInstruction<'ctx> for SExtInst<'ctx> {}

/// Implement the `AnyCast` trait for `SExtInst`.
impl<'ctx> AnyCast<'ctx> for SExtInst<'ctx> {}

/// Implement the `AnyValue` trait for `SExtInst`.
impl<'ctx> AnyValue<'ctx> for SExtInst<'ctx> {}

/// Implement the `Display` trait for `SExtInst`.
impl<'ctx> Display for SExtInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `SExtInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for SExtInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_sext_inst() {
            Ok(SExtInst::new(inst))
        } else {
            Err(())
        }
    }
}
