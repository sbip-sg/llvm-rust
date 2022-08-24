//! Module handling to the `call` instruction of LLVM.

use super::{AnyCall, AnyInstruction, AsInstructionValue};
use crate::values::traits::AsValueRef;
use crate::values::FunctionValue;
use crate::values::{AnyValue, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `call` instruction.
///
/// LLVM Language Reference Manual for the [`call`
/// instruction](https://llvm.org/docs/LangRef.html#call-instruction)
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct CallInst<'ctx> {
    call_inst: InstructionValue<'ctx>,
}

/// Implement methods for `CallInst`
impl<'ctx> CallInst<'ctx> {
    /// Constructor of a `CallInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_call_inst());
        CallInst { call_inst: inst }
    }

    /// get_called_fn_value
    #[llvm_versions(3.9..=latest)]
    pub fn get_called_fn_value(self) -> FunctionValue<'ctx> {
        use llvm_sys::core::LLVMGetCalledValue;

        unsafe {
            FunctionValue::new(LLVMGetCalledValue(self.as_value_ref()))
                .expect("This should never be null?")
        }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::CallInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `CallInst`.
    impl<'ctx> AsInstructionValue<'ctx> for CallInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.call_inst
        }
    }

    /// Implement the `AsValueRef` trait for `CallInst`.
    impl<'ctx> AsValueRef for CallInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.call_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::CallInst;
    use crate::values::instructions::{AnyCall, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `CallInst`.
    impl<'ctx> AnyInstruction<'ctx> for CallInst<'ctx> {}

    /// Implement the `AnyCall` trait for `CallInst`.
    impl<'ctx> AnyCall<'ctx> for CallInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::CallInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `CallInst`.
    impl<'ctx> Display for CallInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `CallInst`.
    impl<'ctx> Clone for CallInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                call_inst: self.call_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `CallInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for CallInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_call_inst() {
                unsafe { Ok(CallInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
