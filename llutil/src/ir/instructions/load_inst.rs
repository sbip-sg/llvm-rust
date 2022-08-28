//! Module handling to the `load` instruction of LLVM.

use super::{AnyInstruction, AsInstructionValue};
use either::Either::Left;
use inkwell::values::{AnyValue, AsValueRef, InstructionValue, PointerValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `load` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct LoadInst<'ctx> {
    /// Instruction value corresponding to the `LoadInst`.
    load_inst: InstructionValue<'ctx>,
}

/// Implement methods for `LoadInst`.
impl<'ctx> LoadInst<'ctx> {
    /// Constructor of a `LoadInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_load_inst());
        LoadInst { load_inst: inst }
    }

    /// Get the pointer operand of the current `LoadInst`.
    pub fn get_pointer_operand(&self) -> PointerValue<'ctx> {
        if let Some(Left(v)) = self.get_operand(0) {
            if v.is_pointer_value() {
                return v.into_pointer_value();
            }
        }

        panic!("Invalid Load instruction: {}", self)
    }
}

/// Implement the `AsInstructionValue` trait for `LoadInst`.
impl<'ctx> AsInstructionValue<'ctx> for LoadInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.load_inst
    }
}

/// Implement the `AsValueRef` trait for `LoadInst`.
impl<'ctx> AsValueRef for LoadInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.load_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `LoadInst`.
impl<'ctx> AnyInstruction<'ctx> for LoadInst<'ctx> {}

/// Implement the `AnyValue` trait for `LoadInst`.
impl<'ctx> AnyValue<'ctx> for LoadInst<'ctx> {}

/// Implement the `Display` trait for `LoadInst`.
impl<'ctx> Display for LoadInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `LoadInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for LoadInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_load_inst() {
            Ok(LoadInst::new(inst))
        } else {
            Err(())
        }
    }
}
