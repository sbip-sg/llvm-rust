//! Module handling to the `load` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling casting instructions such as `SExtInst`,
/// `ZExtInst`, etc.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct CastInst<'ctx> {
    cast_inst: InstructionValue<'ctx>,
}

/// Implement methods for `CastInst`.
impl<'ctx> CastInst<'ctx> {
    /// Constructor of a `CastInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_cast_inst());
        CastInst { cast_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::CastInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `CastInst`.
    impl<'ctx> AsInstructionValue<'ctx> for CastInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.cast_inst
        }
    }

    /// Implement the `AsValueRef` trait for `CastInst`.
    impl<'ctx> AsValueRef for CastInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.cast_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::CastInst;
    use crate::values::instructions::{traits::AnyCast, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `CastInst`.
    impl<'ctx> AnyInstruction<'ctx> for CastInst<'ctx> {}

    /// Implement the `AnyCast` trait for `CastInst`.
    impl<'ctx> AnyCast<'ctx> for CastInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::CastInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `CastInst`.
    impl<'ctx> Display for CastInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `CastInst`.
    impl<'ctx> Clone for CastInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                cast_inst: self.cast_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `CastInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for CastInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_cast_inst() {
                unsafe { Ok(CastInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
