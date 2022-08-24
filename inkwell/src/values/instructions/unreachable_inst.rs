//! Module handling to the `unreachable` instruction of LLVM.

use std::convert::TryFrom;
use std::fmt::{self, Display};

use either::Either::Left;
use llvm_sys::prelude::LLVMValueRef;

use crate::values::{
    AnyValue, AsValueRef, FunctionValue, InstructionValue, PointerValue,
};

use super::{AnyCall, AnyInstruction, AsInstructionValue};

/// Data structure modelling a `unreachable` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct UnreachableInst<'ctx> {
    unreachable_inst: InstructionValue<'ctx>,
}

/// Implement methods for `UnreachableInst`.
impl<'ctx> UnreachableInst<'ctx> {
    /// Constructor of a `UnreachableInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_unreachable_inst());
        UnreachableInst {
            unreachable_inst: inst,
        }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::UnreachableInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `UnreachableInst`.
    impl<'ctx> AsInstructionValue<'ctx> for UnreachableInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.unreachable_inst
        }
    }

    /// Implement the `AsValueRef` trait for `UnreachableInst`.
    impl<'ctx> AsValueRef for UnreachableInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.unreachable_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::UnreachableInst;
    use crate::values::instructions::{
        AnyCall, AnyInstruction, AsInstructionValue,
    };

    /// Implement the `AnyInstruction` trait for `UnreachableInst`.
    impl<'ctx> AnyInstruction<'ctx> for UnreachableInst<'ctx> {}

    /// Implement the `AnyCall` trait for `UnreachableInst`.
    impl<'ctx> AnyCall<'ctx> for UnreachableInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::UnreachableInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `UnreachableInst`.
    impl<'ctx> Display for UnreachableInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `UnreachableInst`.
    impl<'ctx> Clone for UnreachableInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                unreachable_inst: self.unreachable_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `UnreachableInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for UnreachableInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_unreachable_inst() {
                unsafe { Ok(UnreachableInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
