//! Module handling to the `store` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionValue,
    PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `store` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct StoreInst<'ctx> {
    store_inst: InstructionValue<'ctx>,
}

/// Implement methods for `StoreInst`.
impl<'ctx> StoreInst<'ctx> {
    /// Constructor of a `StoreInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_store_inst());
        StoreInst { store_inst: inst }
    }

    /// Get the value operand of the current `StoreInst`.
    pub fn get_value_operand(&self) -> BasicValueEnum<'ctx> {
        if let Some(opr) = self.get_operand(0) {
            if let Left(v) = opr {
                return v;
            }
        }

        panic!("Invalid Store instruction: {}", self)
    }

    /// Get the pointer operand of the current `StoreInst`.
    pub fn get_pointer_operand(&self) -> PointerValue<'ctx> {
        if let Some(opr) = self.get_operand(1) {
            if let Left(v) = opr {
                if v.is_pointer_value() {
                    return v.into_pointer_value();
                }
            }
        }

        panic!("Invalid Store instruction: {}", self)
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::StoreInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `StoreInst`.
    impl<'ctx> AsInstructionValue<'ctx> for StoreInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.store_inst
        }
    }

    /// Implement the `AsValueRef` trait for `StoreInst`.
    impl<'ctx> AsValueRef for StoreInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.store_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::StoreInst;
    use crate::values::instructions::AnyInstruction;

    /// Implement the `AnyInstruction` trait for `StoreInst`.
    impl<'ctx> AnyInstruction<'ctx> for StoreInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::StoreInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `StoreInst`.
    impl<'ctx> Display for StoreInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `StoreInst`.
    impl<'ctx> Clone for StoreInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                store_inst: self.store_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `StoreInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for StoreInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_store_inst() {
                unsafe { Ok(StoreInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
