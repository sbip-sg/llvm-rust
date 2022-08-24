//! Module handling to the `callbr` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use crate::values::traits::AsValueRef;
use crate::values::{AnyValue, FunctionValue, InstructionValue, PointerValue};
use either::Either::Left;
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `callbr` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct CallBrInst<'ctx> {
    callbr_inst: InstructionValue<'ctx>,
}

impl<'ctx> CallBrInst<'ctx> {
    /// Constructor of a `CallBrInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_callbr_inst());
        CallBrInst { callbr_inst: inst }
    }
}

/// Module containing all implementations of conversion traits.
pub mod conversion_traits {
    use super::CallBrInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `CallBrInst`.
    impl<'ctx> AsInstructionValue<'ctx> for CallBrInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.callbr_inst
        }
    }

    /// Implement the `AsValueRef` trait for `CallBrInst`.
    impl<'ctx> AsValueRef for CallBrInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.callbr_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of behaviour traits.
pub mod behaviour_traits {
    use super::CallBrInst;
    use crate::values::instructions::{AnyCall, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `CallBrInst`.
    impl<'ctx> AnyInstruction<'ctx> for CallBrInst<'ctx> {}

    /// Implement the `AnyCall` trait for `CallBrInst`.
    impl<'ctx> AnyCall<'ctx> for CallBrInst<'ctx> {}
}

/// Module containing all implementations of standard traits.
pub mod standard_traits {
    use super::CallBrInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `CallBrInst`.
    impl<'ctx> Display for CallBrInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `CallBrInst`.
    impl<'ctx> Clone for CallBrInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                callbr_inst: self.callbr_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `CallBrInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for CallBrInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_callbr_inst() {
                unsafe { Ok(CallBrInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
