//! Module handling to the `icmp` instruction of LLVM.

use super::{AnyCmp, AnyInstruction, AsInstructionValue};
use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::fmt::{self, Display};

/// Data structure modelling a comparison generic instruction.
///
/// The comparison instruction can be `icmp` or `fcmp`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CmpInst<'ctx> {
    /// Instruction value corresponding to the `CmpInst`.
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

/// Implement the `AnyInstruction` trait for `CmpInst`.
impl<'ctx> AnyInstruction<'ctx> for CmpInst<'ctx> {}

/// Implement the `AnyCmp` trait for `CmpInst`.
impl<'ctx> AnyCmp<'ctx> for CmpInst<'ctx> {}

/// Implement the `AnyValue` trait for `CmpInst`.
impl<'ctx> AnyValue<'ctx> for CmpInst<'ctx> {}

/// Implement the `Display` trait for `CmpInst`.
impl<'ctx> Display for CmpInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `CmpInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for CmpInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_cmp_inst() {
            Ok(CmpInst::new(inst))
        } else {
            Err(())
        }
    }
}
