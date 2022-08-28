//! Module handling to a function call instruction of LLVM.
//!
//! The function call instruction can be `call`, `callbr`, or `invoke`.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display};

/// Data structure modelling a function call instruction.
///
/// The function call instruction can be `call`, `callbr`, or `invoke`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CallBase<'ctx> {
    /// Instruction value corresponding to the `CallBase`.
    call_base: InstructionValue<'ctx>,
}

/// Implement methods for `CallBase`.
impl<'ctx> CallBase<'ctx> {
    /// Constructor of a `CallBase` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_call_base());
        CallBase { call_base: inst }
    }
}

/// Implement the `AsInstructionValue` trait for `CallBase`.
impl<'ctx> AsInstructionValue<'ctx> for CallBase<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.call_base
    }
}

/// Implement the `AsValueRef` trait for `CallBase`.
impl<'ctx> AsValueRef for CallBase<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.call_base.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `CallBase`.
impl<'ctx> AnyInstruction<'ctx> for CallBase<'ctx> {}

/// Implement the `AnyCall` trait for `CallBase`.
impl<'ctx> AnyCall<'ctx> for CallBase<'ctx> {}

/// Implement the `AnyValue` trait for `CallBase`.
impl<'ctx> AnyValue<'ctx> for CallBase<'ctx> {}

/// Implement the `Display` trait for `CallBase`.
impl<'ctx> Display for CallBase<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `CallBase`x.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for CallBase<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_call_base() {
            Ok(CallBase::new(inst))
        } else {
            Err(())
        }
    }
}
