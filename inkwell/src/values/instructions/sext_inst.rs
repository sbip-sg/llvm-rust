//! Module handling to the `sext` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `sext` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct SExtInst<'ctx> {
    sext_inst: InstructionValue<'ctx>,
}

/// Implement methods for `SExtInst`.
impl<'ctx> SExtInst<'ctx> {
    /// Constructor of a `SExtInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_sext_inst());
        SExtInst { sext_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::SExtInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `SExtInst`.
    impl<'ctx> AsInstructionValue<'ctx> for SExtInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.sext_inst
        }
    }

    /// Implement the `AsValueRef` trait for `SExtInst`.
    impl<'ctx> AsValueRef for SExtInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.sext_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::SExtInst;
    use crate::values::instructions::{traits::AnyCast, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `SExtInst`.
    impl<'ctx> AnyInstruction<'ctx> for SExtInst<'ctx> {}

    /// Implement the `AnyCast` trait for `SExtInst`.
    impl<'ctx> AnyCast<'ctx> for SExtInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::SExtInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `SExtInst`.
    impl<'ctx> Display for SExtInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `SExtInst`.
    impl<'ctx> Clone for SExtInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                sext_inst: self.sext_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `SExtInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for SExtInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_sext_inst() {
                unsafe { Ok(SExtInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
