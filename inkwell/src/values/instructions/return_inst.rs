//! Module handling to the `return` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyCall, AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `return` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct ReturnInst<'ctx> {
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

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::ReturnInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

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
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::ReturnInst;
    use crate::values::instructions::{AnyInstruction, AnyTerminator};

    /// Implement the `AnyTerminator` trait for `ReturnInst`.
    impl<'ctx> AnyTerminator<'ctx> for ReturnInst<'ctx> {}

    /// Implement the `AnyInstruction` trait for `ReturnInst`.
    impl<'ctx> AnyInstruction<'ctx> for ReturnInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::ReturnInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `ReturnInst`.
    impl<'ctx> Display for ReturnInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `ReturnInst`.
    impl<'ctx> Clone for ReturnInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                return_inst: self.return_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `ReturnInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for ReturnInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_return_inst() {
                unsafe { Ok(ReturnInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
