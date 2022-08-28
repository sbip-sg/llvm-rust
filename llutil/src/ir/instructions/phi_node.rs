//! Module handling to the `phi` instruction of LLVM.

use inkwell::values::{AnyValue, AsValueRef, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;
use std::ffi::CStr;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

use super::{AnyInstruction, AsInstructionValue};

/// Data structure modelling a `phi` instruction.
///
/// REVIEW: merge this module with `PhiValue`?
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct PhiNode<'ctx> {
    /// Instruction value corresponding to the `PhiNode`.
    phi_node: InstructionValue<'ctx>,
}

/// Implement methods for `PhiNode`.
impl<'ctx> PhiNode<'ctx> {
    /// Constructor of a `PhiNode` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_phi_node());
        PhiNode { phi_node: inst }
    }

    /// Get name of the instruction.
    pub fn get_name(&self) -> Option<&CStr> {
        self.phi_node.get_name()
    }

    /// Set name of the instruction.
    pub fn set_name(&self, name: &str) -> Result<(), &'static str> {
        self.as_instruction_value().set_name(name)
    }

    /// Replace all uses of the `PhiNode`.
    pub fn replace_all_uses_with(self, other: &PhiNode<'ctx>) {
        self.phi_node
            .replace_all_uses_with(&other.as_instruction_value())
    }
}

/// Implement the `AsInstructionValue` trait for `PhiNode`.
impl<'ctx> AsInstructionValue<'ctx> for PhiNode<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.phi_node
    }
}

/// Implement the `AsValueRef` trait for `PhiNode`.
impl<'ctx> AsValueRef for PhiNode<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.phi_node.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `PhiNode`.
impl<'ctx> AnyInstruction<'ctx> for PhiNode<'ctx> {}

/// Implement the `AnyValue` trait for `PhiNode`.
impl<'ctx> AnyValue<'ctx> for PhiNode<'ctx> {}

/// Implement the `Display` trait for `PhiNode`.
impl<'ctx> Display for PhiNode<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `PhiNode`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for PhiNode<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_phi_node() {
            Ok(PhiNode::new(inst))
        } else {
            Err(())
        }
    }
}
