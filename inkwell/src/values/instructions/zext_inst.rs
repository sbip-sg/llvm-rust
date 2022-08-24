//! Module handling to the `zext` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `zext` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct ZExtInst<'ctx> {
    zext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `ZExtInst`.
impl<'ctx> ZExtInst<'ctx> {
    /// Constructor of a `ZExtInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_zext_inst());
        ZExtInst { zext_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::ZExtInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `ZExtInst`.
    impl<'ctx> AsInstructionValue<'ctx> for ZExtInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.zext_inst
        }
    }

    /// Implement the `AsValueRef` trait for `ZExtInst`.
    impl<'ctx> AsValueRef for ZExtInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.zext_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::ZExtInst;
    use crate::values::instructions::{traits::AnyCast, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `ZExtInst`.
    impl<'ctx> AnyInstruction<'ctx> for ZExtInst<'ctx> {}

    /// Implement the `AnyCast` trait for `ZExtInst`.
    impl<'ctx> AnyCast<'ctx> for ZExtInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::ZExtInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `ZExtInst`.
    impl<'ctx> Display for ZExtInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `ZExtInst`.
    impl<'ctx> Clone for ZExtInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                zext_inst: self.zext_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `ZExtInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for ZExtInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_zext_inst() {
                unsafe { Ok(ZExtInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
