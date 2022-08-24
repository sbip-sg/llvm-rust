//! Module handling to the `callbr` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use crate::values::traits::AsValueRef;
use crate::values::{AnyValue, FunctionValue, InstructionValue, PointerValue};
use either::Either::Left;
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `alloca` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct AllocaInst<'ctx> {
    alloca_inst: InstructionValue<'ctx>,
}

impl<'ctx> AllocaInst<'ctx> {
    /// Constructor of a `AllocaInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_alloca_inst());
        AllocaInst { alloca_inst: inst }
    }
}

/// Module containing all implementations of conversion traits.
pub mod conversion_traits {
    use super::AllocaInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `AllocaInst`.
    impl<'ctx> AsInstructionValue<'ctx> for AllocaInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.alloca_inst
        }
    }

    /// Implement the `AsValueRef` trait for `AllocaInst`.
    impl<'ctx> AsValueRef for AllocaInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.alloca_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of behaviour traits.
pub mod behaviour_traits {
    use super::AllocaInst;
    use crate::values::instructions::{AnyCall, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `AllocaInst`.
    impl<'ctx> AnyInstruction<'ctx> for AllocaInst<'ctx> {}
}

/// Module containing all implementations of standard traits.
pub mod standard_traits {
    use super::AllocaInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `AllocaInst`.
    impl<'ctx> Display for AllocaInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `AllocaInst`.
    impl<'ctx> Clone for AllocaInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                alloca_inst: self.alloca_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `AllocaInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for AllocaInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_alloca_inst() {
                unsafe { Ok(AllocaInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
