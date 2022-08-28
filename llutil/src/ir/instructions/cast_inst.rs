//! Module handling to the `load` instruction of LLVM.

use super::{AnyCast, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display};

/// Data structure modelling casting instructions such as `SExtInst`,
/// `ZExtInst`, etc.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CastInst<'ctx> {
    /// Instruction value corresponding to the `CallInst`.
    cast_inst: InstructionValue<'ctx>,
}

/// Implement methods for `CastInst`.
impl<'ctx> CastInst<'ctx> {
    /// Constructor of a `CastInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_cast_inst());
        CastInst { cast_inst: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `CastInst`.
impl<'ctx> AsInstructionValue<'ctx> for CastInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.cast_inst
    }
}

/// Implement the `AsValueRef` trait for `CastInst`.
impl<'ctx> AsValueRef for CastInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.cast_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `CastInst`.
impl<'ctx> AnyInstruction<'ctx> for CastInst<'ctx> {}

/// Implement the `AnyCast` trait for `CastInst`.
impl<'ctx> AnyCast<'ctx> for CastInst<'ctx> {}

/// Implement the `AnyValue` trait for `CastInst`.
impl<'ctx> AnyValue<'ctx> for CastInst<'ctx> {}

/// Implement the `Display` trait for `CastInst`.
impl<'ctx> Display for CastInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `CastInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for CastInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_cast_inst() {
            Ok(CastInst::new(inst))
        } else {
            Err(())
        }
    }
}
