//! Module handling to the `return` instruction of LLVM.

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};
use either::Either::{Left, Right};
use inkwell::values::{AnyValue, AsValueRef, BasicValueEnum, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `return` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ReturnInst<'ctx> {
    /// Instruction value corresponding to the `ReturnInst`.
    return_inst: InstructionValue<'ctx>,
}

/// Implement methods for `ReturnInst`.
impl<'ctx> ReturnInst<'ctx> {
    /// Constructor of a `ReturnInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_return_inst());
        ReturnInst { return_inst: inst }
    }

    /// Get the returned value of the current `ReturnInst`.
    ///
    /// Return `None` if the instruction does not return any value.
    pub fn get_returned_value(&self) -> Option<BasicValueEnum<'ctx>> {
        match self.get_operand(0) {
            None => None,
            Some(opr) => match opr {
                Left(v) => Some(v),
                Right(_) => panic!("Invalid return instruction: {}", self),
            },
        }
    }
}

/// Implement the `AsInstructionValue` trait for `ReturnInst`.
impl<'ctx> AsInstructionValue<'ctx> for ReturnInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.return_inst
    }
}

/// Implement the `AsValueRef` trait for `ReturnInst`.
impl<'ctx> AsValueRef for ReturnInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.return_inst.as_value_ref()
    }
}

/// Implement the `AnyTerminator` trait for `ReturnInst`.
impl<'ctx> AnyTerminator<'ctx> for ReturnInst<'ctx> {}

/// Implement the `AnyInstruction` trait for `ReturnInst`.
impl<'ctx> AnyInstruction<'ctx> for ReturnInst<'ctx> {}

/// Implement the `AnyValue` trait for `ReturnInst`.
impl<'ctx> AnyValue<'ctx> for ReturnInst<'ctx> {}

/// Implement the `Display` trait for `ReturnInst`.
impl<'ctx> Display for ReturnInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `ReturnInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for ReturnInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_return_inst() {
            Ok(ReturnInst::new(inst))
        } else {
            Err(())
        }
    }
}
