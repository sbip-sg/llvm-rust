//! Module handling to the `store` instruction of LLVM.

use super::{AnyInstruction, AsInstructionValue};
use either::Either::Left;
use inkwell::values::{
    AnyValue, AsValueRef, BasicValueEnum, InstructionValue, PointerValue,
};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `store` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct StoreInst<'ctx> {
    /// Instruction value corresponding to the `StoreInst`.
    store_inst: InstructionValue<'ctx>,
}

/// Implement methods for `StoreInst`.
impl<'ctx> StoreInst<'ctx> {
    /// Constructor of a `StoreInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_store_inst());
        StoreInst { store_inst: inst }
    }

    /// Get the value operand of the current `StoreInst`.
    pub fn get_value_operand(&self) -> BasicValueEnum<'ctx> {
        if let Some(Left(v)) = self.get_operand(0) {
            return v;
        }

        panic!("Invalid Store instruction: {}", self)
    }

    /// Get the pointer operand of the current `StoreInst`.
    pub fn get_pointer_operand(&self) -> PointerValue<'ctx> {
        if let Some(Left(v)) = self.get_operand(1) {
            if v.is_pointer_value() {
                return v.into_pointer_value();
            }
        }

        panic!("Invalid Store instruction: {}", self)
    }
}

/// Implement the `AsInstructionValue` trait for `StoreInst`.
impl<'ctx> AsInstructionValue<'ctx> for StoreInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.store_inst
    }
}

/// Implement the `AsValueRef` trait for `StoreInst`.
impl<'ctx> AsValueRef for StoreInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.store_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `StoreInst`.
impl<'ctx> AnyInstruction<'ctx> for StoreInst<'ctx> {}

/// Implement the `AnyValue` trait for `StoreInst`.
impl<'ctx> AnyValue<'ctx> for StoreInst<'ctx> {}

/// Implement the `Display` trait for `StoreInst`.
impl<'ctx> Display for StoreInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `StoreInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for StoreInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_store_inst() {
            Ok(StoreInst::new(inst))
        } else {
            Err(())
        }
    }
}
