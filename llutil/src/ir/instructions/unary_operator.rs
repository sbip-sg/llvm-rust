//! Module handling to the unary operations of LLVM.

use super::{AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a unary operation.
///
/// The current supported unary operations are: `fneg`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct UnaryOperator<'ctx> {
    /// Instruction value corresponding to the `UnaryOperator`.
    unary_operator: InstructionValue<'ctx>,
}

/// Implement methods for `UnaryOperator`.
impl<'ctx> UnaryOperator<'ctx> {
    /// Constructor of a `UnaryOperator`
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_binary_operator());
        UnaryOperator {
            unary_operator: inst,
        }
    }
}

/// Implement the `AsInstructionValue` trait for `UnaryOperator`.
impl<'ctx> AsInstructionValue<'ctx> for UnaryOperator<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.unary_operator
    }
}

/// Implement the `AsValueRef` trait for `UnaryOperator`.
impl<'ctx> AsValueRef for UnaryOperator<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.unary_operator.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `UnaryOperator`.
impl<'ctx> AnyInstruction<'ctx> for UnaryOperator<'ctx> {}

/// Implement the `AnyValue` trait for `UnaryOperator`.
impl<'ctx> AnyValue<'ctx> for UnaryOperator<'ctx> {}

/// Implement the `Display` trait for `UnaryOperator`.
impl<'ctx> Display for UnaryOperator<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `UnaryOperator`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for UnaryOperator<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_unary_operator() {
            Ok(UnaryOperator::new(inst))
        } else {
            Err(())
        }
    }
}
