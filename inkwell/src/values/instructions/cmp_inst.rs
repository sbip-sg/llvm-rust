//! Module handling to the `icmp` instruction of LLVM.

use super::{AnyCmp, AnyInstruction, AsInstructionValue};
use crate::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a comparison generic instruction.
///
/// The comparison instruction can be `icmp` or `fcmp`.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct CmpInst<'ctx> {
    cmp_inst: InstructionValue<'ctx>,
}

/// Implement methods for `CmpInst`.
impl<'ctx> CmpInst<'ctx> {
    /// Constructor of a `CmpInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_cmp_inst());
        CmpInst { cmp_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::CmpInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `CmpInst`.
    impl<'ctx> AsInstructionValue<'ctx> for CmpInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.cmp_inst
        }
    }

    /// Implement the `AsValueRef` trait for `CmpInst`.
    impl<'ctx> AsValueRef for CmpInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.cmp_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::CmpInst;
    use crate::values::instructions::{AnyCmp, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `CmpInst`.
    impl<'ctx> AnyInstruction<'ctx> for CmpInst<'ctx> {}

    /// Implement the `AnyCmp` trait for `CmpInst`.
    impl<'ctx> AnyCmp<'ctx> for CmpInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::CmpInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `CmpInst`.
    impl<'ctx> Display for CmpInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `CmpInst`.
    impl<'ctx> Clone for CmpInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                cmp_inst: self.cmp_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `CmpInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for CmpInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_cmp_inst() {
                unsafe { Ok(CmpInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
