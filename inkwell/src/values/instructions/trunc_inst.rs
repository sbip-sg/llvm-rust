//! Module handling to the `trunc` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `trunc` instruction.
#[derive(Debug, PartialEq, Eq, Copy)]
pub struct TruncInst<'ctx> {
    zext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `TruncInst`.
impl<'ctx> TruncInst<'ctx> {
    /// Constructor of a `TruncInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_zext_inst());
        TruncInst { zext_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::TruncInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `TruncInst`.
    impl<'ctx> AsInstructionValue<'ctx> for TruncInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.zext_inst
        }
    }

    /// Implement the `AsValueRef` trait for `TruncInst`.
    impl<'ctx> AsValueRef for TruncInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.zext_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::TruncInst;
    use crate::values::instructions::{traits::AnyCast, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `TruncInst`.
    impl<'ctx> AnyInstruction<'ctx> for TruncInst<'ctx> {}

    /// Implement the `AnyCast` trait for `TruncInst`.
    impl<'ctx> AnyCast<'ctx> for TruncInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::TruncInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `TruncInst`.
    impl<'ctx> Display for TruncInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `TruncInst`.
    impl<'ctx> Clone for TruncInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                zext_inst: self.zext_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `TruncInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for TruncInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_zext_inst() {
                unsafe { Ok(TruncInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
