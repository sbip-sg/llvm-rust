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

/// Data structure modelling a `load` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct LoadInst<'ctx> {
    load_inst: InstructionValue<'ctx>,
}

/// Implement methods for `LoadInst`.
impl<'ctx> LoadInst<'ctx> {
    /// Constructor of a `LoadInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_load_inst());
        LoadInst { load_inst: inst }
    }

    /// Get the pointer operand of the current `LoadInst`.
    pub fn get_pointer_operand(&self) -> PointerValue<'ctx> {
        if let Some(opr) = self.get_operand(0) {
            if let Left(v) = opr {
                if v.is_pointer_value() {
                    return v.into_pointer_value();
                }
            }
        }

        panic!("Invalid Load instruction: {}", self)
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::LoadInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `LoadInst`.
    impl<'ctx> AsInstructionValue<'ctx> for LoadInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.load_inst
        }
    }

    /// Implement the `AsValueRef` trait for `LoadInst`.
    impl<'ctx> AsValueRef for LoadInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.load_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::LoadInst;
    use crate::values::instructions::AnyInstruction;

    /// Implement the `AnyInstruction` trait for `LoadInst`.
    impl<'ctx> AnyInstruction<'ctx> for LoadInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::LoadInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `LoadInst`.
    impl<'ctx> Display for LoadInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `LoadInst`.
    impl<'ctx> Clone for LoadInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                load_inst: self.load_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `LoadInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for LoadInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_load_inst() {
                unsafe { Ok(LoadInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
