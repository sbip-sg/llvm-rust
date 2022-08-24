//! Module handling to the unary operations of LLVM.

use std::convert::TryFrom;
use std::fmt::{self, Display};

use llvm_sys::prelude::LLVMValueRef;

use crate::values::{AnyValue, AsValueRef, InstructionValue};

use super::{AnyInstruction, AsInstructionValue};

/// Data structure modelling a unary operation.
///
/// The current supported unary operations are: `fneg`.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct UnaryOperator<'ctx> {
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

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::UnaryOperator;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

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
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::UnaryOperator;
    use crate::values::instructions::AnyInstruction;

    /// Implement the `AnyInstruction` trait for `UnaryOperator`.
    impl<'ctx> AnyInstruction<'ctx> for UnaryOperator<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::UnaryOperator;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `UnaryOperator`.
    impl<'ctx> Display for UnaryOperator<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `UnaryOperator`.
    impl<'ctx> Clone for UnaryOperator<'ctx> {
        fn clone(&self) -> Self {
            Self {
                unary_operator: self.unary_operator.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `UnaryOperator`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for UnaryOperator<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_unary_operator() {
                unsafe { Ok(UnaryOperator::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
