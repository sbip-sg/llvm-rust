//! Module handling to the `fcmp` instruction of LLVM.

use super::{AnyCmp, AnyInstruction, AsInstructionValue};
use crate::values::traits::AsValueRef;
use crate::values::{AnyValue, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `fcmp` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct FCmpInst<'ctx> {
    fcmp_inst: InstructionValue<'ctx>,
}

/// Implement methods for `FCmpInst`.
impl<'ctx> FCmpInst<'ctx> {
    /// Constructor of a `FCmpInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_fcmp_inst());
        FCmpInst { fcmp_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::FCmpInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `FCmpInst`.
    impl<'ctx> AsInstructionValue<'ctx> for FCmpInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.fcmp_inst
        }
    }

    /// Implement the `AsValueRef` trait for `FCmpInst`.
    impl<'ctx> AsValueRef for FCmpInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.fcmp_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::FCmpInst;
    use crate::values::instructions::{AnyCmp, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `FCmpInst`.
    impl<'ctx> AnyInstruction<'ctx> for FCmpInst<'ctx> {}

    /// Implement the `AnyCmp` trait for `FCmpInst`.
    impl<'ctx> AnyCmp<'ctx> for FCmpInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    use super::FCmpInst;

    /// Implement the `Display` trait for `FCmpInst`.
    impl<'ctx> Display for FCmpInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `FCmpInst`.
    impl<'ctx> Clone for FCmpInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                fcmp_inst: self.fcmp_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `FCmpInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for FCmpInst<'ctx> {
        type Error = ();

        fn try_from(
            value: InstructionValue<'ctx>,
        ) -> Result<Self, Self::Error> {
            if value.is_a_fcmp_inst() {
                unsafe { Ok(FCmpInst::new(value)) }
            } else {
                Err(())
            }
        }
    }
}
