//! Module handling to the binary operations of LLVM.

use super::{AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, BasicValueEnum, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display};

/// Data structure modelling a binary operation.
///
/// A binary operation is one of the following instructions: `add`, `fadd`,
/// `sub`, `fsub`, `mul`, `fmul` `udiv`, `sdiv`, `fdiv`, `urem`, `srem`, `frem`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BinaryOperator<'ctx> {
    /// Instruction value corresponding to the `BinaryOperator`.
    binary_operator: InstructionValue<'ctx>,
}

/// Implement methods for `BinaryOperator`.
impl<'ctx> BinaryOperator<'ctx> {
    /// Constructor of a `BinaryOperator`
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_binary_operator());
        BinaryOperator {
            binary_operator: inst,
        }
    }

    /// Get the first operand of the binary operation.
    pub fn get_first_operand(&self) -> BasicValueEnum<'ctx> {
        match self.get_operand(0) {
            None => panic!(
                "Invalid binary operator: {}\n{}",
                self, "Unable to get the first operand!"
            ),
            Some(opr) => match opr.left() {
                None => panic!(
                    "Invalid binary operator: {}\n{}",
                    self, "Unable to get the first operand!"
                ),
                Some(v) => v,
            },
        }
    }

    /// Get the second operand of the binary operation.
    pub fn get_second_operand(&self) -> BasicValueEnum<'ctx> {
        match self.get_operand(1) {
            None => panic!(
                "Invalid binary operator: {}\n{}",
                self, "Unable to get the second operand!"
            ),
            Some(opr) => match opr.left() {
                None => panic!(
                    "Invalid binary operator: {}\n{}",
                    self, "Unable to get the second operand!"
                ),
                Some(v) => v,
            },
        }
    }

    /// Check if the current binary operator has the `NoUnSignedWrap` (NUW) flag.
    pub fn has_no_unsigned_wrap(&self) -> bool {
        self.binary_operator.has_no_unsigned_wrap()
    }

    /// Check if the current binary operator has the `NoSignedWrap` (NSW) flag.
    pub fn has_no_signed_wrap(&self) -> bool {
        self.binary_operator.has_no_signed_wrap()
    }
}

/// Implement the `AsInstructionValue` trait for `BinaryOperator.`
impl<'ctx> AsInstructionValue<'ctx> for BinaryOperator<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.binary_operator
    }
}

/// Implement the `AsValueRef` trait for `BinaryOperator.`
impl<'ctx> AsValueRef for BinaryOperator<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.binary_operator.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `BinaryOperator.`
impl<'ctx> AnyInstruction<'ctx> for BinaryOperator<'ctx> {}

/// Implement the `AnyValue` trait for `BinaryOperator`.
impl<'ctx> AnyValue<'ctx> for BinaryOperator<'ctx> {}

/// Implement the `Display` trait for `BinaryOperator.`
impl<'ctx> Display for BinaryOperator<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `BinaryOperator.`
impl<'ctx> TryFrom<InstructionValue<'ctx>> for BinaryOperator<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_binary_operator() {
            Ok(BinaryOperator::new(inst))
        } else {
            Err(())
        }
    }
}
