//! Module handling to the `icmp` instruction of LLVM.

use super::{AnyCmp, AnyInstruction, AsInstructionValue};
use crate::values::traits::AsValueRef;
use crate::values::{AnyValue, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Data structure modelling a `icmp` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct ICmpInst<'ctx> {
    icmp_inst: InstructionValue<'ctx>,
}

/// Implement methods for `ICmpInst`.
impl<'ctx> ICmpInst<'ctx> {
    /// Constructor of a `ICmpInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_icmp_inst());
        ICmpInst { icmp_inst: inst }
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::ICmpInst;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `ICmpInst`.
    impl<'ctx> AsInstructionValue<'ctx> for ICmpInst<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.icmp_inst
        }
    }

    /// Implement the `AsValueRref` trait for `ICmpInst`.
    impl<'ctx> AsValueRef for ICmpInst<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.icmp_inst.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::ICmpInst;
    use crate::values::instructions::{AnyCmp, AnyInstruction};

    /// Implement the `AnyInstruction` trait for `ICmpInst`.
    impl<'ctx> AnyInstruction<'ctx> for ICmpInst<'ctx> {}

    /// Implement the `AnyCmp` trait for `ICmpInst`.
    impl<'ctx> AnyCmp<'ctx> for ICmpInst<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::ICmpInst;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `ICmpInst`.
    impl<'ctx> Display for ICmpInst<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `ICmpInst`.
    impl<'ctx> Clone for ICmpInst<'ctx> {
        fn clone(&self) -> Self {
            Self {
                icmp_inst: self.icmp_inst.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `ICmpInst`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for ICmpInst<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_icmp_inst() {
                unsafe { Ok(ICmpInst::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
